use std::env::set_current_dir;
use std::fs;
use std::os::unix::fs::chroot;
use std::path::{Path, PathBuf};

const CHROOT_PATH: &str = "./.sandbox";
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let command = &args[3];
    let command_args = &args[4..];

    let file_name = isolate(command);

    let output = std::process::Command::new(format!("/{}", file_name))
        .args(command_args)
        .output()
        .unwrap();

    print!("{}", String::from_utf8_lossy(&output.stdout));
    eprint!("{}", String::from_utf8_lossy(&output.stderr));

    std::process::exit(output.status.code().unwrap());
}

fn isolate(command: &str) -> String {
    let path = PathBuf::from(command);
    let file_name = path.file_name().unwrap().to_str().unwrap();

    if !Path::new(CHROOT_PATH).exists() {
        fs::create_dir(CHROOT_PATH).expect("not possible to create chroot dir");
    }

    std::fs::copy(command, format!("./.sandbox/{}", file_name)).expect("could not copy command");

    chroot(CHROOT_PATH).expect("not possible to use chroot");
    set_current_dir("/").expect("not possible to set the current dir");

    #[cfg(target_os = "linux")]
    unsafe {
        // creates a new PID namespace and the process and every new child process will be executed in this new PID namespace.
        libc::unshare(libc::CLONE_NEWPID);
    }

    if !Path::new("/dev").exists() {
        fs::create_dir("/dev").expect("not possible to create dev dir");
    }
    std::fs::File::create("/dev/null").expect("creation of null file failed");

    file_name.to_string()
}
