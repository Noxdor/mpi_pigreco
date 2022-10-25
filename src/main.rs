use mpi::traits::*;
use mpi_pigreco::com;

// number of iterations per process
const NUM_IT: u64 = 10_000_000;

fn main() {
    // init mpi implementation and get handler
    // to common functions
    let universe = mpi::initialize().unwrap();
    // world (global) communicator
    let world = universe.world();
    // rank of THIS process
    let rank = world.rank();
    // size of world group (number of processes partaking)
    let size = world.size();

    // one process is the result collector, so we need
    // at least 2 processes to run this calculation
    if size <= 1 {
        eprintln!("Can't run with only 1 process.");
        std::process::exit(-1);
    }

    // first process, takes results of all other processes to calculate
    // an aggregated result
    if rank == 0 {
        let pi = com::receive_results(size, world);
        println!("pi was calculated to be {pi}.");
    // all other processes, that calculate pi to a part and send
    // their results to the first process (with rank 0)
    } else {
        com::calculate_part(NUM_IT, world);
    }
}
