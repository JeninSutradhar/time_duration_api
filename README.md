# Time Duration API

The Time Duration API is a Rust library that provides functionality for working with time and durations. It allows you to easily manage time instances, calculate durations, and format time according to custom patterns.

## Features

- Create `Time` instances representing specific points in time.
- Calculate durations between different time instances.
- Format time according to custom patterns.
- Easy-to-use API for integrating time-related functionality into your Rust applications.

## Installation

Add the following line to your `Cargo.toml` file under `[dependencies]`:

```toml
time_duration_api = "0.1.2"
```
# Time Duration API

The Time Duration API is a Rust library that provides functionality for working with time and durations. It allows you to easily manage time instances, calculate durations, and format time according to custom patterns.

## Features

- Create `Time` instances representing specific points in time.
- Calculate durations between different time instances.
- Format time according to custom patterns.
- Easy-to-use API for integrating time-related functionality into your Rust applications.

## Installation

Add the following line to your `Cargo.toml` file under `[dependencies]`:

```toml
time_duration_api = "0.1.2"
```
- Then, run cargo build to download and build the library.

# Usage
```rust
use time_duration_api::time_duration::*;

fn main() {
    // Create a new Time instance representing the current time
    let current_time = Time::now();

    // Format the current time instance
    let formatted_time = current_time.format("%Y-%m-%d %H:%M:%S");

    println!("Current time: {}", formatted_time);

    // Create a new Time instance representing a specific time
    let custom_time = Time {
        timestamp: SystemTime::now(),
    };

    // Format the custom time instance
    let custom_formatted_time = custom_time.format("%A, %B %d, %Y %H:%M:%S");

    println!("Custom time: {}", custom_formatted_time);

    // Calculate the duration between two time instances
    let duration = custom_time.timestamp.duration_since(current_time.timestamp).unwrap();

    println!("Duration: {:?}", duration);

    // Convert the duration to seconds
    let seconds = duration.as_secs();

    println!("Duration in seconds: {}", seconds);
}
```
# author
- [Jenin Sutradhar](https://github.com/GigaCodeGojo)
