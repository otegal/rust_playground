use rand::{Rng, SeedableRng};
use rand::distributions::Standard;
use rand_pcg::Pcg64Mcg;

pub fn new_u32_vec(n: usize) -> Vec<u32> {
    let mut rng = Pcg64Mcg::from_seed([0; 16]);
    let mut v = Vec::with_capacity(n);

    // _ で受け取る事ですぐに破棄できる
    for _ in 0..n {
        v.push(rng.sample(&Standard));
    }

    v
}
