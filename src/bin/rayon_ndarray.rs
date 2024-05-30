use activematter_rust::params::*;
use activematter_rust::utils;
use ndarray::prelude::*;
use ndarray::Array1;
use ndarray::Zip;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use rayon::prelude::*;
use std::f64::consts::PI;

fn main() {
    let n = utils::parse_n();
    let mut rng: StdRng = utils::seed_rng(SEED);
    let t_start = utils::get_instant();

    // Initialize bird positions
    let mut x = Array1::<f64>::from_shape_fn(n, |_| rng.gen::<f64>() * L);
    let mut y = Array1::<f64>::from_shape_fn(n, |_| rng.gen::<f64>() * L);

    // Initialize bird velocities
    let mut theta = Array1::<f64>::from_shape_fn(n, |_| 2.0 * PI * rng.gen::<f64>());
    let mut vx = theta.mapv(|t| V0 * t.cos());
    let mut vy = theta.mapv(|t| V0 * t.sin());

    // Simulation Main Loop
    for t in 0..NT {
        // Move
        x += &(vx * DT);
        y += &(vy * DT);

        // Apply periodic boundary conditions
        x.mapv_inplace(|xi| xi % L);
        y.mapv_inplace(|yi| yi % L);

        // Find mean angle of neighbors within R
        let mut mean_theta = Array1::<f64>::zeros(n);
        Zip::from(&mut mean_theta)
            .and(&x)
            .and(&y)
            .par_for_each(|mean_theta, &x_b, &y_b| {
                let dx = &x - x_b;
                let dy = &y - y_b;
                let dist_squared = &dx.mapv(|d| d.powi(2)) + &dy.mapv(|d| d.powi(2));
                let neighbors = dist_squared.mapv(|d2| d2 < R.powi(2));
                let sx = theta
                    .iter()
                    .zip(neighbors.iter())
                    .filter(|(_, &n)| n)
                    .map(|(&t, _)| t.cos())
                    .sum::<f64>();
                let sy = theta
                    .iter()
                    .zip(neighbors.iter())
                    .filter(|(_, &n)| n)
                    .map(|(&t, _)| t.sin())
                    .sum::<f64>();
                *mean_theta = sy.atan2(sx);
            });

        // Add random perturbations
        theta = mean_theta + ETA * (&Array1::<f64>::from_shape_fn(n, |_| rng.gen::<f64>()) - 0.5);

        // Update velocities
        vx = theta.mapv(|t| V0 * t.cos());
        vy = theta.mapv(|t| V0 * t.sin());

        if PRINT {
            let x = x.as_slice().unwrap();
            let y = y.as_slice().unwrap();
            let vx = vx.as_slice().unwrap();
            let vy = vy.as_slice().unwrap();
            utils::print_flock_positions(t, &x, &y, &vx, &vy);
        }
    }

    let t_end = utils::get_instant();
    let duration = t_end - t_start;
    let duration = duration.as_secs() as f64 * 1e9 + duration.subsec_nanos() as f64;
    utils::print_time(utils::time_to_unit(duration, "ns", TIME_UNIT), TIME_UNIT);
}
