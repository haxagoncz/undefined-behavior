use std::process::Command;

fn main() {
    let sha = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .unwrap();
    println!(
        "cargo:rustc-env=GIT_HASH={}",
        String::from_utf8(sha.stdout).unwrap()
    );
}
