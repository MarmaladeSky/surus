use std::hash::{BuildHasher, Hasher, RandomState};

pub fn u64() -> u64 {
    RandomState::new().build_hasher().finish()
}

pub fn text() -> String {
    format!("v{}", u64())
}

pub fn int() -> i32 {
    (u64() % 1_000) as i32
}
