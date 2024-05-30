RUSTFLAGS='-C target-cpu=native' 
cargo build --release --all

# move executables to build and rename them to rust_*
executables=$(find target/release -maxdepth 1 -type f -executable)
# rename them and move them
mkdir ../build
for exe in $executables; do
    mv $exe ../build/rust_$(basename $exe)
done



