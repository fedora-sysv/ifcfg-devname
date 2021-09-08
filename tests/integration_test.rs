// TODO: https://rust-cli.github.io/book/tutorial/testing.html
// ? Could be great to use: https://blog.cyplo.net/posts/2018/12/generate-rust-tests-from-data/
use assert_cmd::Command; // Add methods on commands
// use predicates::prelude::*; // Used for writing assertions
use std::path::Path;
use std::fs::{self};

#[test]
fn integration_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("ifcfg_devname")?;

    // print outputs/results

    /* Loop through datasets in directory ./data */
    let data_dir = Path::new("./tests/data");
    for entry in fs::read_dir(data_dir)? {
        let path = entry?.path();

        /* For each dataset; load configuration and run ifcfg_devname binary */
        if path.is_dir() {
            let config_path = Path::new(format!("{}/config.json", path.unwrap()));
            config_path.push("/config.json");
            let cmdline_path = dataset_path.clone().push("/cmdline");
            let ifcfgs_dir_path = dataset_path.clone().push("/ifcfgs"); 

            println!("path: {:?}", config_path);

            //let foo = fs::read_to_string(config_path)?;

            let assert = cmd
                .env("INTERFACE", "new_name")
                .args(&["./cmdlines/cmdline", "./ifcfgs/", "AA:BB:CC:DD:EE:FF"])
                .assert();
            assert.failure().code(1);
        }
    }

    Ok(())
}
