extern crate ndarray;
extern crate rand;

use ndarray::prelude::*;
use ndarray::Array1;
use rand::Rng;
use std::f64::consts::PI;

const V0: f64 = 1.0; // velocity
const ETA: f64 = 0.5; // random fluctuation in angle (in radians)
const L: f64 = 10.0; // size of box
const R: f64 = 1.0; // interaction radius
const DT: f64 = 0.2; // time step
const NT: usize = 200; // number of time steps
const N: usize = 500; // number of birds

fn main() {
    let mut rng = rand::thread_rng();

    // Initialize bird positions
    let mut x = Array1::<f64>::from_shape_fn(N, |_| rng.gen::<f64>() * L);
    let mut y = Array1::<f64>::from_shape_fn(N, |_| rng.gen::<f64>() * L);

    // Initialize bird velocities
    let mut theta = Array1::<f64>::from_shape_fn(N, |_| 2.0 * PI * rng.gen::<f64>());
    let mut vx = theta.mapv(|t| V0 * t.cos());
    let mut vy = theta.mapv(|t| V0 * t.sin());

    // Simulation Main Loop
    for _ in 0..NT {
        // Move
        x += &(vx * DT);
        y += &(vy * DT);

        // Apply periodic boundary conditions
        x.mapv_inplace(|xi| xi % L);
        y.mapv_inplace(|yi| yi % L);

        // Find mean angle of neighbors within R
        let mut mean_theta = theta.clone();
        for b in 0..N {
            let dx = &x - x[b];
            let dy = &y - y[b];
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
            mean_theta[b] = sy.atan2(sx);
        }

        // Add random perturbations
        theta = mean_theta + ETA * (&Array1::<f64>::from_shape_fn(N, |_| rng.gen::<f64>()) - 0.5);

        // Update velocities
        vx = theta.mapv(|t| V0 * t.cos());
        vy = theta.mapv(|t| V0 * t.sin());
    }

    println!("Simulation complete.");
}
