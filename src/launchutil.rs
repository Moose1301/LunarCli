use std::{env, path::PathBuf, fs::{File, self}, io::{self, BufReader, Read}};
use crate::{apiutils::{LaunchResponse, get_launcher_version}, get_lunarclient_folder, UserInput, get_minecraft_folder, hwidutil::get_machine_id};
use reqwest::blocking::Client;
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};
use sha1::{Digest, Sha1};
use zip::ZipArchive;
use uuid::Uuid;

fn get_class_path(launch_response: &LaunchResponse) -> String {
    let artifacts = &launch_response.launchTypeData.artifacts;
    let mut to_return = "".to_string();
    let os = env::consts::OS;
    let joined = if os == "windows" { ";" } else { ":" };
    for ele in artifacts {
        if ele.r#type == "CLASS_PATH" {
            if !to_return.is_empty() {
                to_return.push_str(joined);
            }
            to_return.push_str(&ele.name);
        }

    }
    return to_return.to_string();
}
fn get_external_files(launch_response: &LaunchResponse) -> String {
    let artifacts = &launch_response.launchTypeData.artifacts;
    let mut to_return = "".to_string();
    let os = env::consts::OS;
    let joined = if os == "windows" { ";" } else { ":" };
    for ele in artifacts {
        if ele.r#type == "EXTERNAL_FILE" {
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

pub fn build_java_args(ram: u32, launch_response: &LaunchResponse) -> String {
    let main_class = &launch_response.launchTypeData.mainClass;
    let mut java_arguments = get_ram_args(ram);
    java_arguments.push_str(" -Djava.library.path=natives --add-modules jdk.naming.dns --add-exports jdk.naming.dns/com.sun.jndi.dns=java.naming -Dlog4j2.formatMsgNoLookups=true --add-opens java.base/java.io=ALL-UNNAMED ");
    java_arguments.push_str(("-cp ".to_owned() + &get_class_path(&launch_response)).as_str());
    java_arguments.push_str((" ".to_owned() + &main_class).as_str());

    return java_arguments.to_string();
}
pub fn build_program_args(user_input: &UserInput, launch_response: &LaunchResponse) -> String {
    let mut program_arguments = "--version ".to_string();
    program_arguments.push_str(user_input.version.get_display_name());
    program_arguments.push_str(" --accessToken 0");

    program_arguments.push_str(" --assetIndex ");
    program_arguments.push_str(user_input.version.get_asset_index());

    program_arguments.push_str(" --userProperties {}");

    program_arguments.push_str(" --gameDir ");
    program_arguments.push_str(&get_minecraft_folder().to_string_lossy().to_string());

    program_arguments.push_str(" --texturesDir ");
    program_arguments.push_str(&get_lunarclient_folder().join("textures").to_string_lossy().to_string());

    program_arguments.push_str(" --launcherVersion ");
    program_arguments.push_str(&get_launcher_version());

    program_arguments.push_str(" --hwid ");
    program_arguments.push_str(&get_machine_id(user_input.hide_hwid));

    program_arguments.push_str(" --installationId ");
    program_arguments.push_str(Uuid::new_v4().to_string().as_str());

    program_arguments.push_str(" --width 854 --height 480");

    program_arguments.push_str(" --workingDirectory ");
    program_arguments.push_str(&user_input.working_directory);
    
    program_arguments.push_str(" --classpathDir ");
    program_arguments.push_str(&user_input.cache_folder);

    program_arguments.push_str(" --ichorClassPath ");
    program_arguments.push_str(&get_class_path(&launch_response).as_str().replace(";", ",").replace(":", ","));

    program_arguments.push_str(" --ichorExternalFiles ");
    program_arguments.push_str(&get_external_files(&launch_response).as_str().replace(";", ",").replace(":", ","));


    return program_arguments;
}
pub fn download_files(cache_folder: String, launch_response: &LaunchResponse, multi_progress: &MultiProgress) {
    let client: Client = Client::new();
    let folder: PathBuf = PathBuf::from(cache_folder);
    let size = &launch_response.launchTypeData.artifacts.len();

    let pb = multi_progress.add(ProgressBar::new(*size as u64));
    pb.set_style(ProgressStyle::default_bar()
    .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {human_pos}/{human_len} {percent} ({eta})")
    .unwrap()
    .progress_chars("█ "));

    let mut position = 0;
    for ele in &launch_response.launchTypeData.artifacts {
        //println!("Downloading File: {:?} into {:?}", &ele.name, folder.join(&ele.name));
        pb.set_position(position);
        position += 1;
        let path = folder.join(&ele.name);
        if std::path::Path::new(&path).exists() {
            let file = fs::File::open(&path).expect("Could not get file");
            let mut buf_reader = BufReader::new(file);
            let mut contents = vec![];
            buf_reader.read_to_end(&mut contents).unwrap();
        
            let mut hasher = Sha1::new();
            hasher.update(&contents);
            let result = hasher.finalize();
            let sha1_string = format!("{:x}", result);
            let artifact_sha = ele.sha1.clone();
            if sha1_string == artifact_sha {
                pb.set_message(format!("Skipping {} as the hash is the same", &ele.name));
                continue;
            }
        }
        pb.set_message(format!("Downloading {}", &ele.name));

        let mut resp = client.get(&ele.url).send().expect("request failed");
        let mut out: File;
        if std::path::Path::new(&path).exists() {
            fs::remove_file(&path).expect("Failed to delete file");
            out = File::create(&path.as_path()).expect(format!("Failed to create file '{:?}'", &ele.name).as_str());

        } else {
            out = File::create(&path.as_path()).expect(format!("Failed to create file '{:?}'", &ele.name).as_str());
        }
        io::copy(&mut resp, &mut out).expect("failed to copy content");
        //println!("Downloaded File: {:?}", &ele.name);
        if ele.r#type == "NATIVES" {
            pb.set_message(format!("Unziping {} to natives", &ele.name));
            let archive_file = fs::File::open(&path).unwrap();

            let _archive = zip::ZipArchive::new(&archive_file).expect("Archive validated before-hand");

            extract_archive(_archive, folder.join("natives"));
            pb.set_message(format!("Unzipped {} to natives", &ele.name));
        }
      
    }
    pb.finish_with_message(format!("Downloaded {:?} files", size));
}

pub fn check_jre(launch_response: &LaunchResponse) -> bool {
    let jre_path = get_lunarclient_folder().join("jre");
    return jre_path.join(&launch_response.jre.folderChecksum).exists();
}
#[allow(unused_mut)]
pub fn download_jre(launch_response: &LaunchResponse, multi_progress: &MultiProgress) {
    let pb = multi_progress.add(ProgressBar::new(4));
    pb.set_style(ProgressStyle::default_bar()
    .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {human_pos}/{human_len} {percent} ({eta})")
    .unwrap()
    .progress_chars("█ "));
    pb.set_message("Downloading JRE");
    pb.inc(1);
    let folder_checksum = launch_response.jre.folderChecksum.clone();
    let client: Client = Client::new();
    let jre_path = get_lunarclient_folder().join("jre");
    let path = jre_path.join(&folder_checksum.clone());
    let temp_file_name = jre_path.join(folder_checksum.clone().to_string() + &".tmp.zip");
    fs::create_dir(path.clone()).expect("Failed to create Temp File");

    let mut temp_file = File::create(jre_path.join(folder_checksum.clone().to_string() + &".tmp.zip".to_string())
        .as_path())
        .expect(format!("Failed to create file '{:?}'", folder_checksum.clone() + &".tmp.zip".to_string()).as_str());
    let jre_url = launch_response.jre.download.get("url").unwrap().to_string().replace("\"", "");
    let mut resp = client.get(jre_url).send()
    .expect("request failed");
    io::copy(&mut resp, &mut temp_file).expect("failed to copy content");
    pb.set_message("Extracting JRE");
    pb.inc(1);
    let archive_file = fs::File::open(temp_file_name).unwrap();
    let archive = zip::ZipArchive::new(&archive_file).expect("Archive validated before-hand");
    extract_archive(archive, path);
    pb.set_message("Extracted JRE");
    pb.inc(1);
    fs::remove_file(jre_path.join(folder_checksum.clone().to_string() + &".tmp.zip".to_string())).expect("Failed to delete Temp File");
}

fn extract_archive(mut archive: ZipArchive<&File>, jre_path: PathBuf) {
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => jre_path.join(path.to_owned()),
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
               
            }
        }

        if (*file.name()).ends_with('/') {
            fs::create_dir_all(&outpath).unwrap();
        } else {
            
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
}