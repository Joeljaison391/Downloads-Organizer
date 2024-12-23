mod report;

use notify::{Watcher, RecursiveMode, RecommendedWatcher, Config};
use std::io::Write;
use std::sync::mpsc::channel;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, Instant};
use notify_rust::Notification;
use chrono::{Local, Duration, Datelike};
use report::generate_html_report;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let downloads_folder = dirs::download_dir().unwrap_or_else(|| {
        let fallback = std::env::current_dir().expect("Failed to get current directory").join("Downloads");
        std::fs::create_dir_all(&fallback).expect("Failed to create fallback Downloads directory");
        fallback
    });
    let unused_folder = downloads_folder.join("Unused");
    let report_file = downloads_folder.join("Weekly_Report.html");
    let report_status_file = downloads_folder.join("report_status.txt");
    fs::create_dir_all(&unused_folder)?;

    // Check and generate weekly report if it's a new week
    if is_new_week(&report_status_file) {
        println!("Generating weekly report...");
        generate_html_report(&downloads_folder, &report_file)?;
        update_report_status(&report_status_file);
        println!("Weekly report generated: {}", report_file.display());
    }

    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default()).map_err(|e| {
        eprintln!("Error creating file watcher: {}", e);
        e
    })?;


    watcher.watch(&downloads_folder, RecursiveMode::Recursive).map_err(|e| {
        eprintln!("Error watching folder: {}", e);
        e
    })?;

    println!("Monitoring folder: {}", downloads_folder.display());

    let mut last_scan = Instant::now();

    for event in rx {
        // Log the elapsed time every loop iteration
        let elapsed = last_scan.elapsed().as_secs();
        println!("Elapsed time since last scan: {} seconds", elapsed);

        match event {
            Ok(event) => {
                if let Some(path) = event.paths.first() {
                    // Skip events for files inside the Unused folder
                    if path.starts_with(&unused_folder) {
                        continue;
                    }

                    if let Err(e) = handle_file_event(path, &downloads_folder, &unused_folder) {
                        log_error(&e.to_string());
                        eprintln!("Error handling file: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("File watcher error: {}", e);
            }
        }


        if elapsed >= 60 {
            println!("Starting periodic scan for unused files...");
            log_event("Starting periodic scan for unused files...");
            if let Err(e) = handle_unused_files_recursively(&downloads_folder, &unused_folder) {
                log_error(&e.to_string());
                eprintln!("Error during periodic scan: {}", e);
            }
            println!("Periodic scan completed.");
            log_event("Periodic scan completed.");
            last_scan = Instant::now();
        }
    }

    Ok(())
}

fn is_new_week(report_status_file: &Path) -> bool {
    let today = Local::now().date();
    let week_start = today - chrono::Duration::days(today.weekday().num_days_from_sunday() as i64);

    if report_status_file.exists() {
        let data = fs::read_to_string(report_status_file).unwrap_or_default();
        if let Ok(last_generated) = data.parse::<chrono::NaiveDate>() {
            return last_generated < week_start.naive_local();
        }
    }

    true // If the file doesn't exist, assume it's a new week
}

fn update_report_status(report_status_file: &Path) {
    let today = Local::now().date();
    fs::write(report_status_file, today.format("%Y-%m-%d").to_string()).unwrap();
}

fn handle_file_event(path: &Path, downloads_folder: &Path, unused_folder: &Path) -> Result<(), std::io::Error> {
    if !path.is_file() {
        return Ok(()); // Skip directories or non-files
    }

    if let Some(extension) = path.extension() {
        if extension == "tmp" {
            println!("Ignoring temporary file: {}", path.display());
            return Ok(());
        }
    }

    println!("Detected file event: {}", path.display());
    log_event(&format!("Detected file event: {}", path.display()));

    let mut prev_size = 0;
    loop {
        match fs::metadata(path) {
            Ok(metadata) => {
                let current_size = metadata.len();

                if current_size == prev_size {
                    break;
                }

                prev_size = current_size;
                println!("Waiting for file stability: {}", path.display());
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    println!("File renamed or deleted: {}", path.display());
                    return Ok(());
                } else {
                    return Err(e);
                }
            }
        }
    }

    // Move the file to its specific folder
    move_file_to_specific_folder(path, downloads_folder)?;

    Ok(())
}

fn handle_unused_files_recursively(downloads_folder: &Path, unused_folder: &Path) -> Result<(), std::io::Error> {
    for entry in fs::read_dir(downloads_folder)? {
        let entry = entry?;
        let path = entry.path();

        // Skip the `Unused` directory
        if path == *unused_folder {
            continue;
        }

        if path.is_dir() {
            // Recurse into subdirectories
            handle_unused_files_recursively(&path, unused_folder)?;
        } else if path.is_file() {
            println!("Checking file for unused status: {}", path.display());
            log_event(&format!("Checking file for unused status: {}", path.display()));
            move_unused_files(&path, unused_folder)?;
        }
    }
    Ok(())
}

fn move_file_to_specific_folder(path: &Path, downloads_folder: &Path) -> Result<(), std::io::Error> {
    let target_dir = match path.extension().and_then(|ext| ext.to_str()) {
        Some(ext) => match ext.to_lowercase().as_str() {
            "jpg" | "png" | "gif" | "bmp" | "tiff" | "svg" | "webp" => "Images",
            "mp4" | "mkv" | "avi" | "mov" | "flv" | "wmv" | "webm" | "mpeg" => "Videos",
            "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" | "txt" | "csv" => "Documents",
            "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" | "xz" | "iso" | "dmg" => "Archives",
            "mp3" | "wav" | "aac" | "flac" | "ogg" | "wma" | "m4a" => "Audio",
            _ => "Others",
        },
        None => "Others",
    };

    let target_path = downloads_folder.join(target_dir);
    fs::create_dir_all(&target_path)?;

    let file_name = path.file_name().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::Other, "Failed to get file name")
    })?;
    let new_path = target_path.join(file_name);

    if new_path != path {
        fs::rename(path, &new_path)?;
        println!("Moved '{}' to '{}'", path.display(), target_dir);
        log_event(&format!("Moved '{}' to '{}'", path.display(), target_dir));

        send_notification(file_name.to_string_lossy().as_ref(), target_dir)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    }

    Ok(())
}

