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

    // If command is "exit <code>", the output is an exit with the status of ExitStatus that represent
    // the termination of the child. Can get the code with code()
    std::process::exit(output.status.code().unwrap());
}
