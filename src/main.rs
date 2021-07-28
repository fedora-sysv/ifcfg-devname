use std::env;
use std::error;
use std::path::Path;
use std::fs::File;
use std::io::ErrorKind;
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

/* Implement conversion from any type that implements the Error trait into the trait object Box<Error> */
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

const ENV: &str = "INTERFACE";
const CONFIG_DIR: &str = "/etc/sysconfig/network-scripts";
// const KERNEL_CMD: &str = "/proc/cmdline";
const KERNEL_CMD: &str = "./fake_kernel_cmd";


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
        /* Error while processing ENV INTERFACE */
        None => std::process::exit(1)
    };

    /* Get MAC address of given interface */
    mac_address = match get_mac_address(&kernel_if_name) {
        Ok(Some(val)) => val,
        /* Error while getting MAC address of given network interface */
        _ => std::process::exit(1)
    }; 
    
    // TODO: scan kernel cmd for ifname=new_name:aa:aa:aa:aa:aa:aa
    /* Let's check kernel cmdline and also process ifname= entries
     * as they are documented in dracut.cmdline(7)
     * Example: ifname=test:aa:bb:cc:dd:ee:ff
     */
    scan_kernel_cmd(&mac_address);

    /* Scan config dir and look for ifcfg-* files */
    config_dir = Path::new(CONFIG_DIR);
    list_of_ifcfg_paths = match scan_config_dir(config_dir) {
        Some(val) => val,
        /* Error while getting list of ifcfg files from directory /etc/sysconfig/network-scripts/ */
        None => std::process::exit(1)
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
fn get_mac_address(if_name: &str) -> Result<Option<MacAddress>> {
    Ok(mac_address_by_name(if_name)?)
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
fn scan_config_file(config_file: &Path, mac_address: &MacAddress) -> Result<Option<String>> {
    let file: File;
    let reader: BufReader<File>;

    /* Needs to be Option in order to prevent Error: "borrow of possibly-uninitialized variable" */
    let mut hwaddr: Option<MacAddress>;
    let mut device: Option<String>;

    file = File::open(config_file)?;
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
        let line = line?;

        /* Look for HWADDR= */
        if REGEX_HWADDR.is_match(&line) {
            for capture in REGEX_HWADDR.captures_iter(&line) {
                hwaddr = Some(capture[1].parse()?);
            }
        }

        /* Look for DEVICE= */
        if REGEX_DEVICE.is_match(&line) {
            for capture in REGEX_DEVICE.captures_iter(&line) {
                device = Some(capture[1].parse()?);
            }
        }
    }

    /* When MAC doesn't match it returns OK(None) */
    if hwaddr
        .unwrap()
        .to_string()
        .to_owned()
        .to_lowercase()
        .eq(
            &mac_address
                .to_string()
                .to_owned()
                .to_lowercase()
    ) {
        Ok(device)
    } else {
        Ok(None)
    }
}

/* Scan kernel cmd and look for given hardware address and return new device name */
fn scan_kernel_cmd(mac_address: &MacAddress) -> Result<Option<String>> {
    let file: File;
    let reader: BufReader<File>;

    /* Needs to be Option in order to prevent Error: "borrow of possibly-uninitialized variable" */
    let mut hwaddr: Option<MacAddress>;
    let mut device: Option<String>;

    file = File::open(KERNEL_CMD).unwrap();
    reader = BufReader::new(file);
    hwaddr = None;
    device = None;

    lazy_static! {
        /* Look for paterns like this ifname=new_name:aa:BB:CC:DD:ee:ff at kernel command line */
        static ref REGEX_DEVICE_HWADDR_PAIR: Regex = Regex::new(r"ifname=(\S+?):(\S*)").unwrap();
    }

    /* Read lines of kernel command line and look for ifname= */
    for line in reader.lines() {
        let line = line?;

        println!("kernel cmd: {}", line);

        /* Look for ifname= */
        if REGEX_DEVICE_HWADDR_PAIR.is_match(&line) {
            for capture in REGEX_DEVICE_HWADDR_PAIR.captures_iter(&line) {
                device = Some(capture[1].parse()?);
                hwaddr = Some(capture[2].parse()?);
                
                /* Check MAC */
                if hwaddr
                    .unwrap()
                    .to_string()
                    .to_owned()
                    .to_lowercase()
                    .eq(
                        &mac_address
                            .to_string()
                            .to_owned()
                            .to_lowercase()
                ) {
                    break;
                } else {
                     device = None;
                }
            }
        }
    }

    /* When MAC doesn't match it returns OK(None) */
    match device {
        dev => Ok(dev)
    }
}
