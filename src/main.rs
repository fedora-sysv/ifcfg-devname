use clap::{App, Arg};

fn main() {
    let matches = App::new("rename_device")
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
                .about("Sets a custom config file"),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .takes_value(false)
                .about("Sets a custom config file"),
        )
        .arg(
            Arg::new("silent")
                .short('s')
                .long("silent")
                .takes_value(false)
                .about("Sets a custom config file"),
        )
        .get_matches();

    /* Check HWADDR */
    if let Some(o) = matches.value_of("hwaddr") {
        println!("Value for HWADDR: {}", o);
    }

    // TODO: rename_device logic...
}
