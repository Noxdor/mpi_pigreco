pub mod calc {
    use rand::prelude::*;

    pub fn create_random_point() -> (f64, f64) {
        // create random number generator
        let mut rng = thread_rng();
        // create a point in the rectangle x e [-1; 1], y e [-1; 1]
        (rng.gen_range(-1f64..=1f64), rng.gen_range(-1f64..=1f64))
    }

    pub fn is_hit(x: f64, y: f64) -> bool {
        // radius of unit circle
        const R: f64 = 1f64;
        // calculate with pythagoras if point is contained in unit circle
        f64::sqrt(x.powf(2f64) + y.powf(2f64)) <= R
    }
}

pub mod com {
    use mpi::traits::*;

    pub fn receive_results(size: i32, world: mpi::topology::SystemCommunicator) -> f64 {
        let mut num_hits: u64 = 0;
        let mut num_it: u64 = 0;

        // iterate over each process to get its result
        for process in 1..size {
            // result buffer (array) with [num_hits, num_it]
            let mut buf = [0u64; 2];
            world.process_at_rank(process).receive_into(&mut buf);
            num_hits += buf[0];
            num_it += buf[1];
        }

        num_hits as f64 / (num_it as f64) * 4f64
    }

    pub fn calculate_part(num_it: u64, world: mpi::topology::SystemCommunicator) {
        let mut num_hits_local: u64 = 0;

        for _ in 0..num_it {
            let (x, y) = super::calc::create_random_point();
            if super::calc::is_hit(x, y) {
                num_hits_local += 1;
            }
        }
        let buf = [num_hits_local, num_it];
        world.process_at_rank(0).send(&buf);
    }
}
