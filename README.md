# Time Duration API

The Time Duration Library provides functionalities for handling time and durations in Rust. It offers features to manage specific points in time and durations, along with various formatting options and arithmetic operations.

## Features

- Create `Time` instances representing specific points in time.
- Calculate durations between different time instances.
- Format time according to custom patterns.
- Easy-to-use API for integrating time-related functionality into your Rust applications.

## Installation

Add the following line to your `Cargo.toml` file under `[dependencies]`:

```toml
time_duration_api = "0.1.5"
```
# Time Duration API

The Time Duration API is a Rust library that provides functionality for working with time and durations. It allows you to easily manage time instances, calculate durations, and format time according to custom patterns.

## Features
- **Time Management:** Handle specific points in time, including retrieval of the current time.
- **Time Formatting:** Format time according to custom date and time formats.
- **Time Zone Support:** Format time with different time zones.
- **Duration Arithmetic:** Perform arithmetic operations such as addition, subtraction, multiplication, and division on durations.
- **Error Handling:** Improved error handling for better usability.
- **Comprehensive Documentation:** Clear explanations and examples for easy integration.


# Usage

```rust 
use time_duration::{Time, CustomDuration};

fn main() {
    // Create a Time instance representing the current time
    let now = Time::now();

    // Format the time
    let formatted_time = now.format("%Y-%m-%d %H:%M:%S");
    println!("Formatted time: {}", formatted_time);

    // Add a duration to a time
    let duration = CustomDuration::from_secs(3600);
    let future_time = now.add_duration(&duration);
    println!("Future time: {}", future_time.format("%Y-%m-%d %H:%M:%S"));

    // Subtract a duration from a time
    let past_time = now.sub_duration(&duration);
    println!("Past time: {}", past_time.format("%Y-%m-%d %H:%M:%S"));
}
```

# author
- [Jenin Sutradhar](https://github.com/GigaCodeGojo)
