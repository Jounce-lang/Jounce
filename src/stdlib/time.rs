/// Standard library time and date handling module
/// Provides date, time, and duration types and operations

pub const TIME_DEFINITION: &str = r#"
// Duration represents a span of time
struct Duration {
    milliseconds: i64,
}

impl Duration {
    // Create a duration from milliseconds
    fn from_millis(ms: i64) -> Duration {
        return Duration {
            milliseconds: ms,
        };
    }

    // Create a duration from seconds
    fn from_seconds(secs: i64) -> Duration {
        return Duration {
            milliseconds: secs * 1000,
        };
    }

    // Create a duration from minutes
    fn from_minutes(mins: i64) -> Duration {
        return Duration {
            milliseconds: mins * 60 * 1000,
        };
    }

    // Create a duration from hours
    fn from_hours(hours: i64) -> Duration {
        return Duration {
            milliseconds: hours * 60 * 60 * 1000,
        };
    }

    // Create a duration from days
    fn from_days(days: i64) -> Duration {
        return Duration {
            milliseconds: days * 24 * 60 * 60 * 1000,
        };
    }

    // Get duration as milliseconds
    fn as_millis(self: &Duration) -> i64 {
        return self.milliseconds;
    }

    // Get duration as seconds
    fn as_seconds(self: &Duration) -> i64 {
        return self.milliseconds / 1000;
    }

    // Get duration as minutes
    fn as_minutes(self: &Duration) -> i64 {
        return self.milliseconds / (60 * 1000);
    }

    // Get duration as hours
    fn as_hours(self: &Duration) -> i64 {
        return self.milliseconds / (60 * 60 * 1000);
    }

    // Get duration as days
    fn as_days(self: &Duration) -> i64 {
        return self.milliseconds / (24 * 60 * 60 * 1000);
    }

    // Add two durations
    fn add(self: &Duration, other: &Duration) -> Duration {
        return Duration {
            milliseconds: self.milliseconds + other.milliseconds,
        };
    }

    // Subtract two durations
    fn sub(self: &Duration, other: &Duration) -> Duration {
        return Duration {
            milliseconds: self.milliseconds - other.milliseconds,
        };
    }

    // Multiply duration by a scalar
    fn mul(self: &Duration, factor: i64) -> Duration {
        return Duration {
            milliseconds: self.milliseconds * factor,
        };
    }

    // Divide duration by a scalar
    fn div(self: &Duration, divisor: i64) -> Duration {
        return Duration {
            milliseconds: self.milliseconds / divisor,
        };
    }

    // Compare durations
    fn eq(self: &Duration, other: &Duration) -> bool {
        return self.milliseconds == other.milliseconds;
    }

    fn lt(self: &Duration, other: &Duration) -> bool {
        return self.milliseconds < other.milliseconds;
    }

    fn gt(self: &Duration, other: &Duration) -> bool {
        return self.milliseconds > other.milliseconds;
    }
}

// DateTime represents a specific point in time
struct DateTime {
    timestamp: i64,  // Unix timestamp in milliseconds
}

impl DateTime {
    // Get current date and time
    fn now() -> DateTime {
        // In JavaScript: Date.now()
        // Returns Unix timestamp in milliseconds
        let current_time = 0;  // Will be replaced with Date.now() when compiled to JS

        // For JS compilation, this will become: new Date().getTime()
        // @js: new Date().getTime()

        return DateTime {
            timestamp: current_time,
        };
    }

    // Create from Unix timestamp (milliseconds)
    fn from_timestamp(ms: i64) -> DateTime {
        return DateTime {
            timestamp: ms,
        };
    }

    // Create from Unix timestamp (seconds)
    fn from_unix_seconds(secs: i64) -> DateTime {
        return DateTime {
            timestamp: secs * 1000,
        };
    }

