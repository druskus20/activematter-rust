# activematter-rust

A port of https://github.com/pmocz/activematter-python 

## Building

```sh
cargo build --release --bin <name> 
```

### On Dardel: 

```sh
module load rust/1.78.0`
module load cce gcc aocc PrgEnv-gnu PrgEnv-cray PrgEnv-aocc
export CRAY_MPICH_DIR=/opt/cray/pe/mpich/8.1.28/ucx/cray/17.0/lib/pkgconfig
cargo build --release --bin <name> 
```

### With docker

```sh
./compile_and_export.sh
```

## Running

```sh
./target/release/<name> <N_BIRDS>
```


## Copyright notice

This is based on the original work of Philip Mocz (2021) Princeton Univeristy,
@PMocz. Forked from https://github.com/pmocz/activematter-python.

```
    Active Matter simulation
    Original work Copyright (C) 2021  Philip Mocz
    Modifications (rewrite) by Pedro Burgos, 2024

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
```
