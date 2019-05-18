use std::borrow::Borrow;
use std::fmt::Debug;

pub trait Hasher: Clone + Debug {
    type Output: Iterator<Item = u8>;

    fn hash<D>(&mut self, data: D) -> &mut Self
    where
        D: IntoIterator + Debug + PartialEq,
        D::Item: Borrow<u8> + PartialEq;
    fn reset(&mut self) -> &mut Self;
    fn result(&self) -> Self::Output;
}
