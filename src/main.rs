use std::env;
use std::path::Path;

use clap::App;

use mac_address::{
    mac_address_by_name,
    MacAddress
};

use glob::{
    MatchOptions,
    glob_with
};


// --- --- --- //

const ENV: &str = "INTERFACE";
const CONFIG_DIR: &str = "/etc/sysconfig/network-scripts";


// --- --- --- //

fn main() {
    let kernel_if_name: String;
    let mac_address: MacAddress;
    let config_dir: &Path = Path::new(CONFIG_DIR);

    App::new("rename_device")
        .author("Macku Jan <jamacku@redhat.com>")
        .about("Does awesome things")
        .get_matches();

    /* Read env variable INTERFACE in order to get names of if */
    kernel_if_name = match read_env_interface(ENV) {
        Some(val) => val,
        None => std::process::exit(1)
    };

    /* Get MAC addres of given interface */
    mac_address = match get_mac_address(&kernel_if_name) {
        Some(val) => val,
        None => {
            eprintln!("Error whille getting MAC address of current if: {}.", kernel_if_name);
            std::process::exit(1);
        }
    };

    println!("MAC address of {} is: {}", kernel_if_name, mac_address);

    // ? SCAN config dir /etc/sysconfig/network-scripts
    // ? iterate over them and get DEVICE, SUBCHANNELS, HWADDR and VLAN

    // file ifcfg-** ?? or directory !!
    // contain HWADDR = MAC
    // return NAME = ??

    scan_config_dir(config_dir);

    // ? print out correct name of interface
}


// --- Functions --- //

/* Read env variable INTERFACE in order to get names of if */
fn read_env_interface(env_name: &str) -> Option<String> {
    match env::var_os(env_name) {
        Some(val) => {
            match val.into_string() {
                Ok(val) => Some(val),
                _ => {
                    eprintln!("Error whille procesing env INTERFACE: {}.", env_name);
                    None
                }
            }
        },
        None => {
            eprintln!("{} is not defined in the environment.", env_name);
            None
        }
    }
}

/* Get MAC addres of given interface */
fn get_mac_address(if_name: &str) -> Option<MacAddress> {
    match mac_address_by_name(if_name) {
        Ok(val) => val,
        _ => {
            None
        }
    }
}

/* Scan directory /etc/sysconfig/network-scripts for ifcfg files */
fn scan_config_dir(config_dir: &Path) -> Option<String> {
    let glob_options: MatchOptions;
    let glob_patern: String;
    let mut device_config_name: String = String::new();

    glob_options = glob::MatchOptions {
        case_sensitive: true,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    glob_patern = config_dir.to_str().unwrap().to_owned() + "/ifcfg-*";

    for entry in glob_with(&glob_patern, glob_options).unwrap() {
        match entry {
            Ok(path) => {
                let config_file_path: &Path = Path::new(path.as_path());
                match scan_config_file(config_file_path) {
                    Some(name) => {
                        device_config_name = format!("{}", name);
                        break;
                    }
                    _ => continue
                }
            },
            _ => continue
        };
    }

    if !device_config_name.is_empty() {
        Some(device_config_name)
    } else {
        None
    }
}

/* Scan ifcfg files and look for given HWADDR and return DEVICE name */
fn scan_config_file(config_file: &Path) -> Option<String> {
    println!("config: {}", config_file.display());
    Some("s".to_owned())
}
