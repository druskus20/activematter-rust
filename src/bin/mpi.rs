use activematter_rust::params::*;
use mpi::environment::*;
use mpi::topology::*;
use mpi::traits::*;
use rand::Rng;
use std::f64::consts::PI;

fn main() {
    // Initialize MPI
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let rank = world.rank();
    let size = world.size();

    let mut rng = rand::thread_rng();

    // Initialize bird positions
    let mut x: Vec<f64> = (0..N).map(|_| rng.gen::<f64>() * L).collect();
    let mut y: Vec<f64> = (0..N).map(|_| rng.gen::<f64>() * L).collect();

    // Initialize bird velocities
    let mut theta: Vec<f64> = (0..N).map(|_| 2.0 * PI * rng.gen::<f64>()).collect();
    let mut vx: Vec<f64> = theta.iter().map(|&t| V0 * t.cos()).collect();
    let mut vy: Vec<f64> = theta.iter().map(|&t| V0 * t.sin()).collect();

    // MPI stuff
    let n_per_process = N / size as usize;
    let start = rank as usize * n_per_process;
    let end = if rank == size - 1 {
        N
    } else {
        (rank as usize + 1) * n_per_process
    };

    // Main loop
    for _ in 0..NT {
        // Move
        for i in start..end {
            x[i] += vx[i] * DT;
            y[i] += vy[i] * DT;

            // Apply periodic boundary conditions
            x[i] = x[i] % L;
            y[i] = y[i] % L;
        }

        // Gather positions
        let mut all_x = vec![0.0; N];
        let mut all_y = vec![0.0; N];
        world.all_gather_into(&x[start..end], &mut all_x[..]);
        world.all_gather_into(&y[start..end], &mut all_y[..]);

        // Calc mean angle neighbors
        let mut mean_theta = vec![0.0; n_per_process];
        for b in start..end {
            let mut sx = 0.0;
            let mut sy = 0.0;
            for i in 0..N {
                let a: f64 = all_x[i] - all_x[b];
                let b: f64 = all_y[i] - all_y[b];
                if (a.powi(2) + b.powi(2)) < R.powi(2) {
                    sx += theta[i].cos();
                    sy += theta[i].sin();
                }
            }
            mean_theta[b - start] = sy.atan2(sx);
        }

        // Add random perturbations
        for b in start..end {
            theta[b] = mean_theta[b - start] + ETA * (rng.gen::<f64>() - 0.5);
        }

        // Update velocities
        for b in start..end {
            vx[b] = V0 * theta[b].cos();
            vy[b] = V0 * theta[b].sin();
        }

        // Gather thetas across processes
        let mut gathered_theta = vec![0.0; N];
        world.all_gather_into(&theta[start..end], &mut gathered_theta[..]);
        theta.copy_from_slice(&gathered_theta);
    }

    if rank == 0 {
        println!("Simulation complete.");
    }

    // Finalize MPI (automatically called at the end of the scope)
}
