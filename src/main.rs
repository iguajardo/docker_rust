use std::process::Stdio;

// Usage: your_docker.sh run <image> <command> <arg1> <arg2> ...
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let command = &args[3];
    let command_args = &args[4..];
    let output = std::process::Command::new(command)
        .args(command_args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();

    if output.status.success() {
        let std_out = std::str::from_utf8(&output.stdout).unwrap();
        println!("{}", std_out);
    } else {
        let std_err = std::str::from_utf8(&output.stderr).unwrap();
        println!("{}", std_err);
        std::process::exit(1);
    }
}
