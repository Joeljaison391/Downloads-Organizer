use sysinfo::{System, SystemExt, ProcessExt};
use std::process::{Command};
use std::{thread, time};

fn main() {
    // Start the file monitor application
    let mut child = Command::new("target/debug/downloadManager")
        .spawn()
        .expect("Failed to start file monitor");

    // Monitor its resource usage
    let mut sys = System::new_all();
    let pid = child.id();
    println!("Monitoring process with PID: {}", pid);

    for _ in 0..10 { // Check 10 times with a 2-second interval
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

        thread::sleep(time::Duration::from_secs(2)); // Wait for 2 seconds
    }

    // Terminate the application
    let _ = child.kill();
}
