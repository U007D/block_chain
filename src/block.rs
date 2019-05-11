use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
    mem::size_of,
    slice::from_raw_parts,
};

// core::hash::Hasher::finish() -> u64
pub type HashType = u64;

#[cfg(test)]
mod unit_tests;

#[derive(Debug, Hash, PartialEq)]
pub struct Block<T>
where
    T: Debug + Hash + PartialEq,
{
    data: T,
    prev_hash: HashType,
    hash: HashType,
}

impl<T> Block<T>
where
    T: Debug + Hash + PartialEq,
{
    pub fn new(data: T, prev_hash: HashType, mut hasher: impl Hasher) -> Self {
        hasher.write(to_byte_slice(&data));
        hasher.write_u64(prev_hash);
        Self {
            data,
            prev_hash,
            hash: hasher.finish(),
        }
    }
}

fn to_byte_slice<T>(data: &T) -> &[u8] {
    unsafe { from_raw_parts((data as *const T) as *const u8, size_of::<T>()) }
}
