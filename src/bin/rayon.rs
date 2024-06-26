use activematter_rust::params::*;
use activematter_rust::utils;
use rand::rngs::StdRng;
use rand::Rng;
use rayon::prelude::*;
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
        let mean_theta: Vec<f64> = (0..n)
            .into_par_iter()
            .map(|b| {
                let mut sx = 0.0;
                let mut sy = 0.0;
                for i in 0..n {
                    let dx = x[i] - x[b];
                    let dy = y[i] - y[b];
                    let dist_squared = dx * dx + dy * dy;
                    if dist_squared < R * R {
                        sx += theta[i].cos();
                        sy += theta[i].sin();
                    }
                }
                sy.atan2(sx)
            })
            .collect();

        // Add random perturbations
        theta = mean_theta
            .into_iter()
            .map(|t| t + ETA * (rng.gen::<f64>() - 0.5))
            .collect();

        // Update velocities
        vx = theta.iter().map(|&t| V0 * t.cos()).collect();
        vy = theta.iter().map(|&t| V0 * t.sin()).collect();

        if PRINT {
            utils::print_flock_positions(t, &x, &y, &vx, &vy);
        }
    }

    let t_end = utils::get_instant();
    let duration = t_end - t_start;
    let duration = duration.as_secs() as f64 * 1e9 + duration.subsec_nanos() as f64;
    utils::print_time(utils::time_to_unit(duration, "ns", TIME_UNIT), TIME_UNIT);
}
