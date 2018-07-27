use rand::rngs::SmallRng;
use rand::{thread_rng, SeedableRng};

pub fn rng_maker() -> SmallRng {
    SmallRng::from_rng(thread_rng()).unwrap()
}
