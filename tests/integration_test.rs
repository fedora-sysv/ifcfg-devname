// TODO: https://rust-cli.github.io/book/tutorial/testing.html
// ? Could be great to use: https://blog.cyplo.net/posts/2018/12/generate-rust-tests-from-data/
use assert_cmd::Command; // Add methods on commands
// use predicates::prelude::*; // Used for writing assertions

#[test]
fn integration_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("ifcfg_devname")?;

    // read dir data/
    // for each dataset run binary ifcfg_devname
    // print outputs/results
    
    let assert = cmd
        .env("INTERFACE", "new_name")
        .args(&["./cmdlines/cmdline", "./ifcfgs/", "AA:BB:CC:DD:EE:FF"])
        .assert();
    assert.failure().code(1);

    Ok(())
}
