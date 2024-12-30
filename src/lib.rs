pub mod time_utils {
    use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
    use serde::{Deserialize, Serialize};
    use std::{
        cmp::{Ordering, PartialOrd},
        fmt,
        ops::{Add, Div, Mul, Sub},
        str::FromStr,
        time::{Duration, SystemTime},
    };

    // Custom Error Type
    #[derive(Debug, Clone)]
    pub enum TimeError {
        InvalidTime,
        InvalidTimeFormat(String),
        InvalidTimezoneFormat(String),
        ParseError(String), // Generic parsing error
    }

    impl std::error::Error for TimeError {}

    impl fmt::Display for TimeError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                TimeError::InvalidTime => write!(f, "Invalid time"),
                TimeError::InvalidTimeFormat(msg) => write!(f, "Invalid time format: {}", msg),
                TimeError::InvalidTimezoneFormat(msg) => {
                    write!(f, "Invalid timezone format: {}", msg)
                }
                TimeError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            }
        }
    }

    // Custom Result Type
    pub type Result<T> = std::result::Result<T, TimeError>;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Time {
        timestamp: SystemTime,
        #[serde(skip)]
        cached_utc_datetime: Option<DateTime<Utc>>, // Cache the Utc DateTime
    }

    impl Time {
        /// Creates a new Time instance with the current system time.
        ///
        /// # Example
        ///
        /// ```
        /// use time_lib::time_utils::Time;
        /// let now = Time::now();
        /// println!("Current time: {}", now.format("%Y-%m-%d %H:%M:%S").unwrap());
        /// ```
        pub fn now() -> Self {
            Time {
                timestamp: SystemTime::now(),
                cached_utc_datetime: None,
            }
        }

        /// Formats the time with the given format string.
        ///
        /// Returns a formatted time string or an error if time is invalid.
        ///
        /// # Example
        ///
        /// ```
        /// use time_lib::time_utils::Time;
        /// let time = Time::now();
        /// let formatted_time = time.format("%Y-%m-%d %H:%M:%S").unwrap();
        /// println!("Formatted time: {}", formatted_time);
        /// ```
        pub fn format(&mut self, format: &str) -> Result<String> {
            let datetime = self.get_utc_datetime()?; // Use the cached or generate DateTime
            Ok(datetime.format(format).to_string())
        }

        // Helper function to get cached or generate DateTime<Utc>
        fn get_utc_datetime(&mut self) -> Result<DateTime<Utc>> {
            if let Some(cached) = self.cached_utc_datetime {
                return Ok(cached);
            }

            let duration = self
                .timestamp
                .duration_since(SystemTime::UNIX_EPOCH)
                .map_err(|_| TimeError::InvalidTime)?;

            let datetime = DateTime::<Utc>::from_naive_utc_and_offset(
                NaiveDateTime::from_timestamp_opt(
                    duration.as_secs() as i64,
                    duration.subsec_nanos(),
                )
                .ok_or(TimeError::InvalidTime)?,
                Utc,
            );

            self.cached_utc_datetime = Some(datetime); // Cache the DateTime
            Ok(datetime)
        }
        /// Formats the time with a given format string and timezone.
        ///
        /// Returns a formatted time string or an error if time or timezone is invalid.
        ///
        /// # Example
        ///
        /// ```
        /// use time_lib::time_utils::Time;
        /// let time = Time::now();
        /// let formatted_time = time.format_with_timezone("%Y-%m-%d %H:%M:%S", "+05:30").unwrap();
        /// println!("Formatted time in IST: {}", formatted_time);
        /// ```
        pub fn format_with_timezone(&mut self, format: &str, timezone: &str) -> Result<String> {
            let datetime = self.get_utc_datetime()?;
            let tz: FixedOffset = timezone
                .parse()
                .map_err(|_| TimeError::InvalidTimezoneFormat(timezone.to_string()))?;
            Ok(datetime.with_timezone(&tz).format(format).to_string())
        }

        /// Gets the timestamp in seconds.
        ///
        /// Returns the timestamp as a u64 or an error if time is invalid.
        ///
        /// # Example
        ///
        /// ```
        /// use time_lib::time_utils::Time;
        /// let time = Time::now();
        /// let timestamp = time.timestamp().unwrap();
        /// println!("Timestamp: {}", timestamp);
        /// ```
        pub fn timestamp(&self) -> Result<u64> {
            self.timestamp
                .duration_since(SystemTime::UNIX_EPOCH)
                .map(|duration| duration.as_secs())
                .map_err(|_| TimeError::InvalidTime)
        }

        /// Adds a custom duration to the current time.
        ///
        /// # Example
        ///
        /// ```
        /// use time_lib::time_utils::{Time, CustomDuration};
        /// let time = Time::now();
        /// let duration = CustomDuration::from_secs(3600); // 1 hour
        /// let future_time = time.add_duration(&duration);
        /// println!("Future time: {}", future_time.format("%Y-%m-%d %H:%M:%S").unwrap());
        /// ```
        pub fn add_duration(&self, duration: &CustomDuration) -> Self {
            Time {
                timestamp: self.timestamp + duration.duration,
                cached_utc_datetime: None,
            }
        }

        /// Subtracts a custom duration from the current time.
        ///
        /// # Example
        ///
        /// ```
        /// use time_lib::time_utils::{Time, CustomDuration};
        /// let time = Time::now();
        /// let duration = CustomDuration::from_secs(3600); // 1 hour
        /// let past_time = time.sub_duration(&duration);
        /// println!("Past time: {}", past_time.format("%Y-%m-%d %H:%M:%S").unwrap());
        /// ```
        pub fn sub_duration(&self, duration: &CustomDuration) -> Self {
            Time {
                timestamp: self.timestamp - duration.duration,
                cached_utc_datetime: None,
            }
        }

        /// Converts the time to a specific timezone.
        ///
        /// Returns the time string in the new timezone or an error.
        ///
        /// # Example
        ///
        /// ```
        /// use time_lib::time_utils::Time;
        /// let time = Time::now();
        /// let ist_time = time.to_timezone("+05:30").unwrap();
        /// println!("Time in IST: {}", ist_time);
        /// ```
        pub fn to_timezone(&mut self, timezone: &str) -> Result<String> {
            let datetime = self.get_utc_datetime()?;
            let tz: FixedOffset = timezone
                .parse()
                .map_err(|_| TimeError::InvalidTimezoneFormat(timezone.to_string()))?;
            Ok(datetime.with_timezone(&tz).to_string())
        }

        /// Creates a Time instance from a formatted time string.
        ///
        /// Returns the time or an error if the format is invalid.
        ///
        /// # Example
        ///
        /// ```
        /// use time_lib::time_utils::Time;
        /// let time = Time::from_str("2023-09-20 10:30:00", "%Y-%m-%d %H:%M:%S").unwrap();
        /// println!("Parsed time: {}", time.format("%Y-%m-%d %H:%M:%S").unwrap());
        /// ```
        pub fn from_str(time_str: &str, format: &str) -> Result<Self> {
             match DateTime::parse_from_str(time_str, format) {
                Ok(dt) => Ok(Time {
                    timestamp: SystemTime::from(dt),
                    cached_utc_datetime: None,
                }),
                Err(e) => Err(TimeError::InvalidTimeFormat(format!(
                    "Failed to parse '{}' with format '{}': {}",
                    time_str, format, e
                ))),
            }
        }
    }

    impl fmt::Display for Time {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self.timestamp.duration_since(SystemTime::UNIX_EPOCH) {
                Ok(duration) => {
                    let datetime = DateTime::<Utc>::from_naive_utc_and_offset(
                        NaiveDateTime::from_timestamp_opt(
                            duration.as_secs() as i64,
                            duration.subsec_nanos(),
                        )
                        .unwrap(),
                        Utc,
                    );
                    write!(f, "{}", datetime.format("%Y-%m-%d %H:%M:%S"))
                }
                Err(_) => write!(f, "Invalid Time"),
            }
        }
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub struct CustomDuration {
        duration: Duration,
    }

    impl CustomDuration {
        /// Creates a CustomDuration from a number of seconds.
        ///
        /// # Example
        ///
        /// ```
        /// use time_lib::time_utils::CustomDuration;
        /// let duration = CustomDuration::from_secs(60);
        /// println!("Duration: {}", duration.format_human_readable());
        /// ```
        pub fn from_secs(secs: u64) -> Self {
            CustomDuration {
                duration: Duration::from_secs(secs),
            }
        }

        /// Creates a CustomDuration from a number of milliseconds.
        ///
        /// # Example
        ///
        /// ```
        /// use time_lib::time_utils::CustomDuration;
        /// let duration = CustomDuration::from_millis(1000);
        /// println!("Duration: {}", duration.format_human_readable());
        /// ```
        pub fn from_millis(millis: u64) -> Self {
            CustomDuration {
                duration: Duration::from_millis(millis),
            }
        }

        /// Creates a CustomDuration from a number of microseconds.
        ///
        /// # Example
        ///
        /// ```
        /// use time_lib::time_utils::CustomDuration;
        /// let duration = CustomDuration::from_micros(1000000);
        /// println!("Duration: {}", duration.format_human_readable());
        /// ```
        pub fn from_micros(micros: u64) -> Self {
            CustomDuration {
                duration: Duration::from_micros(micros),
            }
        }

        /// Creates a CustomDuration from a number of nanoseconds.
        ///
        /// # Example
        ///
        /// ```
        /// use time_lib::time_utils::CustomDuration;
        /// let duration = CustomDuration::from_nanos(1000000000);
        /// println!("Duration: {}", duration.format_human_readable());
        /// ```
        pub fn from_nanos(nanos: u64) -> Self {
            CustomDuration {
                duration: Duration::from_nanos(nanos),
            }
        }

        /// Creates a CustomDuration from a human-readable string (e.g., "1h 30m").
        ///
        /// Returns the duration or an error if the string is invalid.
        ///
        /// # Example
        ///
        /// ```
        /// use time_lib::time_utils::CustomDuration;
        /// let duration = CustomDuration::from_str("2h 30m").unwrap();
        /// println!("Duration: {}", duration.format_human_readable());
        /// ```
        pub fn from_str(duration_str: &str) -> Result<Self> {
            humantime::parse_duration(duration_str)
                .map(|dur| CustomDuration { duration: dur })
                .map_err(|e| TimeError::ParseError(e.to_string()))
        }

        /// Adds two CustomDuration instances.
        ///
        /// # Example
        ///
        /// ```
        /// use time_lib::time_utils::CustomDuration;
        /// let dur1 = CustomDuration::from_secs(60);
        /// let dur2 = CustomDuration::from_secs(120);
        /// let sum = dur1.add(&dur2);
        /// println!("Sum: {}", sum.format_human_readable());
        /// ```
        pub fn add(&self, other: &CustomDuration) -> CustomDuration {
            CustomDuration {
                duration: self.duration + other.duration,
            }
        }

        /// Subtracts one CustomDuration from another.
        ///
        /// # Example
        ///
        /// ```
        /// use time_lib::time_utils::CustomDuration;
        /// let dur1 = CustomDuration::from_secs(180);
        /// let dur2 = CustomDuration::from_secs(60);
        /// let diff = dur1.sub(&dur2);
        /// println!("Difference: {}", diff.format_human_readable());
        /// ```
        pub fn sub(&self, other: &CustomDuration) -> CustomDuration {
            CustomDuration {
                duration: self.duration - other.duration,
            }
        }

        /// Multiplies a CustomDuration by a scalar value.
        ///
        /// # Example
        ///
        /// ```
        /// use time_lib::time_utils::CustomDuration;
        /// let duration = CustomDuration::from_secs(60);
        /// let multiplied = duration.mul(3);
        /// println!("Multiplied duration: {}", multiplied.format_human_readable());
        /// ```
        pub fn mul(&self, scalar: u32) -> CustomDuration {
            CustomDuration {
                duration: self.duration * scalar,
            }
        }

        /// Divides a CustomDuration by a scalar value.
        ///
        /// # Example
        ///
        /// ```
        /// use time_lib::time_utils::CustomDuration;
        /// let duration = CustomDuration::from_secs(120);
        /// let divided = duration.div(2);
        /// println!("Divided duration: {}", divided.format_human_readable());
        /// ```
        pub fn div(&self, divisor: u32) -> CustomDuration {
            CustomDuration {
                duration: self.duration / divisor,
            }
        }
        /// Rounds the duration to the nearest second.
        ///
        /// # Example
        ///
        /// ```
        /// use time_lib::time_utils::CustomDuration;
        /// let duration = CustomDuration::from_millis(1500);
        /// let rounded = duration.round_secs();
        /// println!("Rounded duration: {}", rounded.format_human_readable());
        /// ```
        pub fn round_secs(&self) -> CustomDuration {
            CustomDuration {
                duration: Duration::from_secs(self.duration.as_secs()),
            }
        }

        /// Returns the duration as a number of seconds.
        ///
        /// # Example
        ///
        /// ```
        /// use time_lib::time_utils::CustomDuration;
        /// let duration = CustomDuration::from_secs(60);
        /// println!("Seconds: {}", duration.as_secs());
        /// ```
        pub fn as_secs(&self) -> u64 {
            self.duration.as_secs()
        }

        /// Returns the duration as a number of milliseconds.
        ///
        /// # Example
        ///
        /// ```
        /// use time_lib::time_utils::CustomDuration;
        /// let duration = CustomDuration::from_millis(1000);
        /// println!("Milliseconds: {}", duration.as_millis());
        /// ```
        pub fn as_millis(&self) -> u128 {
            self.duration.as_millis()
        }

        /// Returns the duration as a number of microseconds.
        ///
        /// # Example
        ///
        /// ```
        /// use time_lib::time_utils::CustomDuration;
        /// let duration = CustomDuration::from_micros(1000000);
        /// println!("Microseconds: {}", duration.as_micros());
        /// ```
        pub fn as_micros(&self) -> u128 {
            self.duration.as_micros()
        }

        /// Returns the duration as a number of nanoseconds.
        ///
        /// # Example
        ///
        /// ```
        /// use time_lib::time_utils::CustomDuration;
        /// let duration = CustomDuration::from_nanos(1000000000);
        /// println!("Nanoseconds: {}", duration.as_nanos());
        /// ```
        pub fn as_nanos(&self) -> u128 {
            self.duration.as_nanos()
        }

        /// Formats the duration into a human-readable string.
        ///
        /// # Example
        ///
        /// ```
        /// use time_lib::time_utils::CustomDuration;
        /// let duration = CustomDuration::from_secs(3661);
        /// println!("Human readable: {}", duration.format_human_readable());
        /// ```
        pub fn format_human_readable(&self) -> String {
            humantime::format_duration(self.duration).to_string()
        }
    }

    impl Add for CustomDuration {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            CustomDuration {
                duration: self.duration + other.duration,
            }
        }
    }

    impl Sub for CustomDuration {
        type Output = Self;

        fn sub(self, other: Self) -> Self {
            CustomDuration {
                duration: self.duration - other.duration,
            }
        }
    }

    impl Mul<u32> for CustomDuration {
        type Output = Self;

        fn mul(self, scalar: u32) -> Self {
            CustomDuration {
                duration: self.duration * scalar,
            }
        }
    }

    impl Div<u32> for CustomDuration {
        type Output = Self;

        fn div(self, divisor: u32) -> Self {
            CustomDuration {
                duration: self.duration / divisor,
            }
        }
    }

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

    impl fmt::Display for CustomDuration {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", humantime::format_duration(self.duration))
        }
    }

     impl FromStr for Time {
        type Err = TimeError;
    
        fn from_str(s: &str) -> Result<Self> {
            // Define formats with and without timezone
           let formats_with_tz = [
                "%Y-%m-%d %H:%M:%S%z",
                "%Y-%m-%dT%H:%M:%S%z",
                "%Y-%m-%d %H:%M:%S.%f%z",
                "%Y-%m-%dT%H:%M:%S.%f%z",
            ];
             let _ = [
                "%Y-%m-%d %H:%M:%S",
                "%Y-%m-%dT%H:%M:%S",
                "%Y-%m-%d %H:%M:%S.%f",
                "%Y-%m-%dT%H:%M:%S.%f",
            ];
    
             for format in formats_with_tz {
                 if let Ok(dt) = DateTime::parse_from_str(s, format) {
                    return Ok(Time {
                        timestamp: SystemTime::from(dt),
                        cached_utc_datetime: None,
                    });
                }
           }
              // Attempt to parse with common formats without offset,
            if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
               return  Ok(Time {
                   timestamp: SystemTime::from(dt),
                    cached_utc_datetime: None,
                });
            }

            
             for format in  formats_with_tz {
                match DateTime::parse_from_str(s, format) {
                    Ok(dt) => {
                      return Ok(Time {
                         timestamp: SystemTime::from(dt),
                         cached_utc_datetime: None,
                       });
                     }
                Err(_)=> {}
           }
    
        }
    
       Err(TimeError::ParseError(format!("Invalid time string: {}",s)))

    }
    
    }
}