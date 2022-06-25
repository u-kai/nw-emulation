use rand::{prelude::StdRng, Rng, RngCore, SeedableRng};

pub fn gen_rand_bytes(len: usize) -> Vec<u8> {
    let mut thread = rand::thread_rng();
    let seed = thread.gen::<[u8; 32]>();
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let mut key_bytes = vec![0u8; len];
    rng.fill_bytes(&mut key_bytes);
    key_bytes
}