    // Create from date components
    fn from_components(year: i32, month: i32, day: i32, hour: i32, minute: i32, second: i32) -> DateTime {
        // In JavaScript: new Date(year, month - 1, day, hour, minute, second).getTime()
        // @js: new Date(year, month - 1, day, hour, minute, second).getTime()

        // Simplified timestamp calculation (approximate)
        // Days since epoch (Jan 1, 1970)
        let years_diff = year - 1970;
        let days_from_years = years_diff * 365;

        // Add leap years (rough approximation)
        let leap_years = years_diff / 4;
        let total_days = days_from_years + leap_years;

        // Add days from months (approximate)
        let days_in_months = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
        let days_from_months = days_in_months[month - 1];

        // Total days
        let total_days_with_month = total_days + days_from_months + day - 1;

        // Calculate timestamp
        let timestamp = (total_days_with_month as i64) * 24 * 60 * 60 * 1000;
        let timestamp_with_hours = timestamp + ((hour as i64) * 60 * 60 * 1000);
        let timestamp_with_minutes = timestamp_with_hours + ((minute as i64) * 60 * 1000);
        let final_timestamp = timestamp_with_minutes + ((second as i64) * 1000);

        return DateTime {
            timestamp: final_timestamp,
        };
    }

    // Parse from ISO 8601 string
    fn parse(date_str: String) -> Result<DateTime, String> {
        // In JavaScript: Date.parse(dateStr)
        // Returns timestamp in milliseconds or NaN

        // @js: const ts = Date.parse(date_str); if (isNaN(ts)) { return Err("Invalid date"); } return Ok(new DateTime(ts));

        // Simplified parsing - just return current time for now
        // Real implementation would parse ISO 8601 format
        return Ok(DateTime { timestamp: 0 });
    }

    // Get timestamp in milliseconds
    fn timestamp_millis(self: &DateTime) -> i64 {
        return self.timestamp;
    }

    // Get timestamp in seconds
    fn timestamp_seconds(self: &DateTime) -> i64 {
        return self.timestamp / 1000;
    }

    // Get year
    fn year(self: &DateTime) -> i32 {
        // In JavaScript: new Date(timestamp).getFullYear()
        // @js: new Date(this.timestamp).getFullYear()

        // Simplified calculation (will be replaced by JS Date API)
        // Epoch year is 1970
        let years_since_epoch = self.timestamp / (365 * 24 * 60 * 60 * 1000);
        return 1970 + (years_since_epoch as i32);
    }

    // Get month (1-12)
    fn month(self: &DateTime) -> i32 {
        // In JavaScript: new Date(timestamp).getMonth() + 1
        // @js: new Date(this.timestamp).getMonth() + 1

        // Will be replaced by JS Date API
        return 1;
    }

    // Get day of month (1-31)
    fn day(self: &DateTime) -> i32 {
        // In JavaScript: new Date(timestamp).getDate()
        // @js: new Date(this.timestamp).getDate()

        // Will be replaced by JS Date API
        return 1;
    }

    // Get day of week (0 = Sunday, 6 = Saturday)
    fn day_of_week(self: &DateTime) -> i32 {
        // In JavaScript: new Date(timestamp).getDay()
        // @js: new Date(this.timestamp).getDay()

        // Will be replaced by JS Date API
        return 0;
    }

    // Get hour (0-23)
    fn hour(self: &DateTime) -> i32 {
        // In JavaScript: new Date(timestamp).getHours()
        // @js: new Date(this.timestamp).getHours()

        // Simplified calculation
        let hours_since_epoch = self.timestamp / (60 * 60 * 1000);
        return (hours_since_epoch % 24) as i32;
    }

    // Get minute (0-59)
    fn minute(self: &DateTime) -> i32 {
        // In JavaScript: new Date(timestamp).getMinutes()
        // @js: new Date(this.timestamp).getMinutes()

        // Simplified calculation
        let minutes_since_epoch = self.timestamp / (60 * 1000);
        return (minutes_since_epoch % 60) as i32;
    }

    // Get second (0-59)
    fn second(self: &DateTime) -> i32 {
        // In JavaScript: new Date(timestamp).getSeconds()
        // @js: new Date(this.timestamp).getSeconds()

        // Simplified calculation
        let seconds_since_epoch = self.timestamp / 1000;
        return (seconds_since_epoch % 60) as i32;
    }

    // Get millisecond (0-999)
    fn millisecond(self: &DateTime) -> i32 {
        return (self.timestamp % 1000) as i32;
    }

    // Add duration to datetime
    fn add_duration(self: &DateTime, duration: &Duration) -> DateTime {
        return DateTime {
            timestamp: self.timestamp + duration.milliseconds,
        };
    }

    // Subtract duration from datetime
    fn sub_duration(self: &DateTime, duration: &Duration) -> DateTime {
        return DateTime {
            timestamp: self.timestamp - duration.milliseconds,
        };
    }

