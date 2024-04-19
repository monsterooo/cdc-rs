pub fn format_bytes(bytes: u64) -> String {
    const SUFFIXES: [&str; 4] = ["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut index = 0;

    while size >= 1024.0 && index < SUFFIXES.len() - 1 {
        size /= 1024.0;
        index += 1;
    }

    format!("{:.2} {}", size, SUFFIXES[index])
}