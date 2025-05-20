use lazy_static::lazy_static;
use std::collections::HashMap;

/// A map of timestamp format names to their corresponding Rust regex patterns.
/// This allows for looking up regex patterns by name when creating regex objects.
lazy_static! {
    pub static ref TIMESTAMP_FORMATS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();

        // ISO 8601 Formats
        map.insert("ISO_DATE", r"^\d{4}-\d{2}-\d{2}$");
        map.insert("ISO_DATETIME", r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}$");
        map.insert("ISO_DATETIME_UTC", r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z$");
        map.insert("ISO_DATETIME_TZ", r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}[+-]\d{2}:\d{2}$");
        map.insert("ISO_DATETIME_MS", r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{1,9}$");
        map.insert("ISO_DATETIME_MS_UTC", r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{1,9}Z$");
        map.insert("ISO_DATE_BASIC", r"^\d{8}$");
        map.insert("ISO_DATETIME_BASIC", r"^\d{8}T\d{6}$");
        map.insert("ISO_ORDINAL_DATE", r"^\d{4}-\d{3}$");
        map.insert("ISO_WEEK_DATE", r"^\d{4}-W\d{2}-\d{1}$");

        // Unix/Epoch Timestamps
        map.insert("UNIX_SECONDS", r"^[1-9]\d{9}$");
        map.insert("UNIX_MILLISECONDS", r"^[1-9]\d{12}$");
        map.insert("UNIX_MICROSECONDS", r"^[1-9]\d{15}$");
        map.insert("UNIX_NANOSECONDS", r"^[1-9]\d{18}$");

        // RFC Standards
        map.insert("RFC_822_1123", r"^[A-Z][a-z]{2}, \d{1,2} [A-Z][a-z]{2} \d{4} \d{2}:\d{2}:\d{2} GMT$");
        map.insert("RFC_850_1036", r"^[A-Z][a-z]+, \d{1,2}-[A-Z][a-z]{2}-\d{2} \d{2}:\d{2}:\d{2} GMT$");
        map.insert("ANSI_C_ASCTIME", r"^[A-Z][a-z]{2} [A-Z][a-z]{2} \d{1,2} \d{2}:\d{2}:\d{2} \d{4}$");
        map.insert("RFC_3339", r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}[+-]\d{2}:\d{2}$");

        // Regional and Localized Formats
        map.insert("US_DATETIME", r"^\d{1,2}/\d{1,2}/\d{4} \d{2}:\d{2}:\d{2}$");
        map.insert("EU_DATETIME", r"^\d{1,2}/\d{1,2}/\d{4} \d{2}:\d{2}:\d{2}$");
        map.insert("ASIAN_DATETIME", r"^\d{4}/\d{1,2}/\d{1,2} \d{2}:\d{2}:\d{2}$");
        map.insert("GERMAN_DATETIME", r"^\d{1,2}\.\d{1,2}\.\d{4} \d{2}:\d{2}:\d{2}$");
        map.insert("UK_DATETIME", r"^\d{1,2}-[A-Z][a-z]{2}-\d{4} \d{2}:\d{2}:\d{2}$");
        map.insert("SHORT_EU_DATETIME", r"^\d{1,2}-\d{1,2}-\d{2} \d{2}:\d{2}:\d{2}$");
        map.insert("SHORT_US_DATETIME", r"^\d{1,2}-\d{1,2}-\d{2} \d{2}:\d{2}:\d{2}$");
        map.insert("TIME_LEADING_FORMAT", r"^\d{2}:\d{2}:\d{2} \d{1,2}/\d{1,2}/\d{4}$");

        // Time Formats and Variations
        map.insert("TIME_24H", r"^\d{2}:\d{2}:\d{2}$");
        map.insert("TIME_24H_MS", r"^\d{2}:\d{2}:\d{2}\.\d{1,3}$");
        map.insert("TIME_12H", r"^\d{1,2}:\d{2}:\d{2} [AP]M$");
        map.insert("TIME_12H_SHORT", r"^\d{1,2}:\d{2} [AP]M$");
        map.insert("TIME_CONTINUOUS_MS", r"^\d{6}\d{1,3}$");
        map.insert("TIME_MILITARY", r"^\d{6}$");

        // Database Timestamp Formats
        map.insert("SQL_TIMESTAMP", r"^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}$");
        map.insert("SQL_TIMESTAMP_MS", r"^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}\.\d{1,6}$");
        map.insert("ORACLE_TIMESTAMP", r"^\d{1,2}-[A-Z]{3}-\d{2} \d{1,2}\.\d{2}\.\d{2}\.\d{1,4} [AP]M$");
        map.insert("DB2_TIMESTAMP", r"^\d{4}-\d{2}-\d{2}-\d{2}\.\d{2}\.\d{2}\.\d{1,6}$");
        map.insert("MSSQL_TIMESTAMP", r"^\d{8} \d{2}:\d{2}:\d{2}$");

        // Programming Language/System Specific
        map.insert("COMPACT_TIMESTAMP", r"^\d{14}\d{1,3}$");
        map.insert("POSTGRES_TIMESTAMP_TZ", r"^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}\.\d{1,6}[+-]\d{2}:\d{2}$");
        map.insert("TAGGED_UNIX", r"^@\d{10}$");
        map.insert("SHORT_DATE", r"^\d{2}-\d{2}-\d{2}$");
        map.insert("SAS_DATETIME", r"^\d{1,2}[A-Z]{3}\d{4}:\d{2}:\d{2}:\d{2}$");
        map.insert("DOTNET_DATETIME", r"^\d{1,2}/\d{1,2}/\d{4} \d{1,2}:\d{2}:\d{2} [AP]M$");

        // Legacy and Specialized Formats
        map.insert("HYPHEN_SEPARATED", r"^\d{4}-\d{2}-\d{2}-\d{2}-\d{2}-\d{2}$");
        map.insert("CONTINUOUS_DATETIME", r"^\d{14}\d{1,3}$");
        map.insert("NASA_MISSION", r"^\d{2}\.\d{3}/\d{2}:\d{2}:\d{2}$");
        map.insert("EXIF_DATETIME", r"^\d{4}:\d{2}:\d{2} \d{2}:\d{2}:\d{2}$");
        map.insert("JULIAN_DATE", r"^24\d{4}\.\d{1,5}$");
        map.insert("MODIFIED_JULIAN_DATE", r"^\d{5}\.\d{1,5}$");
        map.insert("ORDINAL_DATE_SHORT", r"^\d{2}\d{3}$");
        map.insert("IBM_MAINFRAME", r"^[0-9]\d{16}$");

        // Industry-Specific Formats
        map.insert("AVIATION_METAR", r"^\d{2}\d{4}Z [A-Z]{3} \d{2}$");
        map.insert("BROADCAST_TIMECODE", r"^\d{1,3}:\d{2}:\d{2}:\d{2}:\d{2}$");
        map.insert("SMPTE_TIMECODE", r"^\d{2}:\d{2}:\d{2}:\d{2}$");
        map.insert("ISO_WEEK", r"^\d{4}-W\d{2}$");
        map.insert("ALT_ISO_WEEK", r"^W\d{2}-\d{4}$");
        map.insert("JULIAN_SHORT", r"^\d{2}\d{3}$");
        map.insert("AVIATION_MIXED", r"^\d{4}UTC[A-Z][a-z]{2}\d{1,2}$");

        // Timezone Representations
        map.insert("ZULU_INDICATOR", r"^.*Z$");
        map.insert("ISO_TZ_OFFSET", r"^.*[+-]\d{2}:\d{2}$");
        map.insert("COMPACT_TZ_OFFSET", r"^.*[+-]\d{4}$");
        map.insert("GMT_OFFSET", r"^.* GMT[+-]\d{2}:\d{2}$");
        map.insert("NAMED_TIMEZONE", r"^.* [A-Z]{3,5}$");
        map.insert("IANA_TIMEZONE", r"^.* [A-Za-z]+/[A-Za-z_]+$");
        map.insert("SIMPLE_UTC_OFFSET", r"^.* UTC[+-]\d{1,2}$");

        // Special Format Considerations
        map.insert("JAVA8_DATETIME", r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{1,3}[+-]\d{2}:\d{2}\[[A-Za-z/]+\]$");
        map.insert("SIGNED_UNIX", r"^[+-]\d{10}$");
        map.insert("HYBRID_TIMESTAMP", r"^@\d{13}/\d{4}-\d{2}-\d{2}$");
        map.insert("W3C_DTF", r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}[+-]\d{2}:\d{2}$");
        map.insert("XML_TIMESTAMP", r"^<\d{10}>$");
        map.insert("ISO_WEEK_WEEKDAY", r"^\d{4}\.\d{1,2}\.\d{1}$");
        map.insert("CUSTOM_EPOCH", r"^\d{10,19}$");
        map.insert("COMPACT_DATETIME", r"^\d{6}-\d{6}$");

        // Calendar-Specific Formats
        map.insert("CHINESE_CALENDAR", r"^[^\x00-\x7F]+$");
        map.insert("ISLAMIC_CALENDAR", r"^1[3-5]\d{2}-\d{2}-\d{2}$");
        map.insert("HEBREW_CALENDAR", r"^5[7-8]\d{2}-\d{2}-\d{2}$");
        map.insert("INDIAN_CALENDAR", r"^\d{4} [A-Z][a-z]+ \d{1,2}$");
        map.insert("THAI_CALENDAR", r"^25\d{2}-\d{2}-\d{2}$");
        map.insert("JAPANESE_CALENDAR", r"^[^\x00-\x7F]+$");

        map
    };
}

/// Example usage to match a timestamp against available patterns
pub fn identify_timestamp_format(timestamp: &str) -> Vec<&'static str> {
    use regex::Regex;

    let mut matches = Vec::new();

    for (name, pattern) in TIMESTAMP_FORMATS.iter() {
        if let Ok(regex) = Regex::new(pattern) {
            if regex.is_match(timestamp) {
                matches.push(*name);
            }
        }
    }

    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identify_iso_date() {
        let timestamp = "2025-05-19";
        let formats = identify_timestamp_format(timestamp);
        assert!(formats.contains(&"ISO_DATE"));
    }

    #[test]
    fn test_identify_unix_timestamp() {
        let timestamp = "1716159600";
        let formats = identify_timestamp_format(timestamp);
        assert!(formats.contains(&"UNIX_SECONDS"));
    }

    #[test]
    fn test_identify_rfc_format() {
        let timestamp = "Mon, 19 May 2025 14:30:15 GMT";
        let formats = identify_timestamp_format(timestamp);
        assert!(formats.contains(&"RFC_822_1123"));
    }
}
