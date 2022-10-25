## Usage

### With `just`

If you have [just](https://github.com/casey/just) installed, just run `just` in the
command line to build and run the project.

If `just` should fail for you with an error message telling `there are not enough slots available`,
run `just run <n>` reducing the number for n until the command doesn't error. This number is usually
related to your PC's count of processor cores.

### Manually

Otherwise run

1. `cargo build --release`
2. `mpiexec -n 6 target/release/mpi_pigreco`

If 2. should fail for you with an error message telling `there are not enough slots available`,
reduce the number for `mpiexec`'s argument `n`.
