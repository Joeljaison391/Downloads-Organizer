# Download Manager

## Overview
A lightweight Rust-based file monitoring and organizing tool designed to automatically segregate files downloaded into appropriate folders based on their type. The program monitors the **Downloads** folder for file events and organizes files into categories like Images, Videos, Documents, Archives, Audio, and Others.

## Features
- Monitors the Downloads folder for new files.
- Ignores temporary and partially downloaded files (e.g., `.tmp`, `.crdownload`).
- Waits for file stability before processing.
- Categorizes files into appropriate subfolders.
- Sends desktop notifications upon successful file organization.
- Logs errors and processing details for debugging.
- Efficient resource usage.

## Supported File Types
### Categories:
1. **Images**:
   - `.jpg`, `.png`, `.gif`, `.bmp`, `.tiff`, `.svg`, `.webp`
2. **Videos**:
   - `.mp4`, `.mkv`, `.avi`, `.mov`, `.flv`, `.wmv`, `.webm`, `.mpeg`
3. **Documents**:
   - `.pdf`, `.doc`, `.docx`, `.xls`, `.xlsx`, `.ppt`, `.pptx`, `.txt`, `.csv`
4. **Archives**:
   - `.zip`, `.rar`, `.7z`, `.tar`, `.gz`, `.bz2`, `.xz`, `.iso`, `.dmg`
5. **Audio**:
   - `.mp3`, `.wav`, `.aac`, `.flac`, `.ogg`, `.wma`, `.m4a`
6. **Others**:
   - Any files not matching the above categories.

## Installation
### Requirements
- Windows Operating System
- [Rust](https://www.rust-lang.org/) (for building from source)

### Steps
1. Clone the repository:
   ```bash
   git clone https://github.com/Joeljaison391/Downloads-Organizer.git
   cd download-manager
   ```
2. Build the project:
   ```bash
   cargo build --release
   ```
3. Locate the executable:
   ```bash
   target/release/downloadManager.exe
   ```
4. Optionally, configure it to run at system startup using Task Scheduler (see instructions below).

Alternatively, you can download the prebuilt executable from the [releases page](https://github.com/Joeljaison391/Downloads-Organizer/releases) and configure it directly in Task Scheduler.

## Task Scheduler Setup
1. Open Task Scheduler (`Win + R`, type `taskschd.msc`, and press Enter).
2. Create a new task and name it (e.g., **File Monitor**).
3. Set the trigger to **At system startup**.
4. In the Actions tab, select **Start a program** and browse to the executable path.
5. Save and test the task.

## Running the Program
To manually run the program, execute:
```bash
target/release/downloadManager.exe
```
The program will monitor the Downloads folder and organize files as events occur.

## Benchmark Testing
To test the program's resource usage and performance:
1. Build and run the benchmark tool:
   ```bash
   cargo run --bin benchmark
   ```
2. Alternatively, download the prebuilt benchmark executable from the [releases page](https://github.com/Joeljaison391/Downloads-Organizer/releases).
3. The benchmark will measure CPU and memory usage during file processing events.

### Device Information
The benchmark has been tested on the following device:
- **Device Name**: Jannuarry
- **Processor**: 12th Gen Intel(R) Core(TM) i5-12450H @ 2.00 GHz
- **Installed RAM**: 32.0 GB (31.7 GB usable)
- **OS**: Windows 11 Home Single Language, Version 23H2
- **Build**: 22631.4602
- **Windows Feature Experience Pack**: 1000.22700.1055.0

### Benchmark Results
- **CPU Usage**: Peaks around ~0.77% during active monitoring.
- **Memory Usage**: Stable at ~13 MB during file monitoring.

## Logging
Error logs are saved in the home directory as `file_monitor_logs.txt`:
```
C:\Users\<YourUsername>\file_monitor_logs.txt
```

## License
This project is licensed under the [Apache License 2.0](LICENSE).
