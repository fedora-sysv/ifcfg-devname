// TODO: https://rust-cli.github.io/book/tutorial/testing.html
use assert_cmd::Command; // Add methods on commands
// use predicates::prelude::*; // Used for writing assertions

#[test]
fn integration_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("ifcfg_devname")?;

    let assert = cmd
        .env("INTERFACE", "new_name")
        .args(&["./cmdlines/cmdline", "./ifcfgs/"])
        .assert();
    assert.failure().code(1);

    Ok(())
}