#[cfg(test)]
mod unit_tests;

use crate::Hasher;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TrivialHasher {
    hash: Vec<u8>,
}

impl TrivialHasher {
    pub fn new() -> Self {
        Self { hash: Vec::new() }
    }
}
impl Hasher for TrivialHasher {
    type Output = Vec<u8>;

    fn hash<D: AsRef<[u8]>>(&mut self, data: D) -> &mut Self {
        self.hash.pop();
        self.hash.extend(data.as_ref());
        self.hash.push('*' as u8);
        self
    }

    fn reset(&mut self) -> &mut Self {
        self.hash.clear();
        self
    }

    fn result(&self) -> Self::Output {
        self.hash.clone()
    }
}
