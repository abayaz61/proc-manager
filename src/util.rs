pub fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if bytes >= TB {
        format!("{:.1} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

pub fn format_bytes_rate(bytes_per_sec: u64) -> String {
    format!("{}/s", format_bytes(bytes_per_sec))
}

pub fn format_duration_short(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;

    if hours > 24 {
        let days = hours / 24;
        let remaining_hours = hours % 24;
        format!("{}d {}h", days, remaining_hours)
    } else if hours > 0 {
        format!("{}h {:02}m", hours, minutes)
    } else {
        format!("{}m {:02}s", minutes, seconds % 60)
    }
}

pub fn format_cpu_percent(percent: f32) -> String {
    if percent >= 100.0 {
        format!("{:.0}%", percent)
    } else if percent >= 10.0 {
        format!("{:.1}%", percent)
    } else {
        format!("{:.2}%", percent)
    }
}
