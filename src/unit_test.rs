#[path = "./parse.rs"]
mod parse;

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
}
