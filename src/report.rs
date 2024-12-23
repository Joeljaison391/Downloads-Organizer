use std::collections::HashMap;
use std::fs;
use std::path::Path;
use chrono::Local;

pub fn generate_html_report(downloads_folder: &Path, report_file: &Path) -> Result<(), std::io::Error> {
    let mut report_content = String::new();

    report_content.push_str(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Weekly Downloads Report</title>
            <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
            <style>
                body {
                    font-family: Arial, sans-serif;
                    background-color: #f4f4f9;
                    color: #333;
                    margin: 0;
                    padding: 0;
                }
                .container {
                    width: 90%;
                    max-width: 800px;
                    margin: 20px auto;
                    background: #fff;
                    padding: 20px;
                    border-radius: 10px;
                    box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
                }
                h1, h2, p {
                    text-align: center;
                }
                table {
                    width: 100%;
                    border-collapse: collapse;
                    margin: 20px 0;
                }
                table th, table td {
                    padding: 10px;
                    text-align: left;
                    border: 1px solid #ddd;
                }
                table th {
                    background-color: #f8f8f8;
                }
                .footer {
                    text-align: center;
                    margin-top: 20px;
                    font-size: 0.9em;
                    color: #555;
                }
                canvas {
                    display: block;
                    margin: 20px auto;
                }
            </style>
        </head>
        <body>
            <div class="container">
        "#,
    );

    report_content.push_str("<h1>Weekly Downloads Report</h1>");
    report_content.push_str(&format!(
        "<p>Report Date: {}</p>",
        Local::now().format("%A, %B %d, %Y")
    ));

    let mut total_size = 0;
    let mut file_counts = HashMap::new();
    let mut file_sizes = HashMap::new();
    let mut unused_count = 0;
    let mut unused_size = 0;

    // Analyze the Downloads folder and its subdirectories
    analyze_folder(downloads_folder, downloads_folder, &mut total_size, &mut file_counts, &mut file_sizes, &mut unused_count, &mut unused_size)?;

    report_content.push_str("<h2>Summary</h2>");
    report_content.push_str("<ul>");
    report_content.push_str(&format!("<li>Total Files: {}</li>", file_counts.values().sum::<usize>()));
    report_content.push_str(&format!("<li>Total Size: {:.2} MB</li>", total_size as f64 / (1024.0 * 1024.0)));
    report_content.push_str(&format!("<li>Unused Files: {}</li>", unused_count));
    report_content.push_str(&format!("<li>Unused Size: {:.2} MB</li>", unused_size as f64 / (1024.0 * 1024.0)));
    report_content.push_str("</ul>");

    report_content.push_str("<h2>File Type Breakdown</h2>");
    report_content.push_str("<table><tr><th>File Type</th><th>Count</th><th>Size (MB)</th></tr>");

    for (ext, count) in &file_counts {
        let size_mb = file_sizes.get(ext).unwrap_or(&0) / (1024 * 1024) as u64;
        report_content.push_str(&format!("<tr><td>{}</td><td>{}</td><td>{}</td></tr>", ext, count, size_mb));
    }

    report_content.push_str("</table>");

    report_content.push_str("<h2>Charts</h2>");
    report_content.push_str(r#"<canvas id="fileTypeChart" width="400" height="400"></canvas>"#);
    report_content.push_str(r#"<canvas id="fileSizeChart" width="400" height="400"></canvas>"#);
    let file_types: Vec<_> = file_counts.keys().collect();
    let file_counts_data: Vec<_> = file_counts.values().collect();
    let file_sizes_data: Vec<_> = file_sizes.values().map(|size| (*size as f64 / (1024.0 * 1024.0))).collect();

    report_content.push_str(&format!(
        r#"
        <script>
            const ctx1 = document.getElementById('fileTypeChart').getContext('2d');
            new Chart(ctx1, {{
                type: 'pie',
                data: {{
                    labels: {:?},
                    datasets: [{{
                        data: {:?},
                        backgroundColor: ['#FF6384', '#36A2EB', '#FFCE56', '#4BC0C0', '#9966FF', '#FF9F40'],
                    }}]
                }},
                options: {{
                    responsive: true,
                    plugins: {{
                        legend: {{
                            position: 'top',
                        }},
                    }}
                }}
            }});

            const ctx2 = document.getElementById('fileSizeChart').getContext('2d');
            new Chart(ctx2, {{
                type: 'bar',
                data: {{
                    labels: {:?},
                    datasets: [{{
                        label: 'File Sizes (MB)',
                        data: {:?},
                        backgroundColor: '#36A2EB',
                    }}]
                }},
                options: {{
                    responsive: true,
                    plugins: {{
                        legend: {{
                            display: false,
                        }},
                    }},
                    scales: {{
                        y: {{
                            beginAtZero: true
                        }}
                    }}
                }}
            }});
        </script>
        "#,
        file_types, file_counts_data, file_types, file_sizes_data
    ));

    report_content.push_str(
        r#"
            <p class="footer">Generated by the Downloads Manager Tool</p>
            </div>
        </body>
        </html>
        "#,
    );

    fs::write(report_file, report_content)?;
    println!("Weekly report generated at {}", report_file.display());
    Ok(())
}

fn analyze_folder(
    base_folder: &Path,
    folder: &Path,
    total_size: &mut u64,
    file_counts: &mut HashMap<String, usize>,
    file_sizes: &mut HashMap<String, u64>,
    unused_count: &mut usize,
    unused_size: &mut u64,
) -> Result<(), std::io::Error> {
    for entry in fs::read_dir(folder)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            analyze_folder(base_folder, &path, total_size, file_counts, file_sizes, unused_count, unused_size)?;
        } else if path.is_file() {
            let metadata = fs::metadata(&path)?;
            let size = metadata.len();
            *total_size += size;

            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("unknown").to_lowercase();
            *file_counts.entry(ext.clone()).or_insert(0) += 1;
            *file_sizes.entry(ext).or_insert(0) += size;

            if path.starts_with(base_folder.join("Unused")) {
                *unused_count += 1;
                *unused_size += size;
            }
        }
    }
    Ok(())
}
