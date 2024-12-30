use time_duration_api::time_utils::{CustomDuration, Time};

fn main() {
    // Example usage of Time struct
    let mut now = Time::now();  // Make `now` mutable
    println!("Current Time: {}", now);
    println!(
        "Formatted Time: {}",
        now.format("%Y-%m-%d %H:%M:%S").unwrap()
    );

    println!("Timestamp: {}", now.timestamp().unwrap());

    // Make `future_time` mutable
    let mut future_time = now.add_duration(&CustomDuration::from_secs(3600));
    println!(
        "Time after 1 hour: {}",
        future_time.format("%Y-%m-%d %H:%M:%S").unwrap()
    );

    // Make `past_time` mutable
    let mut past_time = now.sub_duration(&CustomDuration::from_secs(3600));
    println!(
        "Time before 1 hour: {}",
        past_time.format("%Y-%m-%d %H:%M:%S").unwrap()
    );

    // Make `now` mutable again for timezone conversion
    let ist_time = now.to_timezone("+05:30").unwrap();
    println!("Time in IST: {}", ist_time);

    // Make `time_from_str` mutable
    let mut time_from_str = Time::from_str("2023-10-27 12:00:00+05:30", "%Y-%m-%d %H:%M:%S%z").unwrap();
    println!("Time from string: {}", time_from_str);

    // Example usage of CustomDuration struct
    let duration = CustomDuration::from_secs(3661);
    println!("Duration: {}", duration);
    println!(
        "Human-readable Duration: {}",
        duration.format_human_readable()
    );

    let duration_from_str = CustomDuration::from_str("1h 30m").unwrap();
    println!(
        "Duration from str: {}",
        duration_from_str.format_human_readable()
    );
    let duration_add = duration + CustomDuration::from_secs(60);
    println!("Duration add: {}", duration_add.format_human_readable());
    let duration_sub = duration - CustomDuration::from_secs(60);
    println!("Duration sub: {}", duration_sub.format_human_readable());
    let duration_mul = duration * 2;
    println!("Duration mul: {}", duration_mul.format_human_readable());

    let duration_div = duration / 2;
    println!("Duration div: {}", duration_div.format_human_readable());

    println!("Duration in seconds: {}", duration.as_secs());
    let time_from_trait = Time::from_str("2023-10-27 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    println!("Time from trait: {}", time_from_trait);
}