fn move_unused_files(path: &Path, unused_folder: &Path) -> Result<(), std::io::Error> {
    let cutoff_time = SystemTime::now() - Duration::seconds(30 * 24 * 60 * 60).to_std().unwrap();


    if let Ok(metadata) = fs::metadata(&path) {
        if let Ok(modified) = metadata.modified() {
            if modified < cutoff_time {
                let target_dir = match path.extension().and_then(|ext| ext.to_str()) {
                    Some(ext) => match ext.to_lowercase().as_str() {
                        "jpg" | "png" | "gif" | "bmp" | "tiff" | "svg" | "webp" => "Images",
                        "mp4" | "mkv" | "avi" | "mov" | "flv" | "wmv" | "webm" | "mpeg" => "Videos",
                        "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" | "txt" | "csv" => "Documents",
                        "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" | "xz" | "iso" | "dmg" => "Archives",
                        "mp3" | "wav" | "aac" | "flac" | "ogg" | "wma" | "m4a" => "Audio",
                        _ => "Others",
                    },
                    None => "Others",
                };

                let target_path = unused_folder.join(target_dir);
                fs::create_dir_all(&target_path)?;

                let file_name = path.file_name().ok_or_else(|| {
                    std::io::Error::new(std::io::ErrorKind::Other, "Failed to get file name")
                })?;
                let new_path = target_path.join(file_name);

                fs::rename(path, &new_path)?;
                println!("Moved '{}' to 'Unused/{}'", path.display(), target_dir);
                log_event(&format!("Moved '{}' to 'Unused/{}'", path.display(), target_dir));

                send_notification(
                    file_name.to_string_lossy().as_ref(),
                    target_path.to_string_lossy().as_ref(),
                )
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
            }
        }
    }

    Ok(())
}

fn log_error(message: &str) {
    if let Some(logs_dir) = dirs::home_dir().map(|dir| dir.join("file_monitor_logs.txt")) {
        if let Ok(mut file) = fs::OpenOptions::new().create(true).append(true).open(&logs_dir) {
            let _ = writeln!(
                file,
                "[{}] ERROR: {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                message
            );
        }
    }
}

fn log_event(message: &str) {
    if let Some(logs_dir) = dirs::home_dir().map(|dir| dir.join("file_monitor_logs.txt")) {
        if let Ok(mut file) = fs::OpenOptions::new().create(true).append(true).open(&logs_dir) {
            let _ = writeln!(
                file,
                "[{}] EVENT: {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                message
            );
        }
    }
}

fn send_notification(file_name: &str, target_dir: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Notification::new()
        .summary("File Moved")
        .body(&format!("'{}' has been moved to '{}'.", file_name, target_dir))
        .show()?;
    Ok(())
}
