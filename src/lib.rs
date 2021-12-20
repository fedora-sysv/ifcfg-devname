// use std::path::Path;

use lazy_static::lazy_static;
use regex::Regex;

// const KERNEL_CMDLINE: &str = "/proc/cmdline";

// pub fn get_kernel_cmdline() {
//     if !is_correct_number_args {
//         Path::new(KERNEL_CMDLINE)
//     } else {
//         Path::new(&args[1])
//     };
// }

// pub fn process_arguments(args: &env::Args) -> Result<&(Option<Path>, Option<Path>, Option<MacAddress>), Box<dyn error::Error>> {
//     let args: Vec<String> = env::args().collect();

//     if args.len() > 3 {
//         Ok(&(Some(Path::new(&args[1])), Some(Path::new(&args[2])), Some(MacAddress::from_str(&args[3])?)))
//     } else {
//         Ok(&(Some(Path::new()), None, None))
//     }
// }

/* Check if new devname is equal to kernel standard devname (eth0, etc.)
 * If such a name is detected return true else false */
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

    /* Look for HWADDR= */
    if IS_NEW_DEVNAME_ETH_LIKE.is_match(&new_devname) {
        true
    } else {
        false
    }
}
