use assert_cmd::Command;

#[test]
fn version() {
    let mut clc = bin();
    clc.arg("--version");
    clc.assert()
        .success()
        .stdout(format!("clc {}\n", env!("CARGO_PKG_VERSION")));
}

// NOTE: IMPROVEMENT
// Currently we have no idea to test eval loop :(

fn bin() -> Command {
    Command::cargo_bin("clc").unwrap()
}
