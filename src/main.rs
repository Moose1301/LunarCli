pub mod apiutils;
pub mod hwidutil;
pub mod version;
pub mod launchutil;

use std::path::PathBuf;

use clap::Parser;
use version::LunarVersion;

use dirs::home_dir;

use crate::{
    apiutils::{build_download_json, get_launcher_version, send_launch_request, LaunchRequest, LaunchResponse}, launchutil::download_files
};

#[derive(Parser, Debug)]
struct Args {
    //Lunar Client Version
    #[clap(value_enum)]
    #[arg(short = 'v', long = "version")]
    version: LunarVersion,
    //The Client Module Usually
    #[arg(short = 'm', long = "module", default_value = "lunar")]
    module: String,
    //The Branch of Lunar you want to Use
    //Not Recommand to use hide hwid with this
    #[arg(short = 'b', long = "branch", default_value = "master")]
    branch: String,
    #[arg(long, default_value_t = { false })]
    //If we should Hide your HWID that is sent to lunar's services
    hide_hwid: bool,
    
    #[arg(long, default_value_t = get_default_cache_parent().to_string_lossy().to_string())]
    working_directory: String,
  
    #[arg(long, default_value_t = get_default_cache_parent().to_string_lossy().to_string())]
    cache_folder: String,
    #[arg(long, default_value_t = { false })]
    //If we shouldn't Auto Update
    dont_update: bool,
    #[arg(long, default_value_t = 3072)]
    //Memory to Allocate
    ram: u32,

}

fn main() {
    let args = Args::parse();

    println!("Got Args: {:?}", args);
    println!("Launcher Version {:?}", get_launcher_version());

    let launch_request: LaunchRequest = build_download_json(
        args.hide_hwid,
        args.version.get_display_name().to_string(),
        args.branch,
        args.module,
    );
    println!("Launch Request: {:?}", launch_request);
    let launch_response: LaunchResponse = send_launch_request(launch_request);
    if !args.dont_update {
        download_files(args.cache_folder, launch_response)
    }
}
fn get_default_cache_parent() -> PathBuf {
    let home_directory = home_dir().unwrap();
    return home_directory
        .join(".lunarclient")
        .join("offline")
        .join("multiver");
}