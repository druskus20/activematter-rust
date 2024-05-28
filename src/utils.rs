use std::time::{Duration, Instant};

use rand::{
    rngs::{StdRng, ThreadRng},
    SeedableRng,
};

pub fn seed_rng(seed: u8) -> StdRng {
    let seed: [u8; 32] = [seed; 32];
    let rng = SeedableRng::from_seed(seed);
    rng
}

pub fn get_instant() -> Instant {
    Instant::now()
}

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

pub fn print_time(t: f64, unit: &str) {
    println!("Time: {} {}", t, unit);
}

pub fn print_flock_positions(step: usize, x: &[f64], y: &[f64], vx: &[f64], vy: &[f64]) {
    for i in 0..x.len() {
        println!(
            "{} {} {:.2} {:.2} {:.2} {:.2}",
            step, i, x[i], y[i], vx[i], vy[i]
        );
    }
}
