FROM ubuntu:20.04

# Set non-interactive frontend
ENV DEBIAN_FRONTEND=noninteractive

# Set timezone to UTC
RUN ln -fs /usr/share/zoneinfo/UTC /etc/localtime

# Install necessary build dependencies
RUN apt-get update && \
    apt-get install -y build-essential curl git


RUN apt-get install -y mpich
RUN apt-get install -y libclang-dev



# Install rustup and set up Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install specific Rust version (if needed)
RUN rustup toolchain install stable


# Set the working directory
WORKDIR /usr/src/myapp

# Copy the source code into the container
COPY . .


RUN export RUSTFLAGS="-C target-cpu=znver2 -C target-feature=+sse4.2,+avx2,+fma,+bmi2,+pclmul,+aes"

RUN mkdir -p /build2
RUN cargo clean
RUN cargo build --release --bin    rayon_ndarray --features "ndarray_rayon"
RUN mv target/release/rayon_ndarray /build2/rust_rayon_ndarray

RUN cargo build --release --bin    basic
RUN cargo build --release --bin    mpi
RUN cargo build --release --bin    mpi_ndarray
RUN cargo build --release --bin    ndarray
RUN cargo build --release --bin    rayon

#RUN cargo build --release --bin    ndarray_blas
#RUN cargo build --release --bin    rayon_ndarray_blas

 
 RUN mkdir -p /build2 && \
    executables=$(find target/release -maxdepth 1 -type f -executable) && \
    for exe in $executables; do mv "$exe" "/build2/rust_$(basename "$exe")"; done

