use std::path::Path;
use std::error;
use std::str::FromStr;

use mac_address::{mac_address_by_name, MacAddress};

use lazy_static::lazy_static;
use regex::Regex;

/* Check if new devname is equal to kernel standard devname (eth0, etc.) */
pub fn is_like_kernel_name(new_devname: &str) -> bool {
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

    if IS_NEW_DEVNAME_ETH_LIKE.is_match(&new_devname) {
        true
    } else {
        false
    }
}

pub fn is_test_mode(params: &Vec<String>, number_params_required: usize) -> bool {
    if params.len() > number_params_required {
        true
    } else {
        false
    }
}

pub fn get_kernel_cmdline(is_test_mode: bool, args: &Vec<String>, index: usize) -> &Path {
    const KERNEL_CMDLINE: &str = "/proc/cmdline";

    let kernel_cmdline = if is_test_mode {
        Path::new(&args[index])
    } else {
        Path::new(KERNEL_CMDLINE)
    };

    kernel_cmdline
}

pub fn get_mac_address(is_test_mode: bool, args: &Vec<String>, index: usize, kernel_name: &String) -> Result<MacAddress, Box<dyn error::Error>> {
    let mac_address = if is_test_mode {
        let mac_address = args[index].clone();
        MacAddress::from_str(&mac_address)?
    } else {
        match mac_address_by_name(kernel_name)? {
            Some(mac) => mac,
            None => panic!()
        }
    };

    Ok(mac_address)
}

#[cfg(test)]
pub mod should {
    use super::*;

    #[test]
    #[should_panic]
    fn check_for_test_mode() {
        const NUMBER_PARAMS_REQUIRED: usize = 3;
        const ARGS: Vec<String> = Vec::new();

        let is_test_mode = is_test_mode(&ARGS, NUMBER_PARAMS_REQUIRED);

        assert!(is_test_mode);
    }

    #[test]
    fn check_for_kernel_cmdline_path() {
        const IS_TEST_MODE: bool = false;
        const ARGS: &Vec<String> = &Vec::new();
        const INDEX: usize = 1;
        let expected: &Path = &Path::new("/proc/cmdline");

        let kernel_cmdline = get_kernel_cmdline(IS_TEST_MODE, ARGS, INDEX);

        assert_eq!(expected, kernel_cmdline);
    }

    #[test]
    fn check_if_is_like_kernel_name() {
        const KERNEL_LIKE_NAME: &str = "eth123";

        let is_like_kernel = is_like_kernel_name(KERNEL_LIKE_NAME);

        assert!(is_like_kernel);
    }

    #[test]
    #[should_panic]
    fn not_get_mac_address() {
        const IS_TEST_MODE: bool = false;
        const ARGS: &Vec<String> = &Vec::new();
        const INDEX: usize = 3;
        let kernel_name: String = String::from_str("this-should-fail").unwrap();

        let _ = get_mac_address(IS_TEST_MODE, ARGS, INDEX, &kernel_name);
    }
}