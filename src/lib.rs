/// Module for handling time and durations
pub mod time_duration {
    use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
    use std::time::{Duration, SystemTime};

    /// Represents a specific point in time
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
        pub fn format_with_timezone(&self, format: &str, timezone: &str) -> String {
            match self.timestamp.duration_since(SystemTime::UNIX_EPOCH) {
                Ok(duration) => {
                    let datetime = DateTime::<Utc>::from_naive_utc_and_offset(
                        NaiveDateTime::from_timestamp_opt(duration.as_secs() as i64, duration.subsec_nanos()).unwrap(),
                        Utc,
                    );

                    match FixedOffset::east(timezone.parse().unwrap()) {
                        tz => datetime.with_timezone(&tz).format(format).to_string(),
                    }
                }
                Err(_) => "Invalid time".to_string(),
            }
        }

        /// Returns the timestamp as a UNIX timestamp
        pub fn timestamp(&self) -> u64 {
            match self.timestamp.duration_since(SystemTime::UNIX_EPOCH) {
                Ok(duration) => duration.as_secs(),
                Err(_) => 0,
            }
        }

        /// Returns a new Time instance representing a time after a duration
        pub fn add_duration(&self, duration: &CustomDuration) -> Time {
            Time {
                timestamp: self.timestamp + duration.duration,
            }
        }

        /// Returns a new Time instance representing a time before a duration
        pub fn sub_duration(&self, duration: &CustomDuration) -> Time {
            Time {
                timestamp: self.timestamp - duration.duration,
            }
        }
    }

    /// Represents a duration of time
    pub struct CustomDuration {
        /// Duration value
        pub duration: Duration,
    }

    impl CustomDuration {
        /// Creates a new CustomDuration from seconds
        pub fn from_secs(secs: u64) -> Self {
            CustomDuration {
                duration: Duration::from_secs(secs),
            }
        }

        /// Creates a new CustomDuration from milliseconds
        pub fn from_millis(millis: u64) -> Self {
            CustomDuration {
                duration: Duration::from_millis(millis),
            }
        }

        /// Creates a new CustomDuration from microseconds
        pub fn from_micros(micros: u64) -> Self {
            CustomDuration {
                duration: Duration::from_micros(micros),
            }
        }

        /// Creates a new CustomDuration from nanoseconds
        pub fn from_nanos(nanos: u64) -> Self {
            CustomDuration {
                duration: Duration::from_nanos(nanos),
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
                duration: self.duration * scalar, // Adjusting the type
            }
        }

        /// Division operation for durations
        pub fn div(&self, divisor: u32) -> CustomDuration {
            CustomDuration {
                duration: self.duration / divisor, // Adjusting the type
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
    }
}


