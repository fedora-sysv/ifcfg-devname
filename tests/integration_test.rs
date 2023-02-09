// SPDX-License-Identifier: GPL-3.0-or-later

use std::fs::{self};
use std::path::Path;

/* Command execution */
use assert_cmd::Command;
use predicates::prelude::*;

/* JSON */
use serde::{Deserialize, Serialize};

// --- Dataset configuration structure --- //

#[derive(Serialize, Deserialize)]
struct Dataset {
    name: String,
    description: String,
    input: DatasetInput,
    output: DatasetOutput,
}

#[derive(Serialize, Deserialize)]
struct DatasetInput {
    interface: String,
    hw_address: String,
}

#[derive(Serialize, Deserialize)]
struct DatasetOutput {
    should_fail: bool,
    expected_name: String,
}

#[test]
fn integration_test_datasets() -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = Path::new("./tests/integration_test_data");

    /* Loop through datasets in directory ./data */
    for entry in fs::read_dir(data_dir)? {
        let path = entry?.path();

        /* For each dataset; load configuration and run ifcfg-devname binary */
        if path.is_dir() {
            let mut cmd = Command::cargo_bin("ifcfg-devname")?;

            let config_path = path.join("about.json");
            let ifcfgs_dir_path = path.join("ifcfgs");

            /* Read JSON configuration and then serialize it using srade_json */
            let dataset_configuration: Dataset =
                serde_json::from_str(&fs::read_to_string(config_path)?)?;

            /* Run ifcfg-devname with parameters from given dataset */
            let dataset_assert = cmd
                .env("INTERFACE", dataset_configuration.input.interface)
                .args(&[
                    ifcfgs_dir_path.into_os_string().into_string().unwrap(), /* ifcfgs directory */
                    dataset_configuration.input.hw_address,                  /* hw address */
                ])
                .assert();

            /* Test result evaluation */
            if dataset_configuration.output.should_fail {
                dataset_assert.failure().code(1); /* Expected Error code */
            } else {
                dataset_assert.success().stdout(predicate::str::is_match(
                    dataset_configuration.output.expected_name,
                )?);
            }
        }
    }

    Ok(())
}

#[test]
fn integration_test_no_env() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("ifcfg-devname")?;

    cmd.assert().failure().code(1);

    Ok(())
}
