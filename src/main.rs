pub mod version;
use std::{path::PathBuf, env};

use clap::{Parser, builder::Str};
use version::{LunarVersion};
use dirs::{home_dir};

#[derive(Parser, Debug)]
struct Args {
    //Lunar Client Version
    #[clap(value_enum)]
    #[arg(short = 'v', long="version")]
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
    cache_folder: String
}


fn main() {
    let args = Args::parse();
    println!("Got Args: {:?}", args);
}

fn get_default_cache_parent() -> PathBuf {
    let home_directory = home_dir().unwrap();
    return home_directory.join(".lunarclient").join("offline").join("mutliver");
}