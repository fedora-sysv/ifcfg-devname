use clap::{App, Arg};

// https://stackoverflow.com/a/45176487
const DEFAULT_ROOT_PATH: &str = "/";

fn main() {
    let matches: clap::ArgMatches;
    let hw_addr: String;
    let root_path: String;
    let is_verbose_disabled: bool;
    
    matches = App::new("rename_device")
        .version("1.0")
        .author("Macku Jan <jamacku@redhat.com>")
        .about("Does awesome things")
        // *NOTE:* Doesn't work right now...
        //.license("MIT OR Apache-2.0")
        .arg(
            Arg::new("hwaddr")
                .short('m')
                .long("hwaddr")
                .value_name("HWADDR_INPUT")
                .takes_value(true)
                .required(true)
                .about("Hardware address of device which is going to be look for. This option is required."),
        )
        .arg(
            Arg::new("root")
                .short('r')
                .long("root")
                .value_name("ROOT_PATH")
                .takes_value(true)
                .required(false)
                .about("Allows to set custom path where to look for configuration. If not set, defaults to '/'."),
        )
        .get_matches();

    /* Check HWADDR */
    if let Some(o) = matches.value_of("hwaddr") {
        hw_addr = o.to_string();
        println!("Value for HWADDR: {}", hw_addr);
    }

    /* Check ROOT_PATH */
    if let Some(o) = matches.value_of("root") {
        root_path = o.to_string();
        println!("Value for ROOT_PATH: {}", root_path);
    }

    // TODO: rename_device logic...
}
