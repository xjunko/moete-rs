use std::process::Command;

fn main() {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();

    let git_hash_full = String::from_utf8(output.stdout).unwrap();
    let git_hash_trimmed = &git_hash_full[..7];

    println!("cargo:rustc-env=GIT_HASH={}", git_hash_trimmed);
}
