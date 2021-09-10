extern crate syslog;
#[macro_use]
extern crate log;

use std::env;
use std::error;
use std::path::Path;
use std::fs::File;
use std::io:: {
    prelude::*,
    BufReader
};

use mac_address:: {
    mac_address_by_name,
    MacAddress
};
use std::str::FromStr;

use glob::glob_with;

use lazy_static::lazy_static;
use regex::Regex;

use syslog:: {
    Facility,
    Formatter3164,
    BasicLogger
};

use log::LevelFilter;


// --- --- --- //

/* Implement conversion from any type that implements the Error trait into the trait object Box<Error>
 * https://doc.rust-lang.org/std/keyword.dyn.html */
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

const ENV: &str = "INTERFACE";
const CONFIG_DIR: &str = "/etc/sysconfig/network-scripts";
const KERNEL_CMDLINE: &str = "/proc/cmdline";


// --- --- --- //

fn main() -> Result<()> {
    /* Store any commandline arguments */
    let args: Vec<String> = env::args().collect();


    /* Setup syslog logger */ 
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: "ifcfg_devname".into(),
        pid: 0,
    };

    let logger = syslog::unix(formatter).expect("[ifcfg_devname]: could not connect to syslog");
    /* This is a simple convenience wrapper over set_logger */
    log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
        .map(|()| log::set_max_level(LevelFilter::Info))?;

    debug!("Connected to syslog");


    /* Read env variable INTERFACE in order to get names of if */
    let kernel_if_name = match env::var_os(ENV).unwrap().into_string() {
        Ok(val) => val,
        Err(err) => {
            error!("Error while processing ENV INTERFACE - {}", err.to_string_lossy());
            std::process::exit(1)
        }
    };

    
    /* Check for testing hw address passed via arg */
    let mac_address = if args[3].is_empty() {
        /* Get MAC address of given interface */
        match mac_address_by_name(&kernel_if_name) {
            Ok(Some(val)) => val,
            _ => {
                error!("Error while getting MAC address of given network interface ({})", kernel_if_name);
                std::process::exit(1)
            }
        }
    } else {
        MacAddress::from_str(&args[3])?
    };

    
    /* Check for alternative path to kernel cmdline */
    let kernel_cmdline = if args[1].is_empty() {
        Path::new(KERNEL_CMDLINE)
    } else {
        Path::new(&args[1])
    };


    /* Let's check kernel cmdline and also process ifname= entries
     * as they are documented in dracut.cmdline(7)
     * Example: ifname=test:aa:bb:cc:dd:ee:ff
     */
    let mut device_config_name = match parse_kernel_cmdline(&mac_address, kernel_cmdline) {
        Ok(Some(name)) => {
            if check_new_devname(name.clone()).is_some() {
                warn!("Warning!! Please, do NOT use kernel like devnames (eth0, etc.) as new names for your network interface devices! Used name: '{}'", name);
            }
            name
        },
        _ => {
            debug!("New device name for '{}' wasn't found at kernel cmdline", kernel_if_name);
            String::from("")
        }
    };


    /* When device was not found at kernel cmdline look into ifcfg files */
    if device_config_name.is_empty() {
        /* Check for alternative path to config dir */
        let config_dir = if args[2].is_empty() {
            CONFIG_DIR
        } else {
            &args[2]
        };

        /* Scan config dir and look for ifcfg-* files */
        let config_dir_path = Path::new(config_dir);
        let list_of_ifcfg_paths = match scan_config_dir(config_dir_path) {
            Some(val) => val,
            None => {
                error!("Error while getting list of ifcfg files from directory {}", config_dir);
                std::process::exit(1)
            }
        };

        /* Loop through ifcfg configurations and look for matching MAC address and return DEVICE name */
        device_config_name = String::new();
        'config_loop: for path in list_of_ifcfg_paths {
            let config_file_path: &Path = Path::new(&path);

            match parse_config_file(config_file_path, &mac_address) {
                Ok(Some(name)) => {
                    if check_new_devname(name.clone()).is_some() {
                        warn!("Warning!! Please, do NOT use kernel like devnames (eth0, etc.) as new names for your network interface devices! Used name: '{}'", name);
                    }
                    device_config_name = format!("{}", name);
                    break 'config_loop;
                }
                _ => continue
            }
        }
    }


    if !device_config_name.is_empty() {
        println!("{}", device_config_name);
        Ok(())
    } else {
        warn!("Device name or MAC address weren't found in ifcfg files.");
        std::process::exit(1);
    }
}


