# time_duration_api 
[![crates.io](https://img.shields.io/crates/v/time_duration_api)](https://crates.io/crates/time_duration_api) [![docs.rs](https://docs.rs/time_duration_api/badge.svg)](https://docs.rs/time_duration_api) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
This crate provides a time and duration manipulation API for Rust projects. It aims to simplify common time-related tasks by offering a set of easy-to-use structs and methods.


## Overview

The `time_duration_api` crate provides utilities to handle time and duration operations in Rust. It includes two key structures:

- `Time`: A struct for working with time, offering methods for getting the current time, formatting, manipulating time, and converting between timezones.
- `CustomDuration`: A struct for representing durations in a human-readable format, with support for arithmetic operations like addition, subtraction, multiplication, and division.

---

## Features

### 1. `Time` Struct

The `Time` struct represents a specific point in time. It provides the following functionalities:

- **Get the current time**.
- **Format the time** into a string using custom formats.
- **Add or subtract durations**.
- **Convert to different timezones**.
- **Convert from string**.

#### Methods

- `now()`: Returns the current time.
- `format(format: &str)`: Formats the time using the provided format string.
- `timestamp()`: Returns the time as a Unix timestamp (seconds since Jan 1, 1970).
- `add_duration(duration: &CustomDuration)`: Adds the specified duration to the current time.
- `sub_duration(duration: &CustomDuration)`: Subtracts the specified duration from the current time.
- `to_timezone(tz: &str)`: Converts the time to the specified timezone.
- `from_str(time_str: &str, format: &str)`: Parses a string into a `Time` object using the provided format.

#### Example Usage

```rust
use time_duration_api::time_utils::{CustomDuration, Time};

fn main() {
    let mut now = Time::now();
    println!("Current Time: {}", now);
    
    // Format time
    println!(
        "Formatted Time: {}",
        now.format("%Y-%m-%d %H:%M:%S").unwrap()
    );

    // Get Unix timestamp
    println!("Timestamp: {}", now.timestamp().unwrap());

    // Add 1 hour to current time
    let future_time = now.add_duration(&CustomDuration::from_secs(3600));
    println!(
        "Time after 1 hour: {}",
        future_time.format("%Y-%m-%d %H:%M:%S").unwrap()
    );

    // Subtract 1 hour from current time
    let past_time = now.sub_duration(&CustomDuration::from_secs(3600));
    println!(
        "Time before 1 hour: {}",
        past_time.format("%Y-%m-%d %H:%M:%S").unwrap()
    );

    // Convert to a different timezone
    let ist_time = now.to_timezone("+05:30").unwrap();
    println!("Time in IST: {}", ist_time);

    // Parse time from string
    let time_from_str = Time::from_str("2023-10-27 12:00:00+05:30", "%Y-%m-%d %H:%M:%S%z").unwrap();
    println!("Time from string: {}", time_from_str);
}
```

---

### 2. `CustomDuration` Struct

The `CustomDuration` struct allows you to represent durations and perform arithmetic operations. It can be initialized using seconds or parsed from a human-readable format (e.g., `"1h 30m"`).

#### Methods

- `from_secs(secs: u64)`: Creates a `CustomDuration` from the specified number of seconds.
- `format_human_readable()`: Returns the duration in a human-readable format (e.g., `"1 hour 1 minute"`).
- `from_str(duration_str: &str)`: Parses a human-readable duration string into a `CustomDuration`.
- Arithmetic operations:
  - `+` : Adds durations.
  - `-` : Subtracts durations.
  - `*` : Multiplies the duration by a scalar.
  - `/` : Divides the duration by a scalar.

#### Example Usage

```rust
use time_duration_api::time_utils::{CustomDuration, Time};

fn main() {
    // Create a duration of 3661 seconds (1 hour, 1 minute)
    let duration = CustomDuration::from_secs(3661);
    println!("Duration: {}", duration);
    println!(
        "Human-readable Duration: {}",
        duration.format_human_readable()
    );

    // Parse duration from string
    let duration_from_str = CustomDuration::from_str("1h 30m").unwrap();
    println!(
        "Duration from str: {}",
        duration_from_str.format_human_readable()
    );

    // Arithmetic operations on durations
    let duration_add = duration + CustomDuration::from_secs(60);
    println!("Duration add: {}", duration_add.format_human_readable());

    let duration_sub = duration - CustomDuration::from_secs(60);
    println!("Duration sub: {}", duration_sub.format_human_readable());

    let duration_mul = duration * 2;
    println!("Duration mul: {}", duration_mul.format_human_readable());

    let duration_div = duration / 2;
    println!("Duration div: {}", duration_div.format_human_readable());

    // Duration in seconds
    println!("Duration in seconds: {}", duration.as_secs());
}
```

---

## How to Use

To use the `time_duration_api` crate in your Rust project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
time_duration_api = "0.1.7"
chrono = "0.4"
humantime = "2.1"
serde = { version = "1.0", features = ["derive"] }
```

### Example: Using `time_duration_api` in `main.rs`

Your `main.rs` file should include the following imports:

```rust
use time_duration_api::time_utils::{CustomDuration, Time};
```

After that, you can use the provided methods and structs as shown in the examples above.

---
## Error Handling
The crate uses a custom TimeError enum for error reporting:
```rust
#[derive(Debug, Clone)]
pub enum TimeError {
    InvalidTime,
    InvalidTimeFormat(String),
    InvalidTimezoneFormat(String),
    ParseError(String),
}
```

- You should use the custom Result type alias to handle these errors gracefully.
```
pub type Result<T> = std::result::Result<T, TimeError>;
```
### Example:
```rust
use time_duration_api::time_utils::Time;

fn main() {
    let time = Time::from_str("invalid_time", "%Y-%m-%d %H:%M:%S");
    match time {
        Ok(t) => println!("Parsed time: {}", t),
        Err(e) => println!("Error: {}", e),
    }
}
```

# Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.