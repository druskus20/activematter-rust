extern crate rand;
use activematter_rust::params::*;
use rand::Rng;
use std::f64::consts::PI;

fn main() {
    let mut rng = rand::thread_rng();

    // Initialize bird positions
    let mut x: Vec<f64> = (0..N).map(|_| rng.gen::<f64>() * L).collect();
    let mut y: Vec<f64> = (0..N).map(|_| rng.gen::<f64>() * L).collect();

    // Initialize bird velocities
    let mut theta: Vec<f64> = (0..N).map(|_| 2.0 * PI * rng.gen::<f64>()).collect();
    let mut vx: Vec<f64> = theta.iter().map(|&t| V0 * t.cos()).collect();
    let mut vy: Vec<f64> = theta.iter().map(|&t| V0 * t.sin()).collect();

    // Simulation Main Loop
    for _ in 0..NT {
        // Move
        for i in 0..N {
            x[i] += vx[i] * DT;
            y[i] += vy[i] * DT;

            // Apply periodic boundary conditions
            x[i] = x[i] % L;
            y[i] = y[i] % L;
        }

        // Find mean angle of neighbors within R
        let mut mean_theta = vec![0.0; N];
        for b in 0..N {
            let mut sx = 0.0;
            let mut sy = 0.0;
            for i in 0..N {
                if ((x[i] - x[b]).powi(2) + (y[i] - y[b]).powi(2)) < R.powi(2) {
                    sx += theta[i].cos();
                    sy += theta[i].sin();
                }
            }
            mean_theta[b] = sy.atan2(sx);
        }

        // Add random perturbations
        for b in 0..N {
            theta[b] = mean_theta[b] + ETA * (rng.gen::<f64>() - 0.5);
        }

        // Update velocities
        for b in 0..N {
            vx[b] = V0 * theta[b].cos();
            vy[b] = V0 * theta[b].sin();
        }
    }

    println!("Simulation complete.");
}
