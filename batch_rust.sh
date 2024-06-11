#!/bin/bash -l

#SBATCH -t 1:00:00
#SBATCH -A edu24.DD2356
#SBATCH -p shared


module load rust

nodes=$1
tasks=$2

#SBATCH --job-name="pedro_birds_$nodes_$tasks"
#SBATCH -e errors-${SLURM_JOB_NUM_NODES}-${SLURM_NTASKS}.e"


for birds in 128 256 512 1024 2048 4096; do
  touch rust_basic-${SLURM_JOB_NUM_NODES}-${SLURM_NTASKS}-${birds}.txt
  touch rust_mpi-${SLURM_JOB_NUM_NODES}-${SLURM_NTASKS}-${birds}.txt
  touch rust_mpi_ndarray-${SLURM_JOB_NUM_NODES}-${SLURM_NTASKS}-${birds}.txt
  touch rust_rayon-${SLURM_JOB_NUM_NODES}-${SLURM_NTASKS}-${birds}.txt
  touch rust_rayon_ndarray-${SLURM_JOB_NUM_NODES}-${SLURM_NTASKS}-${birds}.txt

  for i in 1 2 3 4 5; do 
    echo "Running $i rust_basic $birds" >> rust_basic-${SLURM_JOB_NUM_NODES}-${SLURM_NTASKS}-${birds}.txt
    srun ./rust_basic $birds >> rust_basic-${SLURM_JOB_NUM_NODES}-${SLURM_NTASKS}-${birds}.txt
    echo "Running $i rust_mpi $birds" >> rust_mpi-${SLURM_JOB_NUM_NODES}-${SLURM_NTASKS}-${birds}.txt
    mpirun srun ./rust_mpi $birds >> rust_mpi-${SLURM_JOB_NUM_NODES}-${SLURM_NTASKS}-${birds}.txt
    echo "Running $i rust_mpi_ndarray $birds" >> rust_mpi_ndarray-${SLURM_JOB_NUM_NODES}-${SLURM_NTASKS}-${birds}.txt
    mpirun srun ./rust_mpi_ndarray $birds >> rust_mpi_ndarray-${SLURM_JOB_NUM_NODES}-${SLURM_NTASKS}-${birds}.txt
    echo "Running $i rust_rayon $birds" >> rust_rayon-${SLURM_JOB_NUM_NODES}-${SLURM_NTASKS}-${birds}.txt
    srun ./rust_rayon $birds >> rust_rayon-${SLURM_JOB_NUM_NODES}-${SLURM_NTASKS}-${birds}.txt
    echo "Running $i rust_rayon_ndarray $birds" >> rust_rayon_ndarray-${SLURM_JOB_NUM_NODES}-${SLURM_NTASKS}-${birds}.txt
    srun ./rust_rayon_ndarray $birds >> rust_rayon_ndarray-${SLURM_JOB_NUM_NODES}-${SLURM_NTASKS}-${birds}.txt
  done
done

