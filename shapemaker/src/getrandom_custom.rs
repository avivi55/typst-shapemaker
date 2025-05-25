use getrandom::{register_custom_getrandom, Error};
use std::sync::atomic::{AtomicU64, Ordering};

static SEED: AtomicU64 = AtomicU64::new(1);
static COUNTER: AtomicU64 = AtomicU64::new(0);

fn custom_getrandom(dest: &mut [u8]) -> Result<(), Error> {
    let seed = SEED.load(Ordering::Relaxed);
    let counter = COUNTER.fetch_add(1, Ordering::Relaxed);
    
    let mut state = seed.wrapping_add(counter);
    
    for chunk in dest.chunks_mut(8) {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        
        let bytes = state.to_le_bytes();
        let len = chunk.len().min(8);
        chunk[..len].copy_from_slice(&bytes[..len]);
        
        state = state.wrapping_mul(1103515245).wrapping_add(12345);
    }
    
    Ok(())
}

register_custom_getrandom!(custom_getrandom);

pub fn set_random_seed(seed: u64) {
    SEED.store(seed, Ordering::Relaxed);
    COUNTER.store(0, Ordering::Relaxed);
}

pub fn set_random_seed_from_string(input: &str) -> u64 {
    let seed = hash_string(input);
    set_random_seed(seed);
    seed
}

fn hash_string(input: &str) -> u64 {
    const FNV_OFFSET_BASIS: u64 = 14695981039346656037;
    const FNV_PRIME: u64 = 1099511628211;
    
    let mut hash = FNV_OFFSET_BASIS;
    for byte in input.bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

pub fn get_current_seed() -> u64 {
    SEED.load(Ordering::Relaxed)
}
