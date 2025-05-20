# Comprehensive Timestamp Format Patterns

This document catalogs the various timestamp format patterns used across different systems, standards, and industries, including Rust-compatible regex patterns for identification.

## Note on Regex Usage in Rust

The regex patterns provided use Rust's regex syntax and should be used with the `regex` crate. Some important considerations:

1. These patterns focus on format identification rather than validation (e.g., they don't check if months are 1-12 or days are valid for the month)
2. In actual implementation, you may need to:
   - Add anchors (`^` and `) to ensure full string matching
   - Use more precise character classes for specific components
   - Combine patterns with additional logic for complete validation

Example Rust code for using these patterns:

```rust
use regex::Regex;

fn identify_timestamp_format(timestamp: &str) -> Option<&str> {
    let patterns = [
        (r"^\d{4}-\d{2}-\d{2}$", "ISO 8601 date"),
        (r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}$", "ISO 8601 datetime"),
        // Add more patterns here
    ];

    for (pattern, format_name) in patterns.iter() {
        if Regex::new(pattern).unwrap().is_match(timestamp) {
            return Some(format_name);
        }
    }

    None
}
```

## ISO 8601 Formats
ISO 8601 is an international standard for date and time representations.

| Name | Pattern | Example | Rust Regex | Known For |
|------|---------|---------|------------|-----------|
| `ISO_DATE` | `YYYY-MM-DD` | `2025-05-19` | `r"^\d{4}-(0[1-9]|1[0-2])-(0[1-9]|[12]\d|3[01])$"` | ISO 8601 date format; widely used for date-only representation; sortable by default |
| `ISO_DATETIME` | `YYYY-MM-DDTHH:MM:SS` | `2025-05-19T14:30:15` | `r"^\d{4}-(0[1-9]|1[0-2])-(0[1-9]|[12]\d|3[01])T([01]\d|2[0-3]):[0-5]\d:[0-5]\d$"` | Basic ISO 8601 combined date and time format; the "T" explicitly separates date and time |
| `ISO_DATETIME_UTC` | `YYYY-MM-DDTHH:MM:SSZ` | `2025-05-19T14:30:15Z` | `r"^\d{4}-(0[1-9]|1[0-2])-(0[1-9]|[12]\d|3[01])T([01]\d|2[0-3]):[0-5]\d:[0-5]\d(?:Z|UTC)$"` | ISO 8601 with "Z" indicating UTC/Zulu time |
| `ISO_DATETIME_TZ` | `YYYY-MM-DDTHH:MM:SS±HH:MM` | `2025-05-19T14:30:15+02:00` | `r"^\d{4}-(0[1-9]|1[0-2])-(0[1-9]|[12]\d|3[01])T([01]\d|2[0-3]):[0-5]\d:[0-5]\d[+-](?:[01]\d|2[0-3]):[0-5]\d$"` | ISO 8601 with explicit timezone offset |
| `ISO_DATETIME_MS` | `YYYY-MM-DDTHH:MM:SS.sss` | `2025-05-19T14:30:15.234` | `r"^\d{4}-(0[1-9]|1[0-2])-(0[1-9]|[12]\d|3[01])T([01]\d|2[0-3]):[0-5]\d:[0-5]\d\.\d{1,9}$"` | ISO 8601 with millisecond precision |
| `ISO_DATETIME_MS_UTC` | `YYYY-MM-DDTHH:MM:SS.sssZ` | `2025-05-19T14:30:15.234Z` | `r"^\d{4}-(0[1-9]|1[0-2])-(0[1-9]|[12]\d|3[01])T([01]\d|2[0-3]):[0-5]\d:[0-5]\d\.\d{1,9}(?:Z|UTC)$"` | ISO 8601 with milliseconds and UTC indicator |
| `ISO_DATE_BASIC` | `YYYYMMDD` | `20250519` | `r"^\d{4}(0[1-9]|1[0-2])(0[1-9]|[12]\d|3[01])$"` | ISO 8601 basic format (without separators); used in filenames and compact data storage |
| `ISO_DATETIME_BASIC` | `YYYYMMDDTHHMMSS` | `20250519T143015` | `r"^\d{4}(0[1-9]|1[0-2])(0[1-9]|[12]\d|3[01])T([01]\d|2[0-3])[0-5]\d[0-5]\d$"` | ISO 8601 basic format for date and time; used where character count matters |
| `ISO_ORDINAL_DATE` | `YYYY-DDD` | `2025-139` | `r"^\d{4}-(00[1-9]|0[1-9]\d|[1-2]\d\d|3[0-5]\d|36[0-6])$"` | ISO 8601 ordinal date format (day of year); used in astronomical and scientific applications |
| `ISO_WEEK_DATE` | `YYYY-Www-D` | `2025-W21-1` | `r"^\d{4}-W(0[1-9]|[1-4]\d|5[0-3])-[1-7]$"` | ISO 8601 week date format (year-week-day); common in some European business contexts |

## Unix/Epoch Timestamps
Based on seconds elapsed since January 1, 1970, 00:00:00 UTC.

| Name | Pattern | Example | Rust Regex | Known For |
|------|---------|---------|------------|-----------|
| `UNIX_SECONDS` | `[seconds]` | `1716159600` | `r"^[1-9]\d{9}$"` | Unix/POSIX timestamp; integer representation for easy mathematical operations; widely used in computing |
| `UNIX_MILLISECONDS` | `[milliseconds]` | `1716159600000` | `r"^[1-9]\d{12}$"` | Unix timestamp with millisecond precision; used by JavaScript, Java, and many modern systems |
| `UNIX_MICROSECONDS` | `[microseconds]` | `1716159600000000` | `r"^[1-9]\d{15}$"` | Unix timestamp with microsecond precision; used in scientific and high-precision applications |
| `UNIX_NANOSECONDS` | `[nanoseconds]` | `1716159600000000000` | `r"^[1-9]\d{18}$"` | Unix timestamp with nanosecond precision; used in high-frequency trading and precision timing systems |

## RFC Standards
Various Request for Comments (RFC) timestamp formats used in internet protocols.

| Name | Pattern | Example | Rust Regex | Known For |
|------|---------|---------|------------|-----------|
| `RFC_822_1123` | `ddd, DD MMM YYYY HH:MM:SS GMT` | `Mon, 19 May 2025 14:30:15 GMT` | `r"^(Mon|Tue|Wed|Thu|Fri|Sat|Sun), (0[1-9]|[12]\d|3[01]) (Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec) \d{4} ([01]\d|2[0-3]):[0-5]\d:[0-5]\d GMT$"` | RFC 822/1123; standard for HTTP headers and email systems |
| `RFC_850_1036` | `dddd, DD-MMM-YY HH:MM:SS GMT` | `Monday, 19-May-25 14:30:15 GMT` | `r"^(Monday|Tuesday|Wednesday|Thursday|Friday|Saturday|Sunday), (0[1-9]|[12]\d|3[01])-(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)-\d{2} ([01]\d|2[0-3]):[0-5]\d:[0-5]\d GMT$"` | RFC 850/1036; older standard used in HTTP and Usenet posts |
| `ANSI_C_ASCTIME` | `ddd MMM DD HH:MM:SS YYYY` | `Mon May 19 14:30:15 2025` | `r"^(Mon|Tue|Wed|Thu|Fri|Sat|Sun) (Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec) ([ 1-9]|[12]\d|3[01]) ([01]\d|2[0-3]):[0-5]\d:[0-5]\d \d{4}$"` | ANSI C asctime() format; used in Unix logs |
| `RFC_3339` | `YYYY-MM-DDTHH:MM:SS+HH:MM` | `2025-05-19T14:30:15+02:00` | `r"^\d{4}-(0[1-9]|1[0-2])-(0[1-9]|[12]\d|3[01])T([01]\d|2[0-3]):[0-5]\d:[0-5]\d(?:[+-](?:[01]\d|2[0-3]):[0-5]\d|Z)$"` | RFC 3339; a profile of ISO 8601 for Internet protocols; used in many web APIs |

## Regional and Localized Formats
Date and time formats that vary by geographical region or locale.

| Name | Pattern | Example | Rust Regex | Known For |
|------|---------|---------|------------|-----------|
| `US_DATETIME` | `MM/DD/YYYY HH:MM:SS` | `05/19/2025 14:30:15` | `r"^(0?[1-9]|1[0-2])/(0?[1-9]|[12]\d|3[01])/\d{4} ([01]\d|2[0-3]):[0-5]\d:[0-5]\d$"` | US standard format; widely used in North America |
| `EU_DATETIME` | `DD/MM/YYYY HH:MM:SS` | `19/05/2025 14:30:15` | `r"^(0?[1-9]|[12]\d|3[01])/(0?[1-9]|1[0-2])/\d{4} ([01]\d|2[0-3]):[0-5]\d:[0-5]\d$"` | European format; common in most of Europe, Australia, and Latin America |
| `ASIAN_DATETIME` | `YYYY/MM/DD HH:MM:SS` | `2025/05/19 14:30:15` | `r"^\d{4}/(0?[1-9]|1[0-2])/(0?[1-9]|[12]\d|3[01]) ([01]\d|2[0-3]):[0-5]\d:[0-5]\d$"` | East Asian format; used in Japan, China, Korea; also sortable |
| `GERMAN_DATETIME` | `DD.MM.YYYY HH:MM:SS` | `19.05.2025 14:30:15` | `r"^(0?[1-9]|[12]\d|3[01])\.(0?[1-9]|1[0-2])\.\d{4} ([01]\d|2[0-3]):[0-5]\d:[0-5]\d$"` | Format used in many European countries, particularly Germany, Russia, and Eastern Europe |
| `UK_DATETIME` | `DD-MMM-YYYY HH:MM:SS` | `19-May-2025 14:30:15` | `r"^(0?[1-9]|[12]\d|3[01])-(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)-\d{4} ([01]\d|2[0-3]):[0-5]\d:[0-5]\d$"` | Common in the UK and some Commonwealth countries; month as abbreviation increases readability |
| `SHORT_EU_DATETIME` | `DD-MM-YY HH:MM:SS` | `19-05-25 14:30:15` | `r"^(0?[1-9]|[12]\d|3[01])-(0?[1-9]|1[0-2])-\d{2} ([01]\d|2[0-3]):[0-5]\d:[0-5]\d$"` | Short date format with 2-digit year; saves space but can cause Y2K-style ambiguity |
| `SHORT_US_DATETIME` | `MM-DD-YY HH:MM:SS` | `05-19-25 14:30:15` | `r"^(0?[1-9]|1[0-2])-(0?[1-9]|[12]\d|3[01])-\d{2} ([01]\d|2[0-3]):[0-5]\d:[0-5]\d$"` | US short date format; same ambiguity issues as above |
| `TIME_LEADING_FORMAT` | `HH:MM:SS DD/MM/YYYY` | `14:30:15 19/05/2025` | `r"^([01]\d|2[0-3]):[0-5]\d:[0-5]\d (0?[1-9]|[12]\d|3[01])/(0?[1-9]|1[0-2])/\d{4}$"` | Time-leading format; occasionally used in log files and certain applications |

## Time Formats and Variations

| Name | Pattern | Example | Rust Regex | Known For |
|------|---------|---------|------------|-----------|
| `TIME_24H` | `HH:MM:SS` | `14:30:15` | `r"^([01]\d|2[0-3]):[0-5]\d:[0-5]\d$"` | 24-hour time format; standard in most of the world; avoids AM/PM ambiguity |
| `TIME_24H_MS` | `HH:MM:SS.sss` | `14:30:15.234` | `r"^([01]\d|2[0-3]):[0-5]\d:[0-5]\d\.\d{1,3}$"` | Time with millisecond precision; used in scientific and technical applications |
| `TIME_12H` | `hh:MM:SS AM/PM` | `02:30:15 PM` | `r"^(0?[1-9]|1[0-2]):[0-5]\d:[0-5]\d [AP]M$"` | 12-hour time format with AM/PM; common in the US and some Commonwealth countries |
| `TIME_12H_SHORT` | `hh:MM AM/PM` | `02:30 PM` | `r"^(0?[1-9]|1[0-2]):[0-5]\d [AP]M$"` | 12-hour time format without seconds; used in everyday contexts where second precision isn't needed |
| `TIME_CONTINUOUS_MS` | `HHMMSSsss` | `143015234` | `r"^([01]\d|2[0-3])[0-5]\d[0-5]\d\d{3}$"` | Continuous time format without separators; used in some telecommunications and data logging systems |
| `TIME_MILITARY` | `HHMMSS` | `143015` | `r"^([01]\d|2[0-3])[0-5]\d[0-5]\d$"` | Military/24-hour time without separators; used in aviation, emergency services, and military contexts |

## Database Timestamp Formats
Formats specifically used in database systems.

| Name | Pattern | Example | Rust Regex | Known For |
|------|---------|---------|------------|-----------|
| `SQL_TIMESTAMP` | `YYYY-MM-DD HH:MM:SS` | `2025-05-19 14:30:15` | `r"^\d{4}-(0[1-9]|1[0-2])-(0[1-9]|[12]\d|3[01]) ([01]\d|2[0-3]):[0-5]\d:[0-5]\d$"` | Standard SQL timestamp format; used in MySQL, PostgreSQL, and many other RDBMS |
| `SQL_TIMESTAMP_MS` | `YYYY-MM-DD HH:MM:SS.mmmmmm` | `2025-05-19 14:30:15.123456` | `r"^\d{4}-(0[1-9]|1[0-2])-(0[1-9]|[12]\d|3[01]) ([01]\d|2[0-3]):[0-5]\d:[0-5]\d\.\d{1,6}$"` | SQL timestamp with microsecond precision; used in modern database systems |
| `ORACLE_TIMESTAMP` | `DD-MON-YY HH.MM.SS.mmmm AM` | `19-MAY-25 02.30.15.1234 PM` | `r"^(0?[1-9]|[12]\d|3[01])-(JAN|FEB|MAR|APR|MAY|JUN|JUL|AUG|SEP|OCT|NOV|DEC)-\d{2} (0?[1-9]|1[0-2])\.(0[0-9]|[1-5]\d)\.(0[0-9]|[1-5]\d)\.\d{1,4} [AP]M$"` | Oracle Database timestamp format; note period separators for time |
| `DB2_TIMESTAMP` | `YYYY-MM-DD-HH.MM.SS.mmmmmm` | `2025-05-19-14.30.15.123456` | `r"^\d{4}-(0[1-9]|1[0-2])-(0[1-9]|[12]\d|3[01])-([01]\d|2[0-3])\.(0[0-9]|[1-5]\d)\.(0[0-9]|[1-5]\d)\.\d{1,6}$"` | IBM DB2 timestamp format; uses hyphens for date-time separation and periods for time units |
| `MSSQL_TIMESTAMP` | `YYYYMMDD HH:MM:SS` | `20250519 14:30:15` | `r"^\d{4}(0[1-9]|1[0-2])(0[1-9]|[12]\d|3[01]) ([01]\d|2[0-3]):[0-5]\d:[0-5]\d$"` | Microsoft SQL Server format option; combines basic date format with standard time format |

## Programming Language/System Specific

| Name | Pattern | Example | Rust Regex | Known For |
|------|---------|---------|------------|-----------|
| `COMPACT_TIMESTAMP` | `YYYYMMDDHHMMSSmmm` | `20250519143015234` | `r"^\d{4}(0[1-9]|1[0-2])(0[1-9]|[12]\d|3[01])([01]\d|2[0-3])[0-5]\d[0-5]\d\d{1,3}$"` | Compact numerical representation; used in some programming contexts where separators are unnecessary |
| `POSTGRES_TIMESTAMP_TZ` | `YYYY-MM-DD HH:MM:SS.nnnnnn±TZ` | `2025-05-19 14:30:15.123456-04:00` | `r"^\d{4}-(0[1-9]|1[0-2])-(0[1-9]|[12]\d|3[01]) ([01]\d|2[0-3]):[0-5]\d:[0-5]\d\.\d{1,6}[+-]([01]\d|2[0-3]):[0-5]\d$"` | PostgreSQL timestamp with timezone format; highly precise with explicit timezone |
| `TAGGED_UNIX` | `@[seconds]` | `@1716159600` | `r"^@[1-9]\d{9}$"` | Format used in some programming contexts to explicitly mark a Unix timestamp |
| `SHORT_DATE` | `YY-MM-DD` | `25-05-19` | `r"^\d{2}-(0[1-9]|1[0-2])-(0[1-9]|[12]\d|3[01])$"` | Short date format; used in legacy systems but discouraged due to century ambiguity |
| `SAS_DATETIME` | `DDMMMYYYY:HH:MM:SS` | `19MAY2025:14:30:15` | `r"^(0?[1-9]|[12]\d|3[01])(JAN|FEB|MAR|APR|MAY|JUN|JUL|AUG|SEP|OCT|NOV|DEC)\d{4}:([01]\d|2[0-3]):[0-5]\d:[0-5]\d$"` | SAS datetime format; used in statistical analysis |
| `DOTNET_DATETIME` | `M/D/YYYY h:MM:SS A` | `5/19/2025 2:30:15 PM` | `r"^(0?[1-9]|1[0-2])/(0?[1-9]|[12]\d|3[01])/\d{4} (0?[1-9]|1[0-2]):[0-5]\d:[0-5]\d [AP]M$"` | .NET/C# default short DateTime format (US culture) |

## Legacy and Specialized Formats

| Name | Pattern | Example | Rust Regex | Known For |
|------|---------|---------|------------|-----------|
| `HYPHEN_SEPARATED` | `YYYY-MM-DD-HH-MM-SS` | `2025-05-19-14-30-15` | `r"^\d{4}-(0[1-9]|1[0-2])-(0[1-9]|[12]\d|3[01])-([01]\d|2[0-3])-[0-5]\d-[0-5]\d$"` | Hyphen-separated format; used in some file naming conventions |
| `CONTINUOUS_DATETIME` | `YYYYMMDDHHMMSSmmm` | `20250519143015234` | `r"^\d{4}(0[1-9]|1[0-2])(0[1-9]|[12]\d|3[01])([01]\d|2[0-3])[0-5]\d[0-5]\d$"` | Continuous digit format; used in log files and systems where compactness is important |
| `NASA_MISSION` | `YY.DDD/HH:MM:SS` | `25.139/14:30:15` | `r"^\d{2}\.(00[1-9]|0[1-9]\d|[1-2]\d\d|3[0-5]\d|36[0-6])/([01]\d|2[0-3]):[0-5]\d:[0-5]\d$"` | NASA mission format; year and day-of-year with time; used in space missions |
| `EXIF_DATETIME` | `YYYY:MM:DD HH:MM:SS` | `2025:05:19 14:30:15` | `r"^\d{4}:(0[1-9]|1[0-2]):(0[1-9]|[12]\d|3[01]) ([01]\d|2[0-3]):[0-5]\d:[0-5]\d$"` | EXIF metadata format; used in digital photography |
| `JULIAN_DATE` | `[Julian Date]` | `2460443.10434` | `r"^24\d{4}\.\d{1,5}$"` | Astronomical Julian date; continuous day count since January 1, 4713 BCE; used in astronomy |
| `MODIFIED_JULIAN_DATE` | `[Modified Julian Date]` | `60442.60434` | `r"^[5-6]\d{4}\.\d{1,5}$"` | Modified Julian Date; days since November 17, 1858; used in space systems and astronomy |
| `ORDINAL_DATE_SHORT` | `YYYDDD` | `25139` | `r"^\d{2}(00[1-9]|0[1-9]\d|[1-2]\d\d|3[0-5]\d|36[0-6])$"` | Ordinal date format without separators; used in some mainframe systems |
| `IBM_MAINFRAME` | `CYYMMDDHHMMSSmmm` | `1250519143015234` | `r"^[1-2]\d{16}$"` | IBM mainframe format with century indicator (1=21st century); used in legacy financial systems |

## Industry-Specific Formats

| Name | Pattern | Example | Rust Regex | Known For |
|------|---------|---------|------------|-----------|
| `AVIATION_METAR` | `DDHHMMZ MMM YY` | `191430Z MAY 25` | `r"^(0[1-9]|[12]\d|3[01])([01]\d|2[0-3])[0-5]\dZ (JAN|FEB|MAR|APR|MAY|JUN|JUL|AUG|SEP|OCT|NOV|DEC) \d{2}$"` | Aviation METAR format; used in weather reports for pilots |
| `BROADCAST_TIMECODE` | `DOY:HH:MM:SS:FF` | `139:14:30:15:24` | `r"^(00[1-9]|0[1-9]\d|[1-2]\d\d|3[0-5]\d|36[0-6]):([01]\d|2[0-3]):[0-5]\d:[0-5]\d:[0-5]\d$"` | Timecode format; day of year, hours, minutes, seconds, frames; used in broadcast media |
| `SMPTE_TIMECODE` | `HH:MM:SS:FF` | `14:30:15:24` | `r"^([01]\d|2[0-3]):[0-5]\d:[0-5]\d:([0-2]\d|3[0-9])$"` | SMPTE timecode; hours, minutes, seconds, frames; standard in film and video production |
| `ISO_WEEK` | `YYYY-Www` | `2025-W21` | `r"^\d{4}-W(0[1-9]|[1-4]\d|5[0-3])$"` | ISO 8601 week format; used in fiscal accounting and some European business contexts |
| `ALT_ISO_WEEK` | `Www-YYYY` | `W21-2025` | `r"^W(0[1-9]|[1-4]\d|5[0-3])-\d{4}$"` | Alternative week format; used in retail and inventory planning |
| `JULIAN_SHORT` | `YYDDD` | `25139` | `r"^\d{2}(00[1-9]|0[1-9]\d|[1-2]\d\d|3[0-5]\d|36[0-6])$"` | Julian date format; used in pharmaceutical and military industries |
| `AVIATION_MIXED` | `HHMMSSdd` | `1430UTCMay19` | `r"^([01]\d|2[0-3])[0-5]\dUTC(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)(0[1-9]|[12]\d|3[01])$"` | Mixed format; used in some aviation and emergency communication |

## Timezone Representations

| Name | Pattern | Example | Rust Regex | Known For |
|------|---------|---------|------------|-----------|
| `ZULU_INDICATOR` | `Z` | `2025-05-19T14:30:15Z` | `r"^\d{4}-(0[1-9]|1[0-2])-(0[1-9]|[12]\d|3[01])T([01]\d|2[0-3]):[0-5]\d:[0-5]\d(?:\.\d{1,9})?Z$"` | Zulu time indicator; denotes UTC; used in aviation, military, and scientific contexts |
| `ISO_TZ_OFFSET` | `±HH:MM` | `2025-05-19T14:30:15+02:00` | `r"^\d{4}-(0[1-9]|1[0-2])-(0[1-9]|[12]\d|3[01])T([01]\d|2[0-3]):[0-5]\d:[0-5]\d(?:\.\d{1,9})?[+-]([01]\d|2[0-3]):[0-5]\d$"` | ISO 8601 timezone offset format; precise time zone representation |
| `COMPACT_TZ_OFFSET` | `±HHMM` | `2025-05-19T14:30:15+0200` | `r"^\d{4}-(0[1-9]|1[0-2])-(0[1-9]|[12]\d|3[01])T([01]\d|2[0-3]):[0-5]\d:[0-5]\d(?:\.\d{1,9})?[+-]([01]\d|2[0-3])[0-5]\d$"` | Compact timezone offset; used where character count matters |
| `GMT_OFFSET` | `GMT±HH:MM` | `2025-05-19 14:30:15 GMT+02:00` | `r"^\d{4}-(0[1-9]|1[0-2])-(0[1-9]|[12]\d|3[01]) ([01]\d|2[0-3]):[0-5]\d:[0-5]\d GMT[+-]([01]\d|2[0-3]):[0-5]\d$"` | Greenwich Mean Time with offset; common in international communications |
| `NAMED_TIMEZONE` | `[Named Zone]` | `2025-05-19 14:30:15 EDT` | `r"^\d{4}-(0[1-9]|1[0-2])-(0[1-9]|[12]\d|3[01]) ([01]\d|2[0-3]):[0-5]\d:[0-5]\d [A-Z]{3,5}$"` | Named timezone abbreviation; problematic due to ambiguity (e.g., EST could be Eastern Standard Time in different countries) |
| `IANA_TIMEZONE` | `[Full Zone Name]` | `2025-05-19 14:30:15 America/New_York` | `r"^\d{4}-(0[1-9]|1[0-2])-(0[1-9]|[12]\d|3[01]) ([01]\d|2[0-3]):[0-5]\d:[0-5]\d [A-Za-z]+/[A-Za-z_]+$"` | IANA Timezone database name; unambiguous timezone representation; used in modern systems |
| `SIMPLE_UTC_OFFSET` | `[UTC Offset]` | `2025-05-19 14:30:15 UTC+2` | `r"^\d{4}-(0[1-9]|1[0-2])-(0[1-9]|[12]\d|3[01]) ([01]\d|2[0-3]):[0-5]\d:[0-5]\d UTC[+-]([01]?\d|2[0-3])$"` | Simplified UTC offset notation; common in informal contexts |

## Special Format Considerations

| Name | Pattern | Example | Rust Regex | Known For |
|------|---------|---------|------------|-----------|
| `JAVA8_DATETIME` | `YYYY-MM-DDThh:mm:ss.sss[TimeZoneID]` | `2025-05-19T14:30:15.123+02:00[Europe/Paris]` | `r"^\d{4}-(0[1-9]|1[0-2])-(0[1-9]|[12]\d|3[01])T([01]\d|2[0-3]):[0-5]\d:[0-5]\d\.\d{1,3}[+-]([01]\d|2[0-3]):[0-5]\d\[[A-Za-z/]+\]$"` | Extended ISO 8601 with IANA timezone; used in Java 8+ and modern systems |
| `SIGNED_UNIX` | `±[seconds]` | `+1716159600` | `r"^[+-][1-9]\d{9}$"` | Signed Unix timestamp; can represent dates before 1970 |
| `HYBRID_TIMESTAMP` | `@[ms]/[YYYY-MM-DD]` | `@1716159600000/2025-05-19` | `r"^@[1-9]\d{12}/\d{4}-(0[1-9]|1[0-2])-(0[1-9]|[12]\d|3[01])$"` | Hybrid format with both timestamp and human-readable date; used in some logging systems |
| `W3C_DTF` | `[W3C DTF]` | `2025-05-19T14:30:15-04:00` | `r"^\d{4}-(0[1-9]|1[0-2])-(0[1-9]|[12]\d|3[01])T([01]\d|2[0-3]):[0-5]\d:[0-5]\d[+-]([01]\d|2[0-3]):[0-5]\d$"` | W3C Date and Time Format; profile of ISO 8601; used in HTML5, XML, and web standards |
| `XML_TIMESTAMP` | `<[seconds]>` | `<1716159600>` | `r"^<[1-9]\d{9}>$"` | XML-style Unix timestamp; used in some data exchange formats |
| `ISO_WEEK_WEEKDAY` | `[ISO Week].[Weekday]` | `2025.21.1` | `r"^\d{4}\.(0[1-9]|[1-4]\d|5[0-3])\.[1-7]$"` | Year, ISO week number, and weekday; used in some European business applications |
| `CUSTOM_EPOCH` | `[Custom Epoch]` | `[varies]` | `r"^(?:[1-9]\d{9}|[1-9]\d{12}|[1-9]\d{15}|[1-9]\d{18})$"` | Non-Unix epochs; e.g., Microsoft .NET (ticks since 01/01/0001), Apple (seconds since 01/01/2001) |
| `COMPACT_DATETIME` | `YYMMDD-HHMMSS` | `250519-143015` | `r"^\d{2}(0[1-9]|1[0-2])(0[1-9]|[12]\d|3[01])-([01]\d|2[0-3])[0-5]\d[0-5]\d$"` | Combined short format; used in some file naming conventions and legacy systems |

## Calendar-Specific Formats

| Name | Pattern | Example | Rust Regex | Known For |
|------|---------|---------|------------|-----------|
| `CHINESE_CALENDAR` | `[Chinese]` | `甲辰年四月二十二日` | `r"^[\u4E00-\u9FFF年月日]+$"` | Traditional Chinese calendar format; used in cultural contexts |
| `ISLAMIC_CALENDAR` | `[Islamic]` | `1447-01-01` | `r"^1[3-5]\d{2}-(0[1-9]|1[0-2])-(0[1-9]|[12]\d|30)$"` | Islamic Hijri calendar; used in Muslim countries for religious and some civil purposes |
| `HEBREW_CALENDAR` | `[Hebrew]` | `5785-09-01` | `r"^5[7-8]\d{2}-(0[1-9]|1[0-2])-(0[1-9]|[12]\d|30)$"` | Hebrew/Jewish calendar; used in Israel and Jewish communities for religious purposes |
| `INDIAN_CALENDAR` | `[Indian]` | `1947 Jyaishtha 15` | `r"^\d{4} (Chaitra|Vaisakha|Jyaishtha|Ashadha|Sravana|Bhadra|Asvina|Kartika|Agrahayana|Pausha|Magha|Phalguna) (0?[1-9]|[12]\d|3[0-1])$"` | Indian national calendar; parallel civil calendar in India |
| `THAI_CALENDAR` | `[Thai]` | `2568-05-19` | `r"^25\d{2}-(0[1-9]|1[0-2])-(0[1-9]|[12]\d|3[01])$"` | Buddhist Era calendar; official calendar in Thailand (543 years ahead of Gregorian) |
| `JAPANESE_CALENDAR` | `[Japanese]` | `令和7年5月19日` | `r"^(令和|平成|昭和|大正|明治)\d{1,2}年(0?[1-9]|1[0-2])月(0?[1-9]|[12]\d|3[01])日$"` | Japanese Imperial calendar; uses era names based on the reigning Emperor |

---

Some regex patterns could match the same string, by analyzing the patterns for potential overlaps, with a deterministic finite automaton (DFA) minimization approach, we can identify overlaps of pattern structure.

However some might be discoverable via simpler means (do they have the same pattern?)

## Identical Regex Patterns

These pairs have identical regex patterns despite representing different format interpretations:

1. **US vs. European Date Formats**
   ```
   US_DATETIME: r"^\d{1,2}/\d{1,2}/\d{4} \d{2}:\d{2}:\d{2}$"
   EU_DATETIME: r"^\d{1,2}/\d{1,2}/\d{4} \d{2}:\d{2}:\d{2}$"
   ```
   These match exactly the same strings (e.g., "05/19/2025 14:30:15") but interpret the numbers differently.

2. **Short Date Formats**
   ```
   SHORT_EU_DATETIME: r"^\d{1,2}-\d{1,2}-\d{2} \d{2}:\d{2}:\d{2}$"
   SHORT_US_DATETIME: r"^\d{1,2}-\d{1,2}-\d{2} \d{2}:\d{2}:\d{2}$"
   ```

3. **ISO 8601 with Timezone**
   ```
   RFC_3339: r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}[+-]\d{2}:\d{2}$"
   ISO_DATETIME_TZ: r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}[+-]\d{2}:\d{2}$"
   W3C_DTF: r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}[+-]\d{2}:\d{2}$"
   ```

4. **Compact Timestamp Formats**
   ```
   COMPACT_TIMESTAMP: r"^\d{14}\d{1,3}$"
   CONTINUOUS_DATETIME: r"^\d{14}\d{1,3}$"
   ```

5. **Julian/Ordinal Date Formats**
   ```
   ORDINAL_DATE_SHORT: r"^\d{2}\d{3}$"
   JULIAN_SHORT: r"^\d{2}\d{3}$"
   ```

6. **Non-ASCII Calendar Formats**
   ```
   CHINESE_CALENDAR: r"^[^\x00-\x7F]+$"
   JAPANESE_CALENDAR: r"^[^\x00-\x7F]+$"
   ```

## Highly Overlapping Patterns

1. **Timezone Indicators**
   All of these use `.*` prefixes, creating enormous potential for overlap:
   ```
   ZULU_INDICATOR: r"^.*Z$"
   ISO_TZ_OFFSET: r"^.*[+-]\d{2}:\d{2}$"
   COMPACT_TZ_OFFSET: r"^.*[+-]\d{4}$"
   NAMED_TIMEZONE: r"^.* [A-Z]{3,5}$"
   ```

2. **Digit Sequences**
   Several patterns match simple digit sequences with different lengths:
   ```
   ISO_DATE_BASIC: r"^\d{8}$"           // 8 digits
   TIME_MILITARY: r"^\d{6}$"            // 6 digits
   UNIX_SECONDS: r"^[1-9]\d{9}$"        // 10 digits starting with non-zero
   CUSTOM_EPOCH: r"^\d{10,19}$"         // 10-19 digits (overlaps with Unix timestamps)
   ```

## Overlapping Hierarchies

Some patterns are strict subsets or extensions of others:

1. **ISO Date Hierarchies**
   ```
   ISO_DATE: r"^\d{4}-\d{2}-\d{2}$"
   ISO_DATETIME: r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}$"
   ISO_DATETIME_UTC: r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z$"
   ISO_DATETIME_MS: r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{1,9}$"
   ```

2. **Time Format Hierarchies**
   ```
   TIME_24H: r"^\d{2}:\d{2}:\d{2}$"
   TIME_24H_MS: r"^\d{2}:\d{2}:\d{2}\.\d{1,3}$"
   ```

DFA minimization would confirm these overlaps mathematically and likely identify additional subtler overlaps. For a production system, you would want to:

1. Prioritize more specific patterns over general ones
2. Implement a disambiguation strategy for identical patterns
3. Consider making patterns more specific where appropriate

Would you like me to suggest improvements to make these patterns more distinctive and reduce ambiguity?
