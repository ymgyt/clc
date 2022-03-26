use clc_cli::ClcApp;

fn main() {
    let app = ClcApp::parse();
    if let Err(err) = app.exec() {
        eprintln!("{err}");
    }
}
