# Download Manager

## Overview
A lightweight Rust-based file monitoring and organizing tool designed to automatically segregate files downloaded into appropriate folders based on their type. The program monitors the **Downloads** folder for file events and organizes files into categories like Images, Videos, Documents, Archives, Audio, and Others.

## Features
- **File Monitoring**: Monitors the Downloads folder for new files and organizes them based on file type.
- **Stability Check**: Waits for file stability before processing (e.g., for partially downloaded files).
- **File Categorization**: Automatically moves files into appropriate subfolders (e.g., Images, Videos, Documents).
- **Unused File Management**: Identifies unused files and moves them to the `Unused` folder, organized by type.
- **Initial Report Generation**: Generates a detailed report about the Downloads folder on the first run.
- **Weekly Reports**: Automatically generates a detailed weekly report summarizing the files in the Downloads folder.
- **Subdirectory Analysis**: Includes all subdirectories (e.g., `Images`, `Videos`, `Unused`) in report generation.
- **Desktop Notifications**: Sends notifications for file organization and report generation.
- **Logging**: Logs errors and events for easy debugging.
- **Charts and Visuals**: Weekly reports include interactive charts (file type distribution, file sizes) for better visualization.

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
- **Operating System**: Windows, Linux, or macOS
- **Rust**: [Install Rust](https://www.rust-lang.org/) (for building from source)

### Steps
1. Clone the repository:
   ```bash
   git clone https://github.com/Joeljaison391/Downloads-Organizer.git
   cd Downloads-Organizer
   ```
2. Build the project:
   ```bash
   cargo build --release
   ```
3. Locate the executable:
   ```bash
   target/release/downloadManager
   ```
4. Optionally, configure the application to run at system startup using Task Scheduler or a system service (see instructions below).

Alternatively, you can download the prebuilt executable from the [releases page](https://github.com/Joeljaison391/Downloads-Organizer/releases) and configure it directly.

## Running the Program
To manually run the program, execute:
```bash
target/release/downloadManager
```
The program will monitor the Downloads folder, organize files, and generate reports.

### Initial Run
- On the first run, the program analyzes the Downloads folder and generates an **Initial Report** summarizing the current state of the folder.

### Weekly Report
- A new weekly report is automatically generated every Sunday at 12:00 AM (or the first time the program runs during the new week).

## Task Scheduler Setup (Windows)
1. Open Task Scheduler (`Win + R`, type `taskschd.msc`, and press Enter).
2. Create a new task and name it (e.g., **Download Manager**).
3. Set the trigger to **At system startup**.
4. In the Actions tab, select **Start a program** and browse to the executable path.
5. Save and test the task.

## Subdirectory Analysis
The program supports subdirectories such as:
- **Images**
- **Videos**
- **Documents**
- **Archives**
- **Unused**

These are included in both file organization and report generation.

## Benchmark Testing
To test the program's performance:
1. Build and run the benchmark tool:
   ```bash
   cargo run --bin benchmark
   ```
2. Alternatively, download the prebuilt benchmark executable from the [releases page](https://github.com/Joeljaison391/Downloads-Organizer/releases).
3. The benchmark measures CPU and memory usage during file monitoring and processing.

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

## Weekly Report
### Features:
- **Summary**:
  - Total files and size in the Downloads folder.
  - Number and size of unused files.
- **Breakdown**:
  - File type distribution (count and size).
  - Unused file statistics.
- **Visualizations**:
  - **Pie chart** for file type distribution.
  - **Bar chart** for file sizes.

### Example File:
The report is saved as `Weekly_Report.html` in the Downloads directory.

## Logging
Error logs are saved in the home directory as `file_monitor_logs.txt`:
```
C:\Users\<YourUsername>\file_monitor_logs.txt
```

## License
This project is licensed under the [Apache License 2.0](LICENSE).
