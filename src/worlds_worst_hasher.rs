use crate::Hasher;
use std::borrow::Borrow;
use std::fmt::Debug;
use std::vec::IntoIter;

#[derive(Clone, Debug)]
pub struct WorldsWorstHasher {
    data: Vec<u8>,
}

impl WorldsWorstHasher {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}

impl Hasher for WorldsWorstHasher {
    type Output = IntoIter<u8>;

    fn hash<D>(&mut self, data: D) -> &mut Self
    where
        D: IntoIterator + Debug + PartialEq,
        D::Item: Borrow<u8> + PartialEq,
    {
        self.data.pop();
        self.data.extend(data);
        self.data.push('*' as u8);
        self
    }

    fn reset(&mut self) -> &mut Self {
        self.data.clear();
        self
    }

    fn result(&self) -> Self::Output {
        self.data.into_iter()
    }
}
