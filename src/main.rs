use std::process::{Stdio};

// Usage: your_docker.sh run <image> <command> <arg1> <arg2> ...
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let command = &args[3];
    let command_args = &args[4..];
    let output = std::process::Command::new(command)
        .args(command_args)
        .stdout(Stdio::inherit()) // Stdio::piped() // not needed if .output exists
        .stderr(Stdio::inherit()) // Stdio::piped()
        .output()
        .unwrap();

    std::process::exit(output.status.code().unwrap());
}
