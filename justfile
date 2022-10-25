run: build
  mpiexec -n 6 target/release/mpi_pigreco
  

build:
  cargo build --release

