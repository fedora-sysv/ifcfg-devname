use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{
    prelude::*,
    BufReader
};

use clap::App;

use mac_address::{
    mac_address_by_name,
    MacAddress
};

use glob::{
    MatchOptions,
    glob_with
};

use lazy_static::lazy_static;
use regex::Regex;


// --- --- --- //

const ENV: &str = "INTERFACE";
const CONFIG_DIR: &str = "/etc/sysconfig/network-scripts";


// --- --- --- //

fn main() {
    let kernel_if_name: String;
    let mac_address: MacAddress;
    let config_dir: &Path;
    let list_of_ifcfg_paths: Vec<String>;

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

    /* Scan config dir and look for ifcfg-* files */
    config_dir = Path::new(CONFIG_DIR);
    list_of_ifcfg_paths = match scan_config_dir(config_dir) {
        Some(val) => val,
        None => {
            eprintln!("Error whille getting list of ifcfg files in directory: {}.", config_dir.display());
            std::process::exit(1);
        }
    };

    println!("list of configs: {:?}", list_of_ifcfg_paths);

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
fn scan_config_dir(config_dir: &Path) -> Option<Vec<String>> {
    let glob_options: MatchOptions;
    let glob_patern: String;
    let mut list_of_config_paths: Vec<String>;

    glob_options = glob::MatchOptions {
        case_sensitive: true,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    glob_patern = config_dir.to_str().unwrap().to_owned() + "/ifcfg-*";

    list_of_config_paths = vec![];

    for entry in glob_with(&glob_patern, glob_options).unwrap() {
        match entry {
            Ok(path) => {
                list_of_config_paths.push(path.to_str().unwrap().to_owned());
                // let config_file_path: &Path = Path::new(path.as_path());
                // match scan_config_file(config_file_path) {
                //     Some(name) => {
                //         device_config_name = format!("{}", name);
                //         break;
                //     }
                //     _ => continue
                // }
            },
            _ => continue
        };
    }

    if !list_of_config_paths.is_empty() {
        Some(list_of_config_paths)
    } else {
        None
    }
}

/* Scan ifcfg files and look for given HWADDR and return DEVICE name */
// ? get DEVICE, SUBCHANNELS, HWADDR and VLAN
fn scan_config_file(config_file: &Path) -> Option<String> {
    // TODO: Proper error handling using ``?``
    let file = File::open(config_file).unwrap();
    let reader = BufReader::new(file);

    lazy_static! {
        /* look for line that starts with DEVICE= and then store everything else in group */
        static ref REGEX_DEVICE: Regex = Regex::new(r"^DEVICE=(.*)").unwrap();

        /* look for line with mac address and store its value in group for later */
        static ref REGEX_HWADDR: Regex = Regex::new(r"^HWADDR=(.*)").unwrap();
    }

    /* Read lines of given file and look for DEVICE= and HWADDR= */
    for line in reader.lines(){
        let line = line.unwrap();

        if REGEX_HWADDR.is_match(&line){
            for capture in REGEX_HWADDR.captures_iter(&line) {
                println!("mac: {}", &capture[1]);
            }
        }

        if REGEX_DEVICE.is_match(&line){
            for capture in REGEX_DEVICE.captures_iter(&line) {
                println!("name: {}", &capture[1]);
            }
        }

    }

    Some("s".to_owned())
}
