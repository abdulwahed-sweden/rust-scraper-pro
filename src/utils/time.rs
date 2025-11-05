//! Time utilities for safe timestamp handling across the application

use chrono::{DateTime, Utc};

/// Parse a timestamp string or return current UTC time
pub fn parse_or_now(timestamp_str: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(timestamp_str)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

/// Get current UTC timestamp
pub fn now() -> DateTime<Utc> {
    Utc::now()
}

/// Convert Option<String> timestamp to DateTime<Utc>
pub fn parse_optional_or_now(timestamp: Option<&str>) -> DateTime<Utc> {
    timestamp
        .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|| Utc::now())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_or_now() {
        let valid_timestamp = "2024-01-01T00:00:00Z";
        let result = parse_or_now(valid_timestamp);
        assert_eq!(result.to_rfc3339(), "2024-01-01T00:00:00+00:00");

        let invalid_timestamp = "not a timestamp";
        let result = parse_or_now(invalid_timestamp);
        assert!(result.timestamp() > 0);
    }

    #[test]
    fn test_now() {
        let timestamp = now();
        assert!(timestamp.timestamp() > 0);
    }

    #[test]
    fn test_parse_optional_or_now() {
        let valid = Some("2024-01-01T00:00:00Z");
        let result = parse_optional_or_now(valid);
        assert_eq!(result.to_rfc3339(), "2024-01-01T00:00:00+00:00");

        let result = parse_optional_or_now(None);
        assert!(result.timestamp() > 0);
    }
}
