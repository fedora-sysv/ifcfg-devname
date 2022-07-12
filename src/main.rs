use std::env;
use std::error;
use std::path::Path;

use log::*;

mod lib;
mod logger;
mod parser;
mod scanner;

enum Args {
    ConfigDir = 1,
    Mac,
    Length,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    const ENV: &str = "INTERFACE";
    const CONFIG_DIR: &str = "/etc/sysconfig/network-scripts";

    let args: Vec<String> = env::args().collect();
    let is_test_mode = lib::is_test_mode(&args, Args::Length as usize);

    logger::init();

    let kernel_interface_name = match env::var_os(ENV).unwrap().into_string() {
        Ok(val) => val,
        Err(err) => {
            error!("Fail obtaining ENV {} - {}", ENV, err.to_string_lossy());
            std::process::exit(1)
        }
    };

    let mac_address = match lib::get_mac_address(
        is_test_mode,
        &args,
        Args::Mac as usize,
        &kernel_interface_name,
    ) {
        Ok(val) => val,
        _ => {
            error!("Fail to resolve MAC address of '{}'", kernel_interface_name);
            std::process::exit(1)
        }
    };

    let simple_mac_address = mac_address.to_string().to_lowercase();

    let config_dir = if !is_test_mode {
        CONFIG_DIR
    } else {
        &args[Args::ConfigDir as usize]
    };

    let config_dir_path = Path::new(config_dir);
    let ifcfg_paths = match scanner::config_dir(config_dir_path) {
        Some(val) => val,
        None => {
            error!(
                "Fail to get list of ifcfg files from directory {}",
                config_dir
            );
            std::process::exit(1)
        }
    };

    let mut device_config_name = String::new();

    for path in ifcfg_paths {
        let config_file_path: &Path = Path::new(&path);

        match parser::config_file(config_file_path, &simple_mac_address) {
            Ok(Some(name)) => {
                if lib::is_like_kernel_name(&name) {
                    warn!("Don't use kernel names (eth0, etc.) as new names for network devices! Used name: '{}'", name);
                }
                device_config_name = name;
                break;
            }
            _ => continue,
        }
    }

    if !device_config_name.is_empty() {
        println!("{}", device_config_name);
        Ok(())
    } else {
        error!("Device name or MAC address weren't found in ifcfg files.");
        std::process::exit(1);
    }
}