// --- Functions --- //
/* Scan directory /etc/sysconfig/network-scripts for ifcfg files */
fn scan_config_dir(config_dir: &Path) -> Option<Vec<String>> {
    let glob_options = glob::MatchOptions {
        case_sensitive: true,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    let glob_pattern = config_dir.to_str()?.to_owned() + "/ifcfg-*";

    let mut list_of_config_paths = vec![];

    for entry in glob_with(&glob_pattern, glob_options).unwrap() {
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

/* Scan kernel cmdline and look for given hardware address and return new device name */
#[allow(unused)]
fn parse_kernel_cmdline(mac_address: &MacAddress, kernel_cmdline_path: &Path) -> Result<Option<String>> {
    let file = File::open(kernel_cmdline_path).unwrap();
    let mut reader = BufReader::new(file);
    let mut hwaddr: Option<MacAddress> = None;
    let mut device: Option<String> = None;
    let mut kernel_cmdline = String::new();

    lazy_static! {
        /* Look for patterns like this ifname=new_name:aa:BB:CC:DD:ee:ff at kernel command line
         * regex: ifname=(\S[^:]{1,15}):(([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2}))
         * ifname=(group1):(group2) - look for pattern starting with `ifname=` following with two groups separated with `:` character
         * group1: (\S[^:]{1,15}) - match non-whitespace characters ; minimum 1 and maximum 15 ; do not match `:` character
         * group2: (([0-9A-Fa-f]{2}[:]){5}([0-9A-Fa-f]{2})) - match 48-bit hw address expressed in hexadecimal system ; each of inner 8-bits are separated with `:` character ; case insensitive
         * example: ifname=new-devname007:00:1b:44:11:3A:B7
         *                 ^^^^^^^^^^^^^^ ~~~~~~~~~~~~~~~~~
         *                 new dev name   hw address of if */
        static ref REGEX_DEVICE_HWADDR_PAIR: Regex = Regex::new(r"ifname=(\S[^:]{1,15}):(([0-9A-Fa-f]{2}[:]){5}([0-9A-Fa-f]{2}))").unwrap();
    }

    /* Read kernel command line and look for ifname= */
    reader.read_line(&mut kernel_cmdline)?;

    /* Look for ifname= */
    if REGEX_DEVICE_HWADDR_PAIR.is_match(&kernel_cmdline) {
        for capture in REGEX_DEVICE_HWADDR_PAIR.captures_iter(&kernel_cmdline) {
            device = Some(capture[1].parse()?);
            hwaddr = Some(capture[2].parse()?);
                
            /* Check MAC */
            if hwaddr.is_some() {
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
            } else {
                device = None;
            }
        }
    }

    /* When MAC doesn't match it returns OK(None) */
    match device {
        dev => Ok(dev)
    }
}

/* Scan ifcfg files and look for given HWADDR and return DEVICE name */
fn parse_config_file(config_file: &Path, mac_address: &MacAddress) -> Result<Option<String>> {
    let file = File::open(config_file)?;
    let reader = BufReader::new(file);
    let mut hwaddr: Option<MacAddress> = None;
    let mut device: Option<String> = None;

    lazy_static! {
        /* Look for line that starts with DEVICE= and then store everything else in group
         * regex: ^DEVICE=(\S[^:]{1,15})
         * ^DEVICE=(group1) - look for line starting with `DEVICE=` following with group of characters describing new device name
         * group1: (\S[^:]{1,15}) - match non-whitespace characters ; minimum 1 and maximum 15 ; do not match `:` character
         * example: DEVICE=new-devname007
         *                 ^^^^^^^^^^^^^^
         *                 new dev name */
        static ref REGEX_DEVICE: Regex = Regex::new(r"^DEVICE=(\S[^:]{1,15})").unwrap();

        /* Look for line with mac address and store its value in group for later
         * regex: ^HWADDR=(([0-9A-Fa-f]{2}[:]){5}([0-9A-Fa-f]{2}))
         * ^HWADDR=(group1) - look for line starting with `HWADDR=` following with group of characters describing hw address of device
         * group1: (([0-9A-Fa-f]{2}[:]){5}([0-9A-Fa-f]{2})) - match 48-bit hw address expressed in hexadecimal system ; each of inner 8-bits are separated with `:` character ; case insensitive
         * example: HWADDR=00:1b:44:11:3A:B7
         *                 ^^^^^^^^^^^^^^^^^
         *                 hw address of if */
        static ref REGEX_HWADDR: Regex = Regex::new(r"^HWADDR=(([0-9A-Fa-f]{2}[:]){5}([0-9A-Fa-f]{2}))").unwrap();
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

    if hwaddr.is_some() {
        if hwaddr
            .unwrap()
            .to_string()
            .to_owned()
            .to_lowercase()
            .ne(
                &mac_address
                    .to_string()
                    .to_owned()
                    .to_lowercase()
        ) {
            device = None;
        }
    } else {
        device = None;
    }

    /* When MAC doesn't match it returns OK(None) */
    match device {
        dev => Ok(dev)
    }
}

/* Check if new devname is equal to kernel standard devname (eth0, etc.)
 * If such a name is detected return Some(()) else None */
fn check_new_devname(new_devname: String) -> Option<()> {
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
        Some(())
    } else {
        None
    }
}


// --- Unit tests --- //
#[cfg(test)]
mod should {
    use super::*;
    use std::str::FromStr;

    const TEST_CONFIG_DIR: &str = "./tests/unit_test_data/ifcfgs";
    const TEST_KERNEL_CMDLINE_DIR: &str = "./tests/unit_test_data/cmdlines";
    
    
    // --- Kernel cmdline parser - Unit tests --- //
    #[test]
    fn parse_cmdline() {
        let mac_address = MacAddress::from_str("AA:BB:CC:DD:EE:1F").unwrap();
        let kernel_cmdline_path = Path::new(TEST_KERNEL_CMDLINE_DIR).join("1_should_pass");

        let device_config_name = match parse_kernel_cmdline(&mac_address, &kernel_cmdline_path) {
            Ok(Some(name)) => name,
            _ => {
                String::from("")
            }
        };

        assert_eq!("unit_test_1", device_config_name);
    }

    #[test]
    #[should_panic]
    fn not_parse_cmdline() {
        let mac_address = MacAddress::from_str("AA:BB:CC:DD:EE:2F").unwrap();
        let kernel_cmdline_path = Path::new(TEST_KERNEL_CMDLINE_DIR).join("2_should_fail");

        let device_config_name = match parse_kernel_cmdline(&mac_address, &kernel_cmdline_path) {
            Ok(Some(name)) => name,
            _ => {
                String::from("")
            }
        };

        assert_eq!("unit_test_2", device_config_name);
    }


    // --- Scaning and parsing of ifcfg configuration files - Unit tests --- //
    #[test]
    fn scan_ifcfg_dir() {
        let ifcfg_dir_path = Path::new(TEST_CONFIG_DIR);

        let test_result = match scan_config_dir(ifcfg_dir_path) {
            Some(result) => result.eq(
                    &vec!("tests/unit_test_data/ifcfgs/ifcfg-eth0", "tests/unit_test_data/ifcfgs/ifcfg-eth1")
                ),
            _ => false
        };

        assert!(test_result);
    }

    #[test]
    fn parse_ifcfg_configuration() {
        let mac_address = MacAddress::from_str("AA:BB:CC:DD:EE:3F").unwrap();
        let ifcfg_config_path = Path::new(TEST_CONFIG_DIR).join("ifcfg-eth0");

        let test_result = match parse_config_file(&ifcfg_config_path, &mac_address) {
            Ok(Some(result)) => result.eq("correct_if_name"),
            _ => false
        };

        assert!(test_result);
    }

    #[test]
    #[should_panic]
    fn not_parse_ifcfg_configuration() {
        let mac_address = MacAddress::from_str("AA:BB:CC:DD:EE:4F").unwrap();
        let ifcfg_config_path = Path::new(TEST_CONFIG_DIR).join("ifcfg-eth1");

        let test_result = match parse_config_file(&ifcfg_config_path, &mac_address) {
            Ok(Some(result)) => result.eq("im_not_here"),
            _ => false
        };

        assert!(test_result);
    }
}
