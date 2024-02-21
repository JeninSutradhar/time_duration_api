//! This crate provides a time/duration API for Rust.
//!
//! # jeninsutradhar@gmail.com
//!
//! # Examples
//!
//! ```
//! use time_duration_api::time_duration::{Time, CustomDuration};
//!
//! // Create a new Time instance representing the current time
//! let current_time = Time::now();
//!
//! // Format the time as a string
//! let formatted_time = current_time.format("%Y-%m-%d %H:%M:%S");
//!
//! println!("Current time: {}", formatted_time);
//!
//! // Create a custom duration of 5 seconds
//! let duration = CustomDuration::from_secs(5);
//!
//! println!("Duration: {:?}", duration.duration);
//! ```

/// Module for handling time and durations
pub mod time_duration {
    use chrono::NaiveDateTime;
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
                // Current time
            }
        }

        /// Formats the time as a string based on the provided format
        pub fn format(&self, format: &str) -> String {
            match self.timestamp.duration_since(SystemTime::UNIX_EPOCH) {
                Ok(duration) => {
                    let datetime_opt = NaiveDateTime::from_timestamp_opt(
                    
                        duration.as_secs() as i64,
                    
                        duration.subsec_nanos(),
                    );


                    match datetime_opt {
                        Some(datetime) => datetime.format(format).to_string(),
                        None => "Invalid time".to_string(),
                    }
                }
                Err(_) => "Invalid time".to_string(),
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
    }
}
