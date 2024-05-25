# Time Duration API

The Time Duration Library provides functionalities for handling time and durations in Rust. It offers features to manage specific points in time and durations, along with various formatting options and arithmetic operations.

## Features
- **Time Handling:** Represent specific points in time with the Time struct.
- **Duration Management:** Manage durations of time with the CustomDuration struct.
- **Time Formatting:** Format time instances into strings with custom formats.
- **Timezone Support:** Convert times between different timezones seamlessly.
- **Arithmetic Operations:** Add, subtract, multiply, and divide durations with ease

## Installation

Add the following line to your `Cargo.toml` file under `[dependencies]`:

```toml
time_duration_api = "0.1.6"
```

# Usage

```rust
use time_duration::{CustomDuration, Time};

fn main() {
    // Creating a new Time instance representing the current time
    let current_time = Time::now();

    // Formatting the current time in UTC
    println!("Current UTC time: {}", current_time.format("%Y-%m-%d %H:%M:%S"));

    // Formatting the current time in a specific timezone
    match current_time.format_with_timezone("%Y-%m-%d %H:%M:%S", "+05:30") {
        Ok(time_str) => println!("Current time in +05:30 timezone: {}", time_str),
        Err(err) => println!("Error formatting time: {}", err),
    }

    // Getting the current timestamp as UNIX timestamp
    match current_time.timestamp() {
        Ok(timestamp) => println!("Current timestamp: {}", timestamp),
        Err(err) => println!("Error getting timestamp: {}", err),
    }

    // Creating a custom duration of 2 hours
    let duration = CustomDuration::from_secs(2 * 3600);

    // Adding the custom duration to the current time
    let future_time = current_time.add_duration(&duration);
    println!("Time after 2 hours: {}", future_time.format("%Y-%m-%d %H:%M:%S"));

    // Subtracting the custom duration from the current time
    let past_time = current_time.sub_duration(&duration);
    println!("Time 2 hours ago: {}", past_time.format("%Y-%m-%d %H:%M:%S"));

    // Using the multiplication operation for durations
    let multiplied_duration = duration.mul(3);
    println!("Duration multiplied by 3: {} seconds", multiplied_duration.as_secs());

    // Using the division operation for durations
    let divided_duration = duration.div(2);
    println!("Duration divided by 2: {} seconds", divided_duration.as_secs());
}
```

## Examples
- **Formatting Time:** Format time instances into custom strings.
- **Time Arithmetic:** Add or subtract durations from time instances.
- **Timezone Conversion:** Convert times between different timezones.


# Links
- **GitHub Repository:** https://github.com/yourusername/time_duration_api
- **Crates.io Package:** https://crates.io/crates/time_duration_api
- **lib.rs:** https://lib.rs/crates/time_duration_api