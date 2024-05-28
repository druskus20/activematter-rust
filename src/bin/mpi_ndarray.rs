use activematter_rust::params::*;
use mpi::environment::*;
use mpi::topology::*;
use mpi::traits::*;
use ndarray::prelude::*;
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
    // Initialize bird positions
    let mut x = Array1::<f64>::from_shape_fn(N, |_| rng.gen::<f64>() * L);
    let mut y = Array1::<f64>::from_shape_fn(N, |_| rng.gen::<f64>() * L);

    // Initialize bird velocities
    let mut theta = Array1::<f64>::from_shape_fn(N, |_| 2.0 * PI * rng.gen::<f64>());
    let mut vx = theta.mapv(|t| V0 * t.cos());
    let mut vy = theta.mapv(|t| V0 * t.sin());

    // Divide work among processes
    let n_per_process = N / size as usize;
    let start = rank as usize * n_per_process;
    let end = if rank == size - 1 {
        N
    } else {
        (rank as usize + 1) * n_per_process
    };

    // Simulation Main Loop
    for _ in 0..NT {
        // Move
        for i in start..end {
            x[i] = (x[i] + vx[i] * DT) % L;
            y[i] = (y[i] + vy[i] * DT) % L;
        }

        // Gather all positions
        let mut all_x = Array1::<f64>::zeros(N);
        let mut all_y = Array1::<f64>::zeros(N);
        world.all_gather_into(&x.slice(s![start..end]).to_vec()[..], &mut all_x.to_vec());
        world.all_gather_into(&y.slice(s![start..end]).to_vec()[..], &mut all_y.to_vec());

        // Find mean angle of neighbors within R
        let mut mean_theta = Array1::<f64>::zeros(n_per_process);
        for b in start..end {
            let mut sx = 0.0;
            let mut sy = 0.0;
            for i in 0..N {
                if ((all_x[i] - all_x[b]).powi(2) + (all_y[i] - all_y[b]).powi(2)) < R.powi(2) {
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
        vx = &theta.mapv(f64::cos) * V0;
        vy = &theta.mapv(f64::sin) * V0;

        // Synchronize thetas across processes
        let mut gathered_theta = Array1::<f64>::zeros(N);
        world.all_gather_into(
            &theta.slice(s![start..end]).to_vec()[..],
            &mut gathered_theta.to_vec(),
        );
        theta.assign(&Array::from_vec(gathered_theta.to_vec()));
    }

    if rank == 0 {
        println!("Simulation complete.");
    }

    // Finalize MPI (automatically called at the end of the scope)
}