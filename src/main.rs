use notify::{Watcher, RecursiveMode, RecommendedWatcher, Config};
use std::io::Write;
use std::sync::mpsc::channel;
use std::fs;
use std::path::Path;
use notify_rust::Notification;
use chrono::Local;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let downloads_folder = dirs::download_dir().expect("Failed to locate Downloads folder");
    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default()).map_err(|e| {
        eprintln!("Error creating file watcher: {}", e);
        e
    })?;

    watcher.watch(&downloads_folder, RecursiveMode::NonRecursive).map_err(|e| {
        eprintln!("Error watching folder: {}", e);
        e
    })?;

    println!("Monitoring folder: {}", downloads_folder.display());

    for event in rx {
        match event {
            Ok(event) => {
                if let Some(path) = event.paths.first() {
                    if let Err(e) = handle_file_event(path, &downloads_folder) {
                        log_error(&e.to_string());
                        eprintln!("Error handling file: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("File watcher error: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_file_event(path: &Path, downloads_folder: &Path) -> Result<(), std::io::Error> {
    if !path.is_file() {
        return Ok(()); // Ignore directories
    }

    // Ignore `.tmp` files entirely
    if let Some(extension) = path.extension() {
        if extension == "tmp" {
            println!("Ignoring temporary file: {}", path.display());
            return Ok(()); // Skip temporary files
        }
    }

    // Wait for file stability
    println!("Detected file event: {}", path.display());
    let mut prev_size = 0;
    loop {
        match fs::metadata(path) {
            Ok(metadata) => {
                let current_size = metadata.len();

                if current_size == prev_size {
                    break; // File is stable; break loop
                }

                prev_size = current_size;
                println!("Waiting for file stability: {}", path.display());
                std::thread::sleep(std::time::Duration::from_secs(1)); // Wait 1 second
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    println!("File renamed or deleted: {}", path.display());
                    return Ok(()); // Gracefully exit for this file
                } else {
                    return Err(e); // Return other errors
                }
            }
        }
    }

    // Re-check extension after stabilization
    if let Some(extension) = path.extension() {
        if extension == "tmp" {
            println!("Skipping file: {} (still temporary)", path.display());
            return Ok(());
        }
    }

    // Determine target directory based on final file extension
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

    // Create target directory if it doesn't exist
    let target_path = downloads_folder.join(target_dir);
    fs::create_dir_all(&target_path)?;

    // Move the file
    let file_name = path.file_name().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::Other, "Failed to get file name")
    })?;
    let new_path = target_path.join(file_name);
    fs::rename(path, &new_path)?;

    // Send notification
    send_notification(file_name.to_string_lossy().as_ref(), target_dir)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    Ok(())
}

fn log_error(message: &str) {
    if let Some(logs_dir) = dirs::home_dir().map(|dir| dir.join("file_monitor_logs.txt")) {
        if let Ok(mut file) = fs::OpenOptions::new().create(true).append(true).open(&logs_dir) {
            let _ = writeln!(
                file,
                "[{}] {}",
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
        .show()?; // Display notification
    Ok(())
}
