use activematter_rust::params::*;
use activematter_rust::utils;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use rayon::prelude::*;
use std::f64::consts::PI;

const V0: f64 = 1.0; // velocity
const ETA: f64 = 0.5; // random fluctuation in angle (in radians)
const L: f64 = 10.0; // size of box
const R: f64 = 1.0; // interaction radius
const DT: f64 = 0.2; // time step
const NT: usize = 200; // number of time steps
const N: usize = 500; // number of birds

fn main() {
    let mut rng: StdRng = utils::seed_rng(SEED);
    let t_start = utils::get_instant();

    // Initialize bird positions
    let mut x: Vec<f64> = (0..N).map(|_| rng.gen::<f64>() * L).collect();
    let mut y: Vec<f64> = (0..N).map(|_| rng.gen::<f64>() * L).collect();

    // Initialize bird velocities
    let mut theta: Vec<f64> = (0..N).map(|_| 2.0 * PI * rng.gen::<f64>()).collect();
    let mut vx: Vec<f64> = theta.iter().map(|&t| V0 * t.cos()).collect();
    let mut vy: Vec<f64> = theta.iter().map(|&t| V0 * t.sin()).collect();

    // Simulation Main Loop
    for t in 0..NT {
        // Move
        for i in 0..N {
            x[i] += vx[i] * DT;
            y[i] += vy[i] * DT;

            // Apply periodic boundary conditions
            x[i] = x[i] % L;
            y[i] = y[i] % L;
        }

        // Find mean angle of neighbors within R
        let mean_theta: Vec<f64> = (0..N)
            .into_par_iter()
            .map(|b| {
                let mut sx = 0.0;
                let mut sy = 0.0;
                for i in 0..N {
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
