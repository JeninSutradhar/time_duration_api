use my_crate::{Time, CustomDuration};

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