    // Calculate duration between two datetimes
    fn duration_since(self: &DateTime, other: &DateTime) -> Duration {
        return Duration {
            milliseconds: self.timestamp - other.timestamp,
        };
    }

    // Format datetime as ISO 8601 string
    fn to_iso_string(self: &DateTime) -> String {
        // In JavaScript: new Date(timestamp).toISOString()
        // @js: new Date(this.timestamp).toISOString()

        // Manual formatting as fallback
        let year = self.year();
        let month = self.month();
        let day = self.day();
        let hour = self.hour();
        let minute = self.minute();
        let second = self.second();
        let ms = self.millisecond();

        // Format: "YYYY-MM-DDTHH:MM:SS.sssZ"
        let result = "";
        result = result + year.to_string();
        result = result + "-";
        result = result + self.pad_zero(month, 2);
        result = result + "-";
        result = result + self.pad_zero(day, 2);
        result = result + "T";
        result = result + self.pad_zero(hour, 2);
        result = result + ":";
        result = result + self.pad_zero(minute, 2);
        result = result + ":";
        result = result + self.pad_zero(second, 2);
        result = result + ".";
        result = result + self.pad_zero(ms, 3);
        result = result + "Z";

        return result;
    }

    // Helper to pad number with zeros
    fn pad_zero(self: &DateTime, num: i32, width: i32) -> String {
        let s = num.to_string();
        let mut result = s;

        while result.len() < width {
            result = "0" + result;
        }

        return result;
    }

    // Format datetime with custom format string
    fn format(self: &DateTime, format_str: String) -> String {
        // Support format codes:
        // %Y - 4-digit year
        // %m - 2-digit month
        // %d - 2-digit day
        // %H - 2-digit hour (24h)
        // %M - 2-digit minute
        // %S - 2-digit second

        let mut result = format_str;

        // Replace format codes
        result = result.replace("%Y", self.year().to_string());
        result = result.replace("%m", self.pad_zero(self.month(), 2));
        result = result.replace("%d", self.pad_zero(self.day(), 2));
        result = result.replace("%H", self.pad_zero(self.hour(), 2));
        result = result.replace("%M", self.pad_zero(self.minute(), 2));
        result = result.replace("%S", self.pad_zero(self.second(), 2));

        return result;
    }

    // Format as RFC 2822 (email format)
    fn to_rfc2822(self: &DateTime) -> String {
        // In JavaScript: new Date(timestamp).toUTCString()
        // @js: new Date(this.timestamp).toUTCString()

        // Format: "Mon, 15 Jan 2024 14:30:00 +0000"
        let days = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
        let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

        let day_name = days[self.day_of_week()];
        let month_name = months[self.month() - 1];

        let result = "";
        result = result + day_name + ", ";
        result = result + self.day().to_string() + " ";
        result = result + month_name + " ";
        result = result + self.year().to_string() + " ";
        result = result + self.pad_zero(self.hour(), 2) + ":";
        result = result + self.pad_zero(self.minute(), 2) + ":";
        result = result + self.pad_zero(self.second(), 2);
        result = result + " +0000";

        return result;
    }

    // Compare datetimes
    fn eq(self: &DateTime, other: &DateTime) -> bool {
        return self.timestamp == other.timestamp;
    }

    fn lt(self: &DateTime, other: &DateTime) -> bool {
        return self.timestamp < other.timestamp;
    }

    fn gt(self: &DateTime, other: &DateTime) -> bool {
        return self.timestamp > other.timestamp;
    }

    // Check if datetime is before another
    fn is_before(self: &DateTime, other: &DateTime) -> bool {
        return self.timestamp < other.timestamp;
    }

    // Check if datetime is after another
    fn is_after(self: &DateTime, other: &DateTime) -> bool {
        return self.timestamp > other.timestamp;
    }
}

// Time zone information
enum TimeZone {
    UTC,
    Local,
    Offset(i32),  // Offset in minutes
}

// Date with timezone
struct ZonedDateTime {
    datetime: DateTime,
    timezone: TimeZone,
}

impl ZonedDateTime {
    // Create from datetime and timezone
    fn new(datetime: DateTime, timezone: TimeZone) -> ZonedDateTime {
        return ZonedDateTime {
            datetime: datetime,
            timezone: timezone,
        };
    }

