#![windows_subsystem = "windows"]

use std::thread;
use std::time::Duration;

fn main() {
    loop {
        thread::sleep(Duration::from_secs(3600));
    }
}

