use std::{env::args, error::Error, thread::sleep, time::Duration};

use chrono::Local;
use rand::{distributions::Uniform, prelude::Distribution, thread_rng};

const DEFAULT_MIN_DELAY: u64 = 100;
const DEFAULT_MAX_DELAY: u64 = 5000;
const DEFAULT_COUNT_LIMIT: u64 = 0;
const LOG_MESSAGES: [&str; 4] = ["ERROR An error is usually an exception that has been caught and not handled.", "INFO This is less important than debug log and is often used to provide context in the current task.", "WARN A warning that should be ignored is usually at this level and should be actionable.", "DEBUG This is a debug log that shows a log that can be ignored."];

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = args().skip(1);

    let min_delay = match args.next() {
        Some(min) => min.parse().unwrap_or(DEFAULT_MIN_DELAY),
        None => DEFAULT_MIN_DELAY,
    };

    let max_delay = match args.next() {
        Some(max) => max.parse().unwrap_or(DEFAULT_MAX_DELAY),
        None => DEFAULT_MAX_DELAY,
    }
    .max(min_delay);

    let count_limit = match args.next() {
        Some(count_limit) => count_limit.parse().unwrap_or(DEFAULT_COUNT_LIMIT),
        None => DEFAULT_COUNT_LIMIT,
    };

    let mut count = 0;
    let mut rng = thread_rng();
    let delay_range = Uniform::new_inclusive(min_delay, max_delay);
    let log_range = Uniform::new(0u8, 4u8);
    while count_limit == 0 || count < count_limit {
        sleep(Duration::from_millis(delay_range.sample(&mut rng)));
        let date = Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, false);
        let message = LOG_MESSAGES[log_range.sample(&mut rng) as usize];
        println!("{date} {message}");
        count += 1;
    }

    Ok(())
}