    // Get current time in UTC
    fn now_utc() -> ZonedDateTime {
        return ZonedDateTime {
            datetime: DateTime::now(),
            timezone: TimeZone::UTC,
        };
    }

    // Get current time in local timezone
    fn now_local() -> ZonedDateTime {
        return ZonedDateTime {
            datetime: DateTime::now(),
            timezone: TimeZone::Local,
        };
    }

    // Convert to UTC
    fn to_utc(self: &ZonedDateTime) -> ZonedDateTime {
        // Would apply timezone conversion
        return ZonedDateTime {
            datetime: self.datetime,
            timezone: TimeZone::UTC,
        };
    }

    // Convert to local timezone
    fn to_local(self: &ZonedDateTime) -> ZonedDateTime {
        // Would apply timezone conversion
        return ZonedDateTime {
            datetime: self.datetime,
            timezone: TimeZone::Local,
        };
    }

    // Get the underlying DateTime
    fn datetime(self: &ZonedDateTime) -> DateTime {
        return self.datetime;
    }
}

// Timer for measuring elapsed time
struct Timer {
    start_time: i64,
}

impl Timer {
    // Start a new timer
    fn start() -> Timer {
        let now = DateTime::now();
        return Timer {
            start_time: now.timestamp,
        };
    }

    // Get elapsed time since timer started
    fn elapsed(self: &Timer) -> Duration {
        let now = DateTime::now();
        return Duration {
            milliseconds: now.timestamp - self.start_time,
        };
    }

    // Reset the timer
    fn reset(self: &mut Timer) {
        let now = DateTime::now();
        self.start_time = now.timestamp;
    }
}

// Stopwatch for lap timing
struct Stopwatch {
    start_time: i64,
    laps: Vec<i64>,
    running: bool,
}

impl Stopwatch {
    // Create a new stopwatch
    fn new() -> Stopwatch {
        return Stopwatch {
            start_time: 0,
            laps: Vec::new(),
            running: false,
        };
    }

    // Start the stopwatch
    fn start(self: &mut Stopwatch) {
        let now = DateTime::now();
        self.start_time = now.timestamp;
        self.running = true;
    }

    // Stop the stopwatch
    fn stop(self: &mut Stopwatch) -> Duration {
        self.running = false;
        let elapsed = self.elapsed();
        return elapsed;
    }

    // Record a lap time
    fn lap(self: &mut Stopwatch) -> Duration {
        let now = DateTime::now();
        let lap_time = now.timestamp - self.start_time;
        self.laps.push(lap_time);
        return Duration::from_millis(lap_time);
    }

    // Get elapsed time
    fn elapsed(self: &Stopwatch) -> Duration {
        let now = DateTime::now();
        return Duration::from_millis(now.timestamp - self.start_time);
    }

    // Reset the stopwatch
    fn reset(self: &mut Stopwatch) {
        self.start_time = 0;
        self.laps.clear();
        self.running = false;
    }

    // Get all lap times
    fn get_laps(self: &Stopwatch) -> Vec<Duration> {
        let result = Vec::new();
        for lap_ms in self.laps {
            result.push(Duration::from_millis(lap_ms));
        }
        return result;
    }
}

// Public convenience functions

// Get current time
fn now() -> DateTime {
    return DateTime::now();
}

// Get current UTC time
fn now_utc() -> ZonedDateTime {
    return ZonedDateTime::now_utc();
}

// Get current local time
fn now_local() -> ZonedDateTime {
    return ZonedDateTime::now_local();
}

// Sleep for a duration (async operation)
async fn sleep(duration: Duration) {
    // In JavaScript: await new Promise(resolve => setTimeout(resolve, duration.milliseconds))
    // @js: await new Promise(resolve => setTimeout(resolve, duration.milliseconds))

    // This is an async operation that suspends execution
    // Will be implemented by JavaScript setTimeout when compiled
}

// Create a timer
fn timer() -> Timer {
    return Timer::start();
}

// Create a stopwatch
fn stopwatch() -> Stopwatch {
    return Stopwatch::new();
}

// Common duration helpers
fn seconds(n: i64) -> Duration {
    return Duration::from_seconds(n);
}

fn minutes(n: i64) -> Duration {
    return Duration::from_minutes(n);
}

fn hours(n: i64) -> Duration {
    return Duration::from_hours(n);
}

