use std::{path::PathBuf, fs, env};
use rand::Rng;
use dirs::home_dir;
use std::process::Command;
use winreg::enums::*;
use winreg::RegKey;
use sha256::digest;

const HWID_PRIVATE_FILE: &str = "hwid-private-do-not-share";
const HWID_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const HWID_PRIVATE_LENGTH: usize = 512;
const HWID_LENGTH: usize = 64;


pub fn get_hwid_private_file() -> PathBuf {
    let home_directory = home_dir().unwrap();
    return home_directory.join(".lunarclient").join("launcher-cache").join(HWID_PRIVATE_FILE);
}
pub fn get_hwid_private(random: bool) -> String {
    if random {
        let mut rng = rand::thread_rng();
        let random_hwid = (0..HWID_PRIVATE_LENGTH)
        .map(|_| {
            let idx = rng.gen_range(0..HWID_CHARSET.len());
            HWID_CHARSET[idx] as char
        })
        .collect();
         
        return random_hwid;
    }
    let contents = fs::read_to_string(get_hwid_private_file()).expect("HWID Private!");
    return contents;
}

pub fn get_machine_id(random: bool) -> String {
    if random {
        let mut rng = rand::thread_rng();
        let random_hwid = (0..HWID_LENGTH)
        .map(|_| {
            let idx = rng.gen_range(0..HWID_CHARSET.len());
            HWID_CHARSET[idx] as char
        })
        .collect();

        return random_hwid;
    }
    let os = env::consts::OS.to_string();
    let machine_id: String;
    if os == "windows" {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let crytography_key = hklm.open_subkey("SOFTWARE\\Microsoft\\Cryptography").unwrap();
        let machine_guid: String = crytography_key.get_value("MachineGuid").unwrap();
      
        machine_id = machine_guid.to_string();
    } else if os == "linux" {
        machine_id =  String::from_utf8_lossy(&Command::new(get_machine_id_command())
                     .output()
                     .expect("Failed to Machine Id").stdout
        ).to_string();
    } else {
        machine_id = "Dumb Shit".to_string();
    };
    
    return digest(machine_id).to_string();
}
pub fn get_window_directory() -> String {
    let window_directory = match env::var_os("windir") {
        Some(v) => v.into_string().unwrap().to_string(),
        None => "C:\\WINDOWS".to_string()
    };
    window_directory
}

pub fn get_machine_id_command() -> String {
    let os = env::consts::OS.to_string();
    if os == "windows" {
        let reg_folder = match env::var_os("PROCESSOR_ARCHITEW6432") {
            Some(_) => "%windir%\\sysnative\\cmd.exe /c %windir%\\System32".to_string(),
            None => "%windir%\\System32".to_string()
        }.replace("%windir%", &get_window_directory());
        return reg_folder + "\\REG.exe";
    } else if os == "linux" {
        return "( cat /var/lib/dbus/machine-id /etc/machine-id 2> /dev/null || hostname ) | head -n 1 || :".to_string();
    }

    //Fuck every other os
    return "".to_string();
}