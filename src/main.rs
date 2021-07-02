use clap::App;
use std::env;

use mac_address::{
    mac_address_by_name,
    MacAddress
};

const ENV: &str = "INTERFACE";

fn main() {
    let kernel_if_name: String;
    let mac_address: MacAddress; 

    App::new("rename_device")
        .author("Macku Jan <jamacku@redhat.com>")
        .about("Does awesome things")
        .get_matches();

    /* Read env variable INTERFACE in order to get names of if */
    kernel_if_name = match env::var_os(ENV) {
        Some(val) => {
            println!("{}: {:?}", ENV, val);
            match val.into_string() {
                Ok(val) => val,
                _ => {
                    eprintln!("Error whille procesing env INTERFACE: {}.", ENV);
                    std::process::exit(1);
                }
            }
        },
        None => {
            eprintln!("{} is not defined in the environment.", ENV);
            std::process::exit(1);
        }
    };

    /* Get MAC addres of given interface */
    mac_address = match mac_address_by_name(&kernel_if_name) {
        Ok(val) => {
            match val {
                Some(val) => val,
                None => {
                    eprintln!("Error whille getting MAC address of current if: {}.", kernel_if_name);
                    std::process::exit(1);
                }
            }
        },
        _ => {
            eprintln!("Error whille getting MAC address of current if: {}.", kernel_if_name);
            std::process::exit(1);
        }
    };

    // ? SCAN config dir /etc/sysconfig/network-scripts
    // ? iterate over them and get DEVICE, SUBCHANNELS, HWADDR and VLAN

    // ? print out correct name of interface
}
