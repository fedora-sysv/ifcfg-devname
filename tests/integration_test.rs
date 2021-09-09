// TODO: https://rust-cli.github.io/book/tutorial/testing.html
// ? Could be great to use: https://blog.cyplo.net/posts/2018/12/generate-rust-tests-from-data/
use assert_cmd::Command; // Add methods on commands
// use predicates::prelude::*; // Used for writing assertions
use std::path::Path;
use std::fs::{
    self
};

use serde::{
    Deserialize,
    Serialize
};

#[derive(Serialize, Deserialize)]
struct Dataset {
    name: String,
    description: String,
    input: DatasetInput,
    output: DatasetOutput
}

#[derive(Serialize, Deserialize)]
struct DatasetInput {
    interface: String,
    hw_address: String
}

#[derive(Serialize, Deserialize)]
struct DatasetOutput {
    should_fail: bool,
    expected_name: String
}

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
            let config_path = path.join("about.json");
            let cmdline_path = path.join("cmdline");
            let ifcfgs_dir_path = path.join("ifcfgs");

            /* Read JSON configuration and then serialize it using srade_json */
            let dataset_configuration: Dataset = serde_json::from_str(
                &fs::read_to_string(config_path)?
            )?;

            let assert = cmd
                .env("INTERFACE", dataset_configuration.input.interface)
                .args(&[
                    cmdline_path.into_os_string().into_string().unwrap(), 
                    ifcfgs_dir_path.into_os_string().into_string().unwrap(),
                    dataset_configuration.input.hw_address
                ])
                .assert();
            assert.failure().code(1);
        }
    }

    Ok(())
}
