use std::error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

use mac_address::MacAddress;

use lazy_static::lazy_static;
use regex::Regex;

/* Implement conversion from any type that implements the Error trait into the trait object Box<Error>
 * https://doc.rust-lang.org/std/keyword.dyn.html */
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

/* Scan kernel cmdline and look for given hardware address and return new device name */
#[allow(unused)]
pub fn kernel_cmdline(mac_address: &str, kernel_cmdline_path: &Path) -> Result<Option<String>> {
    let file = File::open(kernel_cmdline_path)?;
    let mut reader = BufReader::new(file);
    let mut hwaddr: Option<MacAddress> = None;
    let mut device: Option<String> = None;
    let mut kernel_cmdline = String::new();

    lazy_static! {
        /* Look for patterns like this ifname=new_name:aa:BB:CC:DD:ee:ff at kernel command line
         * regex: ifname=(\S[^:]{0,14}):(([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2}))
         * ifname=(group1):(group2) - look for pattern starting with `ifname=` following with two groups separated with `:` character
         * group1: (\S[^:]{0,14}) - match non-whitespace characters ; minimum 1 and maximum 15 ; do not match `:` character
         * group2: (([0-9A-Fa-f]{2}[:]){5}([0-9A-Fa-f]{2})) - match 48-bit hw address expressed in hexadecimal system ; each of inner 8-bits are separated with `:` character ; case insensitive
         * example: ifname=new-devname007:00:1b:44:11:3A:B7
         *                 ^^^^^^^^^^^^^^ ~~~~~~~~~~~~~~~~~
         *                 new dev name   hw address of if */
        static ref REGEX_DEVICE_HWADDR_PAIR: Regex = Regex::new(r"ifname=(\S[^:]{0,14}):(([0-9A-Fa-f]{2}[:]){5}([0-9A-Fa-f]{2}))").unwrap();
    }

    /* Read kernel command line and look for ifname= */
    reader.read_line(&mut kernel_cmdline)?;

    /* Look for ifname= */
    if REGEX_DEVICE_HWADDR_PAIR.is_match(&kernel_cmdline) {
        for capture in REGEX_DEVICE_HWADDR_PAIR.captures_iter(&kernel_cmdline) {
            device = Some(capture[1].parse()?);
            hwaddr = Some(capture[2].parse()?);
            /* Check MAC */
            if hwaddr
                .unwrap()
                .to_string()
                .to_owned()
                .to_lowercase()
                .eq(mac_address)
            {
                break;
            } else {
                device = None;
            }
        }
    }

    /* When MAC doesn't match it returns OK(None) */
    match device {
        Some(_) => Ok(device),
        None => Err("new device name not found".into()),
    }
}

/* Scan ifcfg files and look for given HWADDR and return DEVICE name */
pub fn config_file(config_file: &Path, mac_address: &str) -> Result<Option<String>> {
    let file = File::open(config_file)?;
    let reader = BufReader::new(file);
    let mut hwaddr: Option<MacAddress> = None;
    let mut device: Option<String> = None;

    lazy_static! {
        /* Look for line that starts with DEVICE= and then store everything else in group
         * regex: ^\s*DEVICE=(\S[^:]{0,14})
         * ^\s*DEVICE=(group1) - look for line starting with `DEVICE=` (ignore whitespaces) following with group of characters describing new device name
         * group1: (\S[^:]{0,14}) - match non-whitespace characters ; minimum 1 and maximum 15 ; do not match `:` character
         * example: DEVICE=new-devname007
         *                 ^^^^^^^^^^^^^^
         *                 new dev name */
        static ref REGEX_DEVICE: Regex = Regex::new(r"^\s*DEVICE=(\S[^:]{0,14})").unwrap();

        /* Look for line with mac address and store its value in group for later
         * regex: ^\s*HWADDR=(([0-9A-Fa-f]{2}[:]){5}([0-9A-Fa-f]{2}))
         * ^\s*HWADDR=(group1) - look for line starting with `HWADDR=` (ignore whitespaces) following with group of characters describing hw address of device
         * group1: (([0-9A-Fa-f]{2}[:]){5}([0-9A-Fa-f]{2})) - match 48-bit hw address expressed in hexadecimal system ; each of inner 8-bits are separated with `:` character ; case insensitive
         * example: HWADDR=00:1b:44:11:3A:B7
         *                 ^^^^^^^^^^^^^^^^^
         *                 hw address of if */
        static ref REGEX_HWADDR: Regex = Regex::new(r"^\s*HWADDR=(([0-9A-Fa-f]{2}[:]){5}([0-9A-Fa-f]{2}))").unwrap();
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
        if hwaddr.unwrap().to_string().to_lowercase().ne(mac_address) {
            Err("new device name not found".into())
        } else {
            Ok(device)
        }
    } else {
        Err("new device name not found".into())
    }
}

#[cfg(test)]
pub mod should {
    use super::*;

    use std::str::FromStr;

    const TEST_CONFIG_DIR: &str = "./tests/unit_test_data/ifcfgs";
    const TEST_KERNEL_CMDLINE_DIR: &str = "./tests/unit_test_data/cmdlines";

    #[test]
    fn parse_cmdline() {
        let mac_address = MacAddress::from_str("AA:BB:CC:DD:EE:1F")
            .unwrap()
            .to_string()
            .to_lowercase();
        let kernel_cmdline_path = Path::new(TEST_KERNEL_CMDLINE_DIR).join("1_should_pass");

        let device_config_name = match kernel_cmdline(&mac_address, &kernel_cmdline_path) {
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

        let device_config_name = match kernel_cmdline(&mac_address, &kernel_cmdline_path) {
            Ok(Some(name)) => name,
            _ => String::from(""),
        };

        assert_eq!("unit_test_2", device_config_name);
    }

    #[test]
    fn parse_ifcfg_configuration() {
        let mac_address = MacAddress::from_str("AA:BB:CC:DD:EE:3F")
            .unwrap()
            .to_string()
            .to_lowercase();
        let ifcfg_config_path = Path::new(TEST_CONFIG_DIR).join("ifcfg-eth0");

        let test_result = match config_file(&ifcfg_config_path, &mac_address) {
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

        let test_result = match config_file(&ifcfg_config_path, &mac_address) {
            Ok(Some(_)) => true,
            _ => false,
        };

        assert!(test_result);
    }
}
