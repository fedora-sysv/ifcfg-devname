use std::env;
use std::fs::{self, DirEntry};
use std::path::Path;

use clap::App;

use mac_address::{
    mac_address_by_name,
    MacAddress
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

    if config_dir.is_dir() {
        for entry in fs::read_dir(config_dir) {
            println!("entry: {:?}", entry);
            /*let entry = entry;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }*/
        }
    }

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
