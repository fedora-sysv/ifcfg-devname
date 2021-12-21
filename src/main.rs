#[macro_use]
extern crate log;

use std::env;
use std::error;
use std::path::Path;
use std::str::FromStr;

use mac_address::{mac_address_by_name, MacAddress};

mod lib;
mod logger;
mod parse;
mod scan;

const ENV: &str = "INTERFACE";
const CONFIG_DIR: &str = "/etc/sysconfig/network-scripts";
const KERNEL_CMDLINE: &str = "/proc/cmdline";
const TEST_MODE_PARAMS_REQUIRED: usize = 3;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();
    let is_test_mode = is_test_mode(&args, TEST_MODE_PARAMS_REQUIRED);

    logger::init();

    let kernel_interface_name = match env::var_os(ENV).unwrap().into_string() {
        Ok(val) => val,
        Err(err) => {
            error!("Fail obtaining ENV {} - {}", ENV, err.to_string_lossy());
            std::process::exit(1)
        }
    };

    let mac_address = if !is_test_mode {
        match mac_address_by_name(&kernel_interface_name) {
            Ok(Some(val)) => val,
            _ => {
                error!("Fail to resolve MAC address of '{}'", kernel_interface_name);
                std::process::exit(1)
            }
        }
    } else {
        MacAddress::from_str(&args[3])?
    };

    let simple_mac_address = mac_address.to_string().to_lowercase();

    let kernel_cmdline = get_kernel_cmdline(is_test_mode, &args);

    /* Let's check kernel cmdline and also process ifname= entries
     * as they are documented in dracut.cmdline(7)
     * Example: ifname=test:aa:bb:cc:dd:ee:ff
     */
    let mut device_config_name = match parse::kernel_cmdline(&simple_mac_address, kernel_cmdline) {
        Ok(Some(name)) => {
            if lib::is_like_kernel_name(&name) {
                warn!("Don't use kernel names (eth0, etc.) as new names for network devices! Used name: '{}'", name);
            }
            name
        }
        _ => {
            debug!(
                "New device name for '{}' wasn't found at kernel cmdline",
                kernel_interface_name
            );
            String::from("")
        }
    };

    if device_config_name.is_empty() {
        let config_dir = if !is_test_mode { CONFIG_DIR } else { &args[2] };

        let config_dir_path = Path::new(config_dir);
        let ifcfg_paths = match scan::config_dir(config_dir_path) {
            Some(val) => val,
            None => {
                error!(
                    "Fail to get list of ifcfg files from directory {}",
                    config_dir
                );
                std::process::exit(1)
            }
        };

        device_config_name = String::new();
        for path in ifcfg_paths {
            let config_file_path: &Path = Path::new(&path);

            match parse::config_file(config_file_path, &simple_mac_address) {
                Ok(Some(name)) => {
                    if lib::is_like_kernel_name(&name) {
                        warn!("Don't use kernel names (eth0, etc.) as new names for network devices! Used name: '{}'", name);
                    }
                    device_config_name = format!("{}", name);
                    break;
                }
                _ => continue,
            }
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

fn is_test_mode(params: &Vec<String>, number_params_required: usize) -> bool {
    if params.len() > number_params_required {
        true
    } else {
        false
    }
}

fn get_kernel_cmdline(is_test_mode: bool, args: &Vec<String>) -> &Path {
    let kernel_cmdline = if is_test_mode {
        Path::new(&args[1])
    } else {
        Path::new(KERNEL_CMDLINE)
    };

    kernel_cmdline
}

#[cfg(test)]
mod unit_test;
