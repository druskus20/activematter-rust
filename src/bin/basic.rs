extern crate rand;
use activematter_rust::params::*;
use activematter_rust::utils;
use rand::rngs::StdRng;
use rand::Rng;
use std::f64::consts::PI;

fn main() {
    let n = utils::parse_n();
    let mut rng: StdRng = utils::seed_rng(SEED);
    let t_start = utils::get_instant();

    // Initialize bird positions
    let mut x: Vec<f64> = (0..n).map(|_| rng.gen::<f64>() * L).collect();
    let mut y: Vec<f64> = (0..n).map(|_| rng.gen::<f64>() * L).collect();

    // Initialize bird velocities
    let mut theta: Vec<f64> = (0..n).map(|_| 2.0 * PI * rng.gen::<f64>()).collect();
    let mut vx: Vec<f64> = theta.iter().map(|&t| V0 * t.cos()).collect();
    let mut vy: Vec<f64> = theta.iter().map(|&t| V0 * t.sin()).collect();

    // Simulation Main Loop
    for t in 0..NT {
        // Move
        for i in 0..n {
            x[i] += vx[i] * DT;
            y[i] += vy[i] * DT;

            // Apply periodic boundary conditions
            x[i] = x[i] % L;
            y[i] = y[i] % L;
        }

        // Find mean angle of neighbors within R
        let mut mean_theta = vec![0.0; n];
        for b in 0..n {
            let mut sx = 0.0;
            let mut sy = 0.0;
            for i in 0..n {
                if ((x[i] - x[b]).powi(2) + (y[i] - y[b]).powi(2)) < R.powi(2) {
                    sx += theta[i].cos();
                    sy += theta[i].sin();
                }
            }
            mean_theta[b] = sy.atan2(sx);
        }

        // Add random perturbations
        for b in 0..n {
            theta[b] = mean_theta[b] + ETA * (rng.gen::<f64>() - 0.5);
        }

        // Update velocities
        for b in 0..n {
            vx[b] = V0 * theta[b].cos();
            vy[b] = V0 * theta[b].sin();
        }
        if PRINT {
            utils::print_flock_positions(t, &x, &y, &vx, &vy);
        }
    }

    let t_end = utils::get_instant();
    let duration = t_end - t_start;
    let duration = duration.as_secs() as f64 * 1e9 + duration.subsec_nanos() as f64;
    utils::print_time(utils::time_to_unit(duration, "ns", TIME_UNIT), TIME_UNIT);
}
