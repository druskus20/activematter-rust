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

RUN cargo build --release --all 
 
 RUN mkdir -p /build && \
    executables=$(find target/release -maxdepth 1 -type f -executable) && \
    for exe in $executables; do mv "$exe" "/build/rust_$(basename "$exe")"; done

