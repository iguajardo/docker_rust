use std::process::Stdio;

// Usage: your_docker.sh run <image> <command> <arg1> <arg2> ...
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let command = &args[3];
    let command_args = &args[4..];
    let output = std::process::Command::new(command)
        .args(command_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .unwrap();

    if output.status.success() {
        // let std_out = std::str::from_utf8(&output.std_out);
        // print!({}, std_out);
        // let std_err = std::str::from_utf8(&output.std_err);
        // eprint!({}, std_err);
        std::process::exit(0);
    }
}
