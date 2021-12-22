#[path = "./parse.rs"]
mod parse;

#[path = "./lib.rs"]
mod lib;

#[cfg(test)]
pub mod should {
    use super::*;

    use std::path::Path;
    use std::str::FromStr;

    use mac_address::MacAddress;

    const TEST_CONFIG_DIR: &str = "./tests/unit_test_data/ifcfgs";
    const TEST_KERNEL_CMDLINE_DIR: &str = "./tests/unit_test_data/cmdlines";

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

    #[test]
    #[should_panic]
    fn check_for_test_mode() {
        const NUMBER_PARAMS_REQUIRED: usize = 3;
        const ARGS: Vec<String> = Vec::new();

        let is_test_mode = lib::is_test_mode(&ARGS, NUMBER_PARAMS_REQUIRED);

        assert!(is_test_mode);
    }

    #[test]
    fn check_for_kernel_cmdline_path() {
        const IS_TEST_MODE: bool = false;
        const ARGS: &Vec<String> = &Vec::new();
        let expected: &Path = &Path::new("/proc/cmdline");

        let kernel_cmdline = lib::get_kernel_cmdline(IS_TEST_MODE, &ARGS);

        assert_eq!(expected, kernel_cmdline);
    }

    #[test]
    fn check_if_is_like_kernel_name() {
        const KERNEL_LIKE_NAME: &str = "eth123";

        let is_like_kernel = lib::is_like_kernel_name(KERNEL_LIKE_NAME);

        assert!(is_like_kernel);
    }
}
