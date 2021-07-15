use std::env;
use std::path::Path;
use std::fs::File;
use std::io:: {
    prelude::*,
    BufReader
};

use clap::App;

use mac_address:: {
    mac_address_by_name,
    MacAddress
};

use glob:: {
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

    let mut device_config_name: String;

    App::new("rename_device")
        .author("Macku Jan <jamacku@redhat.com>")
        .about("Program rename_device reads ENV INTERFACE, which is expected to contain the name of the network interface. Then it looks for the hardware address of such an interface. Finally it scans ifcfg configuration files in directory /etc/sysconfig/network-scripts/ and looks for configuration with HWADDR set to given hw address. If the program successfully finds such a configuration, it returns on standard output content of property DEVICE from matching ifcfg configuration. In all other cases it returns error code 1.")
        .get_matches();

    /* Read env variable INTERFACE in order to get names of if */
    kernel_if_name = match read_env_interface(ENV) {
        Some(val) => val,
        None => {
            /* Error while processing ENV INTERFACE */
            std::process::exit(1);
        }
    };

    /* Get MAC address of given interface */
    mac_address = match get_mac_address(&kernel_if_name) {
        Some(val) => val,
        None => {
            /* Error while getting MAC address of given network interface */
            std::process::exit(1);
        }
    }; 
    
    // TODO: scan kernel cmd for ifname=new_name:aa:aa:aa:aa:aa:aa

    /* Scan config dir and look for ifcfg-* files */
    config_dir = Path::new(CONFIG_DIR);
    list_of_ifcfg_paths = match scan_config_dir(config_dir) {
        Some(val) => val,
        None => {
            /* Error while getting list of ifcfg files from directory /etc/sysconfig/network-scripts/ */
            std::process::exit(1);
        }
    };

    /* Loop through ifcfg configurations and look for matching MAC address and return DEVICE name */
    device_config_name = String::new();
    'config_loop: for path in list_of_ifcfg_paths {
        let config_file_path: &Path = Path::new(&path);

        match scan_config_file(config_file_path, &mac_address) {
            Some(name) => {
                device_config_name = format!("{}", name);
                break 'config_loop;
            }
            None => continue
        }
    }

    if !device_config_name.is_empty() {
        println!("{}", device_config_name);
    } else {
        /* Device name or MAC address weren't found in ifcfg files. */
        std::process::exit(1);
    }
}


// --- Functions --- //

/* Read env variable INTERFACE in order to get name of network interface */
fn read_env_interface(env_name: &str) -> Option<String> {
    /* Converts the OsString into a [String] if it contains valid Unicode data. On failure, ownership of the original OsString is returned. */
    match env::var_os(env_name)?.into_string() {
        Ok(val) => Some(val),
        Err(_err) => None
    }
}

/* Get MAC address of given interface */
fn get_mac_address(if_name: &str) -> Option<MacAddress> {
    match mac_address_by_name(if_name) {
        Ok(val) => val,
        Err(_err) => None
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

    glob_patern = config_dir.to_str()?.to_owned() + "/ifcfg-*";

    list_of_config_paths = vec![];

    for entry in glob_with(&glob_patern, glob_options).unwrap() {
        match entry {
            Ok(path) => {
                list_of_config_paths.push(path.to_str()?.to_owned());
            },
            Err(_err) => continue
        };
    }

    if !list_of_config_paths.is_empty() {
        Some(list_of_config_paths)
    } else {
        None
    }
}

/* Scan ifcfg files and look for given HWADDR and return DEVICE name */
fn scan_config_file(config_file: &Path, mac_address: &MacAddress) -> Option<String> {
    let file: File;
    let reader: BufReader<File>;

    /* Needs to be Option in order to prevent Error: "borrow of possibly-uninitialized variable" */
    let mut hwaddr: Option<MacAddress>;
    let mut device: Option<String>;

    file = File::open(config_file).unwrap();
    reader = BufReader::new(file);
    hwaddr = None;
    device = None;

    lazy_static! {
        /* Look for line that starts with DEVICE= and then store everything else in group */
        static ref REGEX_DEVICE: Regex = Regex::new(r"^DEVICE=(\S*)").unwrap();

        /* Look for line with mac address and store its value in group for later */
        static ref REGEX_HWADDR: Regex = Regex::new(r"^HWADDR=(\S*)").unwrap();
    }

    /* Read lines of given file and look for DEVICE= and HWADDR= */
    for line in reader.lines() {
        let line = line.unwrap();

        /* Look for HWADDR= */
        if REGEX_HWADDR.is_match(&line) {
            for capture in REGEX_HWADDR.captures_iter(&line) {
                hwaddr = Some(capture[1].parse().unwrap());
            }
        }

        /* Look for DEVICE= */
        if REGEX_DEVICE.is_match(&line) {
            for capture in REGEX_DEVICE.captures_iter(&line) {
                device = Some(capture[1].parse().unwrap());
            }
        }
    }

    if hwaddr?
        .to_string()
        .to_owned()
        .to_lowercase()
        .eq(
            &mac_address
                .to_string()
                .to_owned()
                .to_lowercase()
    ) {
        device
    } else {
        None
    }
}
