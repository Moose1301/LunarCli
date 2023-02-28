pub mod apiutils;
pub mod hwidutil;
pub mod version;

use std::path::PathBuf;

use clap::Parser;
use version::LunarVersion;

use dirs::home_dir;

use crate::{
    apiutils::{build_download_json, get_launcher_version, send_launch_request, LaunchRequest},
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
    //The Directory you want to store everything
    //Must be full path
    #[arg(long, default_value_t = get_default_cache_parent().to_string_lossy().to_string())]
    cache_folder: String,
    #[arg(long, default_value_t = { false })]
    //Don't Update
    dont_update: bool,
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
    if !args.dont_update {
        send_launch_request(launch_request);
    }
}
fn get_default_cache_parent() -> PathBuf {
    let home_directory = home_dir().unwrap();
    return home_directory
        .join(".lunarclient")
        .join("offline")
        .join("mutliver");
}