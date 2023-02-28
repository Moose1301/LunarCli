use std::{env, path::PathBuf, fs::File, io::{self, Cursor}};
use crate::apiutils::LaunchResponse;
use reqwest::blocking::Client;


fn get_class_path(launch_response: &LaunchResponse) -> String {
    let test = &launch_response.launchTypeData.artifacts;
    let mut to_return = "".to_string();
    let os = env::consts::OS;
    let joined = if os == "windows" { ";" } else { ":" };
    for ele in test {
        if ele.r#type == "CLASS_PATH" {
            if !to_return.is_empty() {
                to_return.push_str(joined);
            }
            to_return.push_str(&ele.name);
        }
    }
    return to_return.to_string();
}
fn get_ram_args(ram: u32) -> String {
    return "-Xms${allocatedMemoryMb}m -Xmx${allocatedMemoryMb}m"
        .to_string()
        .replace("${allocatedMemoryMb}", &ram.to_string());
}

fn build_java_args(ram: u32, launch_response: LaunchResponse) -> String {
    let main_class = &launch_response.launchTypeData.mainClass;
    let mut java_arguments = get_ram_args(ram);
    java_arguments.push_str(" -Djava.library.path=natives ");
    java_arguments.push_str(("-cp ".to_owned() + &get_class_path(&launch_response)).as_str());
    java_arguments.push_str((" ".to_owned() + &main_class).as_str());

    return java_arguments.to_string();
}

pub fn download_files(cache_folder: String, launch_response: LaunchResponse) {
    let client: Client = Client::new();
    let folder: PathBuf = PathBuf::from(cache_folder);
    println!("Downloading???!!");
    for ele in launch_response.launchTypeData.artifacts {
        println!("Downloading File: {:?} into {:?}", &ele.name, folder.join(&ele.name));
        let mut resp = client.get(ele.url).send().expect("request failed");
        let mut out: File;
        let path = folder.join(&ele.name);
        if std::path::Path::new(&path).exists() {
            out = std::fs::OpenOptions::new()
                .read(true)
                .append(true)
                .open(path)
                .unwrap();
        } else {
            out = File::create(path.as_path()).expect(format!("Failed to create file '{:?}'", &ele.name).as_str());
        }
        io::copy(&mut resp, &mut out).expect("failed to copy content");
        println!("Downloaded File: {:?}", &ele.name);
        if ele.r#type == "NATIVES" {
            println!("Unziping: {:?} to natives", &ele.name);
            zip_extract::extract(Cursor::new(out), &folder.join("natives"), true);
            println!("Unzipped: {:?} to natives", &ele.name);
        }
    }
}
