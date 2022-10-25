default: (run "6")

run count: build
  mpiexec -n {{count}} target/release/mpi_pigreco
  

build:
  cargo build --release

