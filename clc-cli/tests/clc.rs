use assert_cmd::Command;

#[test]
fn flag_version() {
    let mut clc = bin();
    clc.arg("--version");
    clc.assert()
        .success()
        .stdout(format!("clc {}\n", env!("CARGO_PKG_VERSION")));
}

#[test]
fn flag_eval() {
    for flag in ["--eval", "-e"] {
        let mut clc = bin();
        clc.args([flag, "sqrt(100)"]);
        clc.assert().success().stdout("10\n");
    }
}

// NOTE: IMPROVEMENT
// Currently we have no idea to test eval loop :(

fn bin() -> Command {
    Command::cargo_bin("clc").unwrap()
}