fn days(n: i64) -> Duration {
    return Duration::from_days(n);
}

// Parse duration from string
fn parse_duration(s: String) -> Result<Duration, String> {
    // Parse strings like "5s", "2m", "1h", "3d", "500ms"

    let trimmed = s.trim();
    if trimmed.is_empty() {
        return Err("Empty duration string");
    }

    // Extract number and unit
    let mut number_str = "";
    let mut unit = "";
    let mut found_unit = false;

    for i in 0..trimmed.len() {
        let ch = trimmed.substring(i, i + 1);

        if !found_unit && (ch >= "0" && ch <= "9" || ch == ".") {
            number_str = number_str + ch;
        } else {
            found_unit = true;
            unit = unit + ch;
        }
    }

    // Parse the number
    let num = number_str.parse_float();
    if num < 0.0 {
        return Err("Invalid duration number");
    }

    // Convert based on unit
    if unit == "ms" {
        return Ok(Duration::from_millis(num as i64));
    } else if unit == "s" {
        return Ok(Duration::from_seconds(num as i64));
    } else if unit == "m" || unit == "min" {
        return Ok(Duration::from_minutes(num as i64));
    } else if unit == "h" || unit == "hr" {
        return Ok(Duration::from_hours(num as i64));
    } else if unit == "d" || unit == "day" {
        return Ok(Duration::from_days(num as i64));
    } else {
        return Err("Unknown duration unit");
    }
}

// Parse datetime from string
fn parse_datetime(s: String) -> Result<DateTime, String> {
    return DateTime::parse(s);
}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_definition_exists() {
        assert!(!TIME_DEFINITION.is_empty());
    }

    #[test]
    fn test_time_definition_contains_duration() {
        assert!(TIME_DEFINITION.contains("struct Duration"));
        assert!(TIME_DEFINITION.contains("fn from_seconds("));
        assert!(TIME_DEFINITION.contains("fn from_minutes("));
        assert!(TIME_DEFINITION.contains("fn from_hours("));
        assert!(TIME_DEFINITION.contains("fn from_days("));
    }

    #[test]
    fn test_time_definition_contains_datetime() {
        assert!(TIME_DEFINITION.contains("struct DateTime"));
        assert!(TIME_DEFINITION.contains("fn now()"));
        assert!(TIME_DEFINITION.contains("fn from_timestamp("));
        assert!(TIME_DEFINITION.contains("fn year("));
        assert!(TIME_DEFINITION.contains("fn month("));
        assert!(TIME_DEFINITION.contains("fn day("));
    }

    #[test]
    fn test_time_definition_contains_timezone() {
        assert!(TIME_DEFINITION.contains("enum TimeZone"));
        assert!(TIME_DEFINITION.contains("struct ZonedDateTime"));
        assert!(TIME_DEFINITION.contains("fn now_utc()"));
        assert!(TIME_DEFINITION.contains("fn now_local()"));
    }

    #[test]
    fn test_time_definition_contains_timer() {
        assert!(TIME_DEFINITION.contains("struct Timer"));
        assert!(TIME_DEFINITION.contains("struct Stopwatch"));
        assert!(TIME_DEFINITION.contains("fn start()"));
        assert!(TIME_DEFINITION.contains("fn elapsed("));
        assert!(TIME_DEFINITION.contains("fn lap("));
    }

    #[test]
    fn test_time_definition_contains_helpers() {
        assert!(TIME_DEFINITION.contains("fn seconds("));
        assert!(TIME_DEFINITION.contains("fn minutes("));
        assert!(TIME_DEFINITION.contains("fn hours("));
        assert!(TIME_DEFINITION.contains("fn days("));
        assert!(TIME_DEFINITION.contains("fn sleep("));
    }

    #[test]
    fn test_time_definition_contains_formatting() {
        assert!(TIME_DEFINITION.contains("fn to_iso_string("));
        assert!(TIME_DEFINITION.contains("fn format("));
        assert!(TIME_DEFINITION.contains("fn to_rfc2822("));
    }

    #[test]
    fn test_time_definition_contains_parsing() {
        assert!(TIME_DEFINITION.contains("fn parse("));
        assert!(TIME_DEFINITION.contains("fn parse_duration("));
        assert!(TIME_DEFINITION.contains("fn parse_datetime("));
    }
}
