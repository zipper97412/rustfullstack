use std::process::Command;
use std::env;
use std::path::PathBuf;

fn main() {
    let frontend_dir = {
        let mut path: PathBuf = env::var("CARGO_MANIFEST_DIR").unwrap().into();
        path.push("frontend");
        path
    };



    // note that there are a number of downsides to this approach, the comments
    // below detail how to improve the portability of these commands.
    let mut cmd = Command::new("cargo");
    cmd.current_dir(&frontend_dir);
    cmd.args(&["web", "deploy"]);
    if Ok("release".into()) == env::var("PROFILE") {
        cmd.arg("--release");
    }
    cmd.status().unwrap();
    println!("rerun-if-changed=frontend");
    println!("rerun-if-changed=target/deploy");
    println!("rerun-if-env-changed=PROFILE");
}