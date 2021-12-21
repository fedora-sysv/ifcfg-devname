// use std::path::Path;

use lazy_static::lazy_static;
use regex::Regex;

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
