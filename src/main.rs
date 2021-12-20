extern crate syslog;
#[macro_use]
extern crate log;

use std::env;
use std::error;
use std::path::Path;

use mac_address::{mac_address_by_name, MacAddress};
use std::str::FromStr;

use glob::glob_with;

use lazy_static::lazy_static;
use regex::Regex;

mod logger;
mod parse;

const ENV: &str = "INTERFACE";
const CONFIG_DIR: &str = "/etc/sysconfig/network-scripts";
const KERNEL_CMDLINE: &str = "/proc/cmdline";

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();
    let is_correct_number_args = args.len() > 3;

    logger::init();

    let kernel_interface_name = get_interface_name();

    /* Check for testing hw address passed via arg */
    let mac_address = if !is_correct_number_args {
        /* Get MAC address of given interface */
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

    /* Check for alternative path to kernel cmdline */
    let kernel_cmdline = if !is_correct_number_args {
        Path::new(KERNEL_CMDLINE)
    } else {
        Path::new(&args[1])
    };

    /* Let's check kernel cmdline and also process ifname= entries
     * as they are documented in dracut.cmdline(7)
     * Example: ifname=test:aa:bb:cc:dd:ee:ff
     */
    let mut device_config_name = match parse::kernel_cmdline(&simple_mac_address, kernel_cmdline) {
        Ok(Some(name)) => {
            if is_like_kernel_name(&name) {
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

    /* When device was not found at kernel cmdline look into ifcfg files */
    if device_config_name.is_empty() {
        /* Check for alternative path to config dir */
        let config_dir = if !is_correct_number_args {
            CONFIG_DIR
        } else {
            &args[2]
        };

        /* Scan config dir and look for ifcfg-* files */
        let config_dir_path = Path::new(config_dir);
        let ifcfg_paths = match scan_config_dir(config_dir_path) {
            Some(val) => val,
            None => {
                error!(
                    "Fail to get list of ifcfg files from directory {}",
                    config_dir
                );
                std::process::exit(1)
            }
        };

        /* Loop through ifcfg configurations and look for matching MAC address and return DEVICE name */
        device_config_name = String::new();
        for path in ifcfg_paths {
            let config_file_path: &Path = Path::new(&path);

            match parse::config_file(config_file_path, &simple_mac_address) {
                Ok(Some(name)) => {
                    if is_like_kernel_name(&name) {
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

fn get_interface_name() -> String {
    let name = match env::var_os(ENV).unwrap().into_string() {
        Ok(val) => val,
        Err(err) => {
            error!("Fail obtaining ENV {} - {}", ENV, err.to_string_lossy());
            std::process::exit(1)
        }
    };

    name
}

/* Scan directory /etc/sysconfig/network-scripts for ifcfg files */
fn scan_config_dir(config_dir: &Path) -> Option<Vec<String>> {
    let glob_options = glob::MatchOptions {
        case_sensitive: true,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    let glob_pattern = config_dir.to_str()?.to_owned() + "/ifcfg-*";

    let mut config_paths = vec![];

    for entry in glob_with(&glob_pattern, glob_options).unwrap() {
        match entry {
            Ok(path) => {
                config_paths.push(path.to_str()?.to_owned());
            }
            Err(_err) => continue,
        };
    }

    if !config_paths.is_empty() {
        Some(config_paths)
    } else {
        None
    }
}

/* Check if new devname is equal to kernel standard devname (eth0, etc.)
 * If such a name is detected return true else false */
fn is_like_kernel_name(new_devname: &str) -> bool {
    lazy_static! {
        /* Check if new devname is equal to kernel standard devname (eth0, etc.)
         * regex: ^eth\d+$
         * ^eth - look for name starting with `eth`
         * \d+$ - following with set of numbers [0-9]
         * example: eth1234 | eth1234a
         *          ^^^^^^^^  ~~~~~~~~
         *           MATCH    NO MATCH */
        static ref IS_NEW_DEVNAME_ETH_LIKE: Regex = Regex::new(r"^eth\d+$").unwrap();
    }

    /* Look for HWADDR= */
    if IS_NEW_DEVNAME_ETH_LIKE.is_match(&new_devname) {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod should {
    use super::*;
    use std::str::FromStr;

    const TEST_CONFIG_DIR: &str = "./tests/unit_test_data/ifcfgs";
    const TEST_KERNEL_CMDLINE_DIR: &str = "./tests/unit_test_data/cmdlines";
    // --- Kernel cmdline parser - Unit tests --- //
    #[test]
    fn parse_cmdline() {
        let mac_address = MacAddress::from_str("AA:BB:CC:DD:EE:1F")
            .unwrap()
            .to_string()
            .to_lowercase();
        let kernel_cmdline_path = Path::new(TEST_KERNEL_CMDLINE_DIR).join("1_should_pass");

        let device_config_name = match parse::kernel_cmdline(&mac_address, &kernel_cmdline_path) {
            Ok(Some(name)) => name,
            _ => String::from(""),
        };

        assert_eq!("unit_test_1", device_config_name);
    }

    #[test]
    #[should_panic]
    fn not_parse_cmdline() {
        let mac_address = MacAddress::from_str("AA:BB:CC:DD:EE:2F")
            .unwrap()
            .to_string()
            .to_lowercase();
        let kernel_cmdline_path = Path::new(TEST_KERNEL_CMDLINE_DIR).join("2_should_fail");

        let device_config_name = match parse::kernel_cmdline(&mac_address, &kernel_cmdline_path) {
            Ok(Some(name)) => name,
            _ => String::from(""),
        };

        assert_eq!("unit_test_2", device_config_name);
    }

    // --- Scaning and parsing of ifcfg configuration files - Unit tests --- //
    #[test]
    fn scan_ifcfg_dir() {
        let ifcfg_dir_path = Path::new(TEST_CONFIG_DIR);

        let test_result = match scan_config_dir(ifcfg_dir_path) {
            Some(result) => result.eq(&vec![
                "tests/unit_test_data/ifcfgs/ifcfg-eth0",
                "tests/unit_test_data/ifcfgs/ifcfg-eth1",
            ]),
            _ => false,
        };

        assert!(test_result);
    }

    #[test]
    fn parse_ifcfg_configuration() {
        let mac_address = MacAddress::from_str("AA:BB:CC:DD:EE:3F")
            .unwrap()
            .to_string()
            .to_lowercase();
        let ifcfg_config_path = Path::new(TEST_CONFIG_DIR).join("ifcfg-eth0");

        let test_result = match parse::config_file(&ifcfg_config_path, &mac_address) {
            Ok(Some(result)) => result.eq("correct_if_name"),
            _ => false,
        };

        assert!(test_result);
    }

    #[test]
    #[should_panic]
    fn not_parse_ifcfg_configuration() {
        let mac_address = MacAddress::from_str("AA:BB:CC:DD:EE:4F")
            .unwrap()
            .to_string()
            .to_lowercase();
        let ifcfg_config_path = Path::new(TEST_CONFIG_DIR).join("ifcfg-eth1");

        let test_result = match parse::config_file(&ifcfg_config_path, &mac_address) {
            Ok(Some(_)) => true,
            _ => false,
        };

        assert!(test_result);
    }
}
