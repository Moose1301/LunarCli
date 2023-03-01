pub mod apiutils;
pub mod hwidutil;
pub mod version;
pub mod launchutil;

use std::{path::PathBuf, env};

use clap::Parser;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

use version::LunarVersion;

use dirs::home_dir;

use crate::{
    apiutils::{build_download_json, get_launcher_version, send_launch_request, LaunchRequest, LaunchResponse}, 
    launchutil::{download_files, check_jre, download_jre, build_program_args, build_java_args}
};

#[derive(Parser, Debug)]
pub struct UserInput {
    //Lunar Client Version
    #[clap(value_enum)]
    #[arg(short = 'v', long = "version")]
    pub version: LunarVersion,
    //The Client Module Usually
    #[arg(short = 'm', long = "module", default_value = "lunar")]
    pub module: String,
    //The Branch of Lunar you want to Use
    //Not Recommand to use hide hwid with this
    #[arg(short = 'b', long = "branch", default_value = "master")]
    pub branch: String,
    #[arg(long, default_value_t = { false })]
    //If we should Hide your HWID that is sent to lunar's services
    pub hide_hwid: bool,
    
    #[arg(long, default_value_t = get_default_cache_parent().to_string_lossy().to_string())]
    pub working_directory: String,
  
    #[arg(long, default_value_t = get_default_cache_parent().to_string_lossy().to_string())]
    pub cache_folder: String,
    #[arg(long, default_value_t = { false })]
    //If we shouldn't Auto Update
    pub dont_update: bool,
    #[arg(long, default_value_t = 3072)]
    //Memory to Allocate
    pub ram: u32,

}


fn main() {
    let multi_progress = MultiProgress::new();
    let sty = ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {human_pos}/{human_len} {percent} ({eta})")
        .unwrap()
        .progress_chars("█ ");
    let args = UserInput::parse();

    println!("Got Args: {:?}", args);
    println!("Launcher Version {:?}", get_launcher_version());
    let pb = multi_progress.add(ProgressBar::new(5));
    pb.set_style(sty.clone());
    pb.set_message("Generating Launch Request");
    pb.inc(1);
    let launch_request: LaunchRequest = build_download_json(
        args.hide_hwid.clone(),
        args.version.get_display_name().to_string(),
        args.branch.clone(),
        args.module.clone(),
    );
    pb.inc(1);
    pb.set_message("Sending Launch Request");
    let launch_response: LaunchResponse = send_launch_request(launch_request);

    pb.set_message("Sent Launch Request");

    if !args.dont_update {
        pb.set_message("Downloading Files");
        pb.inc(1);
        download_files(args.cache_folder.clone(), &launch_response, &multi_progress)
    }
    pb.set_message("Verifying JRE");
    pb.inc(1);
    if !check_jre(&launch_response) {
        pb.set_message("Downloading JRE");

        download_jre(&launch_response, &multi_progress);
        pb.inc(1);
    }
    println!("Java Args: {:?}", build_java_args(args.ram, &launch_response));
    println!("Program Args: {:?}", build_program_args(&args, &launch_response));
}
fn get_lunarclient_folder() -> PathBuf {
    let home_directory = home_dir().unwrap();
    return home_directory
        .join(".lunarclient");
}
fn get_default_cache_parent() -> PathBuf {
    return get_lunarclient_folder()
        .join("offline")
        .join("multiver");
}
fn get_minecraft_folder() -> PathBuf {
    let os = env::consts::OS;
    let home_directory = home_dir().unwrap();
    if os == "windows" {
        let appdata_directory = match env::var_os("APPDATA") {
            Some(v) => PathBuf::from(v.into_string().unwrap().to_string()),
            None => home_directory,
        };
        return appdata_directory.join(".minecraft");
    }
    return home_directory.join(".minecraft");
}