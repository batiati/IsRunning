/// is_running is a command line utility designed to check if some processes are running
/// It's main goal is to be used as healthy-check probe on containers
/// Usage:
/// ./is_running nginx
use std::env;
use sysinfo::{ProcessExt, SystemExt};

const SUCCESS: i32 = 0;
const ERROR: i32 = 1;

fn main() {
    let mut sys = sysinfo::System::new();
    sys.refresh_processes();

    let processes = sys.get_processes();

    let found = env::args()
        .skip(1)
        .all(|name| processes.values().any(|process| process.name() == name));

    if found {
        std::process::exit(SUCCESS);
    } else {
        std::process::exit(ERROR);
    }
}
