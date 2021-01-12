/// is_running is a command line utility designed to check if some process is running
/// It's main goal is to be used as healthy-check probe on containers

use sysinfo::{SystemExt, ProcessExt};
use std::env;

const ERROR : i32 = -1;
const SUCCESS : i32 = 0;

fn main() {

    let mut args  = env::args();

    //First arg is the process's name
    args.next();

    let mut exit_code= ERROR;

    if let Some(name) = args.next() {

        let mut sys = sysinfo::System::new();
        sys.refresh_processes();
        let exists = sys.get_processes()
            .values()
            .any(|x| x.name() == name);

       if exists {
           exit_code = SUCCESS;
       };
    }

    std::process::exit(exit_code);
}
