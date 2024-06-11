use std::time::Instant;

use rand::{rngs::StdRng, SeedableRng};

use crate::params;

/// Parses the number of birds from command-line arguments.
///
/// If no argument is provided, it returns the default number of birds.
pub fn parse_n() -> usize {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1) {
        Some(arg) => arg.parse().unwrap_or_else(|_| params::N_DEFAULT),
        None => params::N_DEFAULT,
    }
}

/// Seeds a random number generator with the given seed.
pub fn seed_rng(seed: u8) -> StdRng {
    let seed: [u8; 32] = [seed; 32];
    let rng = SeedableRng::from_seed(seed);
    rng
}

/// Gets the current instant in time.
pub fn get_instant() -> Instant {
    Instant::now()
}

/// Converts time from one unit to another.
///
/// Supported units are: "ns" (nanoseconds), "us" (microseconds), "ms" (milliseconds), "s" (seconds).
pub fn time_to_unit(time: f64, source_unit: &str, target_unit: &str) -> f64 {
    let time_in_seconds = match source_unit {
        "ns" => time / 1e9,
        "us" => time / 1e6,
        "ms" => time / 1e3,
        "s" => time,
        _ => {
            eprintln!("Unsupported source unit: {}", source_unit);
            return -1.0;
        }
    };

    match target_unit {
        "ns" => time_in_seconds * 1e9,
        "us" => time_in_seconds * 1e6,
        "ms" => time_in_seconds * 1e3,
        "s" => time_in_seconds,
        _ => {
            eprintln!("Unsupported target unit: {}", target_unit);
            -1.0
        }
    }
}

/// Prints the given time with the specified unit.
pub fn print_time(t: f64, unit: &str) {
    println!("Time: {} {}", t, unit);
}

/// Prints the positions and velocities of birds in the flock.
///
/// Prints each bird's position (x, y) and velocity (vx, vy) along with the step number.
pub fn print_flock_positions(step: usize, x: &[f64], y: &[f64], vx: &[f64], vy: &[f64]) {
    for i in 0..x.len() {
        println!(
            "{} {} {:.2} {:.2} {:.2} {:.2}",
            step, i, x[i], y[i], vx[i], vy[i]
        );
    }
}
