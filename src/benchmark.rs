use sysinfo::{System, SystemExt, ProcessExt};
use std::process::{Command};
use std::{thread, time};

fn main() {
    let mut child = Command::new("target/debug/downloadManager")
        .spawn()
        .expect("Failed to start file monitor");

    let mut sys = System::new_all();
    let pid = child.id();
    println!("Monitoring process with PID: {}", pid);

    for _ in 0..10 { 
        sys.refresh_all();
        if let Some(process) = sys.process(sysinfo::Pid::from(pid as usize)) {
            println!(
                "CPU Usage: {:.2}%, Memory Usage: {} KB",
                process.cpu_usage(),
                process.memory()
            );
        } else {
            println!("Process not found.");
            break;
        }

        thread::sleep(time::Duration::from_secs(2));
    }

    let _ = child.kill();
}
