use std::{env};

use reqwest::{self, blocking::Client};
use serde::{Serialize, Deserialize};
use serde_json::Map;

use crate::hwidutil::{get_machine_id, get_hwid_private};

const API_URL: &str = "https://api.lunarclientprod.com";
const LAUNCHER_VERSION_URL: &str = "https://launcherupdates.lunarclientcdn.com/latest.yml";


pub fn get_launcher_version() -> String {
    let resp = reqwest::blocking::get(LAUNCHER_VERSION_URL).unwrap();
    let text = resp.text().unwrap();
    let version = text.split("\n").next().unwrap_or("2.15.0");
    return version.to_string();
}

#[derive(Serialize, Debug)]
pub struct LaunchRequest {
    os: String,
    hwid: String,
    hwid_private: String,
    arch: String,
    launcher_version: String,
    version: String,
    branch: String,
    launch_type: String,
    module: String,
}
#[derive(Deserialize, Debug)]
#[allow(non_snake_case, dead_code)]
pub struct LaunchResponse {
    pub jre: VersionJRE,
    pub launchTypeData: LaunchTypeData
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case, dead_code)]
pub struct VersionJRE {
    pub download: Map<String, serde_json::Value>,
    pub executablePathInArchive: Vec<String>,
    pub extraArguments: Vec<String>,
    pub folderChecksum: String,
    pub javawDownload: String,
    pub javawExeChecksum: String,

}
#[derive(Deserialize, Debug)]
#[allow(non_snake_case, dead_code)]
pub struct LaunchTypeData {
    pub artifacts: Vec<ClientArtifact>,
    pub ichor: bool,
    pub mainClass: String
}
#[derive(Deserialize, Debug)]
#[allow(non_snake_case, dead_code)]
pub struct ClientArtifact {
    pub differentialUrl: String,
    pub name: String,
    pub sha1: String,
    pub r#type: String,
    pub url: String,
}

pub fn send_launch_request(launch_request: LaunchRequest) -> LaunchResponse {
    let client: Client = Client::new();
    
    let response = client.post(API_URL.to_owned() + &"/launcher/launch".to_string())
        .header("User-Agent", "Lunar Client Launcher v".to_string() + &get_launcher_version())
        .body(serde_json::to_string(&launch_request).unwrap().to_string()).send();
    let response_text = response.unwrap().text().unwrap().to_string();
    //println!("Response: {}", response_text);
    let launch_response: LaunchResponse = serde_json::from_str(response_text.as_str()).unwrap();
   
    return launch_response;
} 

pub fn build_download_json(
    hide_hwid: bool,
    version: String,
    branch: String,
    module: String,
) -> LaunchRequest {
    LaunchRequest {
        os: get_corrected_os().to_string(),
        hwid: get_machine_id(hide_hwid),
        hwid_private: get_hwid_private(hide_hwid),
        arch: "x".to_owned() + &env::consts::ARCH.to_string().replace("x86_", ""),
        launcher_version: get_launcher_version(),
        version: version,
        branch: branch,
        launch_type: "OFFLINE".to_string(),
        module: module,
    }
}
fn get_corrected_os() -> &'static str {
    let os = env::consts::OS;
    if os == "windows" {
        return "win32";
    } else if os == "linux" {
        return "linux";
    }
    return os

}
