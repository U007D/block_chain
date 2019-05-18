use crate::{as_bytes::AsBytes, Hasher};
use std::fmt::Debug;

#[cfg(test)]
mod unit_tests;

#[derive(Debug, PartialEq)]
pub struct Block<D, H>
where
    D: AsBytes + Debug + PartialEq,
    H: Hasher,
{
    data: D,
    prev_hash: H::Output,
    hash: H::Output,
}

impl<D, H> Block<D, H>
where
    D: AsBytes + Debug + PartialEq,
    H: Hasher,
    H::Output: AsRef<[u8]>,
{
    pub fn new(data: D, prev_hash: H::Output, mut hasher: H) -> Self {
        hasher.reset();
        hasher.hash(data.as_bytes());
        hasher.hash(&prev_hash);

        Self {
            data,
            prev_hash,
            hash: hasher.result(),
        }
    }
}
