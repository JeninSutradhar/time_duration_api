/// Module for handling time and durations
pub mod time_duration {
    // Importing necessary types
    use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone, Utc};
    use std::time::{Duration, SystemTime};

    /// Represents a specific point in time
    ///
    /// This struct wraps a SystemTime instance to provide additional functionality
    pub struct Time {
        /// Timestamp representing the time
        pub timestamp: SystemTime,
    }

    impl Time {
        /// Returns a new Time instance representing the current time
        pub fn now() -> Self {
            Time {
                timestamp: SystemTime::now(),
            }
        }

        /// Formats the time as a string based on the provided format
        ///
        /// # Arguments
        ///
        /// * `format` - The format string according to the chrono crate
        ///
        /// # Returns
        ///
        /// A string representation of the time
        pub fn format(&self, format: &str) -> String {
            match self.timestamp.duration_since(SystemTime::UNIX_EPOCH) {
                Ok(duration) => {
                    DateTime::<Utc>::from_naive_utc_and_offset(
                        NaiveDateTime::from_timestamp_opt(duration.as_secs() as i64, duration.subsec_nanos()).unwrap(),
                        Utc,
                    )
                    .format(format)
                    .to_string()
                }
                Err(_) => "Invalid time".to_string(),
            }
        }

        /// Formats the time as a string based on the provided format and timezone
        ///
        /// # Arguments
        ///
        /// * `format` - The format string according to the chrono crate
        /// * `timezone` - The timezone string according to the chrono crate
        ///
        /// # Returns
        ///
        /// A string representation of the time in the specified timezone
        pub fn format_with_timezone(&self, format: &str, timezone: &str) -> Result<String, String> {
            match self.timestamp.duration_since(SystemTime::UNIX_EPOCH) {
                Ok(duration) => {
                    let datetime = DateTime::<Utc>::from_naive_utc_and_offset(
                        NaiveDateTime::from_timestamp_opt(duration.as_secs() as i64, duration.subsec_nanos()).unwrap(),
                        Utc,
                    );

                    match timezone.parse::<FixedOffset>() {
                        Ok(tz) => Ok(datetime.with_timezone(&tz).format(format).to_string()),
                        Err(_) => Err("Invalid timezone format".to_string()),
                    }
                }
                Err(_) => Err("Invalid time".to_string()),
            }
        }

        /// Returns the timestamp as a UNIX timestamp
        ///
        /// # Returns
        ///
        /// A Result containing the UNIX timestamp, or an error string
        pub fn timestamp(&self) -> Result<u64, String> {
            match self.timestamp.duration_since(SystemTime::UNIX_EPOCH) {
                Ok(duration) => Ok(duration.as_secs()),
                Err(_) => Err("Invalid time".to_string()),
            }
        }

        /// Returns a new Time instance representing a time after a duration
        ///
        /// # Arguments
        ///
        /// * `duration` - The duration to add to the time
        ///
        /// # Returns
        ///
        /// A new Time instance representing the time after the duration
        pub fn add_duration(&self, duration: &CustomDuration) -> Time {
            Time {
                timestamp: self.timestamp + duration.duration,
            }
        }

        /// Returns a new Time instance representing a time before a duration
        ///
        /// # Arguments
        ///
        /// * `duration` - The duration to subtract from the time
        ///
        /// # Returns
        ///
        /// A new Time instance representing the time before the duration
        pub fn sub_duration(&self, duration: &CustomDuration) -> Time {
            Time {
                timestamp: self.timestamp - duration.duration,
            }
        }

        /// Converts the time to a different timezone
        ///
        /// # Arguments
        ///
        /// * `timezone` - The timezone string according to the chrono crate
        ///
        /// # Returns
        ///
        /// A string representation of the time in the specified timezone
        pub fn to_timezone(&self, timezone: &str) -> Result<String, String> {
            match self.timestamp.duration_since(SystemTime::UNIX_EPOCH) {
                Ok(duration) => {
                    let datetime = DateTime::<Utc>::from_naive_utc_and_offset(
                        NaiveDateTime::from_timestamp_opt(duration.as_secs() as i64, duration.subsec_nanos()).unwrap(),
                        Utc,
                    );

                    match timezone.parse::<FixedOffset>() {
                        Ok(tz) => Ok(datetime.with_timezone(&tz).to_string()),
                        Err(_) => Err("Invalid timezone format".to_string()),
                    }
                }
                Err(_) => Err("Invalid time".to_string()),
            }
        }

        /// Parses a Time instance from a string and format
        ///
        /// # Arguments
        ///
        /// * `time_str` - The time string to parse
        /// * `format` - The format string according to the chrono crate
        ///
        /// # Returns
        ///
        /// A Result containing the parsed Time instance, or an error string
        pub fn from_str(time_str: &str, format: &str) -> Result<Self, String> {
            match DateTime::parse_from_str(time_str, format) {
                Ok(dt) => Ok(Time {
                    timestamp: SystemTime::from(dt),
                }),
                Err(_) => Err("Invalid time format".to_string()),
            }
        }
    }

    /// Represents a duration of time
    ///
    /// This struct wraps a Duration instance to provide additional functionality
    pub struct CustomDuration {
        /// Duration value
        pub duration: Duration,
    }

    impl CustomDuration {
        /// Creates a new CustomDuration from seconds
        ///
        /// # Arguments
        ///
        /// * `secs` - The duration in seconds
        ///
        /// # Returns
        ///
        /// A new CustomDuration instance
        pub fn from_secs(secs: u64) -> Self {
            CustomDuration {
                duration: Duration::from_secs(secs),
            }
        }

        /// Creates a new CustomDuration from milliseconds
        ///
        /// # Arguments
        ///
        /// * `millis` - The duration in milliseconds
        ///
        /// # Returns
        ///
        /// A new CustomDuration instance
        pub fn from_millis(millis: u64) -> Self {
            CustomDuration {
                duration: Duration::from_millis(millis),
            }
        }

        /// Creates a new CustomDuration from microseconds
        ///
        /// # Arguments
        ///
        /// * `micros` - The duration in microseconds
        ///
        /// # Returns
        ///
        /// A new CustomDuration instance
        pub fn from_micros(micros: u64) -> Self {
            CustomDuration {
                duration: Duration::from_micros(micros),
            }
        }

        /// Creates a new CustomDuration from nanoseconds
        ///
        /// # Arguments
        ///
        /// * `nanos` - The duration in nanoseconds
        ///
        /// # Returns
        ///
        /// A new CustomDuration instance
        pub fn from_nanos(nanos: u64) -> Self {
            CustomDuration {
                duration: Duration::from_nanos(nanos),
            }
        }

        /// Creates a new CustomDuration from a string and format
        ///
        /// # Arguments
        ///
        /// * `duration_str` - The duration string to parse
        ///
        /// # Returns
        ///
        /// A Result containing the parsed CustomDuration instance, or an error string
        pub fn from_str(duration_str: &str, format: &str) -> Result<Self, String> {
            match humantime::parse_duration(duration_str) {
                Ok(dur) => Ok(CustomDuration { duration: dur }),
                Err(_) => Err("Invalid duration format".to_string()),
            }
        }

        /// Addition operation for durations
        pub fn add(&self, other: &CustomDuration) -> CustomDuration {
            CustomDuration {
                duration: self.duration + other.duration,
            }
        }

        /// Subtraction operation for durations
        pub fn sub(&self, other: &CustomDuration) -> CustomDuration {
            CustomDuration {
                duration: self.duration - other.duration,
            }
        }

        /// Multiplication operation for durations
        pub fn mul(&self, scalar: u32) -> CustomDuration {
            CustomDuration {
                duration: self.duration * scalar,
            }
        }

        /// Division operation for durations
        pub fn div(&self, divisor: u32) -> CustomDuration {
            CustomDuration {
                duration: self.duration / divisor,
            }
        }

        /// Rounds the duration to the nearest second
        pub fn round_secs(&self) -> CustomDuration {
            CustomDuration {
                duration: Duration::from_secs(self.duration.as_secs()),
            }
        }

        /// Returns the duration as seconds
        pub fn as_secs(&self) -> u64 {
            self.duration.as_secs()
        }

        /// Returns the duration as milliseconds
        pub fn as_millis(&self) -> u128 {
            self.duration.as_millis()
        }

        /// Returns the duration as microseconds
        pub fn as_micros(&self) -> u128 {
            self.duration.as_micros()
        }

        /// Returns the duration as nanoseconds
        pub fn as_nanos(&self) -> u128 {
            self.duration.as_nanos()
        }

        /// Formats the duration in a human-readable format
        pub fn format_human_readable(&self) -> String {
            humantime::format_duration(self.duration).to_string()
        }
    }

    // Implementing standard traits for CustomDuration

    use std::ops::{Add, Sub, Mul, Div};

    impl Add for CustomDuration {
        type Output = CustomDuration;

        fn add(self, other: CustomDuration) -> CustomDuration {
            CustomDuration {
                duration: self.duration + other.duration,
            }
        }
    }

    impl Sub for CustomDuration {
        type Output = CustomDuration;

        fn sub(self, other: CustomDuration) -> CustomDuration {
            CustomDuration {
                duration: self.duration - other.duration,
            }
        }
    }

    impl Mul<u32> for CustomDuration {
        type Output = CustomDuration;

        fn mul(self, scalar: u32) -> CustomDuration {
            CustomDuration {
                duration: self.duration * scalar,
            }
        }
    }

    impl Div<u32> for CustomDuration {
        type Output = CustomDuration;

        fn div(self, divisor: u32) -> CustomDuration {
            CustomDuration {
                duration: self.duration / divisor,
            }
        }
    }

    // Implementing comparison operators for CustomDuration

    use std::cmp::{PartialEq, PartialOrd, Ordering};

    impl PartialEq for CustomDuration {
        fn eq(&self, other: &Self) -> bool {
            self.duration == other.duration
        }
    }

    impl PartialOrd for CustomDuration {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.duration.partial_cmp(&other.duration)
        }
    }
}

