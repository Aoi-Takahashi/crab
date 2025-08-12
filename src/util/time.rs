use chrono::{DateTime, Local};

pub fn format_timestamp_local(timestamp: u64) -> String {
    match DateTime::from_timestamp(timestamp as i64, 0) {
        Some(utc_datetime) => {
            let local_datetime: DateTime<Local> = utc_datetime.with_timezone(&Local);
            local_datetime.format("%Y-%m-%d %H:%M:%S").to_string()
        }
        None => format!("Invalid timestamp: {timestamp}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_timestamp_local() {
        let timestamp = 1704067200;
        let formatted = format_timestamp_local(timestamp);

        assert!(formatted.contains("2024"));
        assert!(formatted.len() > 10);
    }
}
