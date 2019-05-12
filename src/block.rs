use crate::Hasher;
use std::{fmt::Debug, mem::size_of, slice::from_raw_parts};

#[cfg(test)]
mod unit_tests;

#[derive(Debug, Hash, PartialEq)]
pub struct Block<D, H>
where
    D: Debug + PartialEq,
    H: Hasher,
{
    data: D,
    prev_hash: H::Output,
    hash: H::Output,
}

impl<D, H> Block<D, H>
where
    D: Debug + PartialEq,
    H: Hasher,
{
    pub fn new(data: D, prev_hash: H::Output, mut hasher: H) -> Self {
        Self {
            hash: hasher
//                .reset()
                .hash(to_byte_slice(&data))
                .hash(to_byte_slice(&prev_hash))
                .result(),
            data,
            prev_hash,
        }
    }
}

fn to_byte_slice<T>(data: &T) -> &[u8] {
    unsafe { from_raw_parts((data as *const T) as *const u8, size_of::<T>()) }
}
