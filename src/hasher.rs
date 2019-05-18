use std::fmt::Debug;

pub trait Hasher {
    type Output: Debug;
    fn hash<D>(&mut self, data: D) -> &mut Self
    where
        D: IntoIterator<Item = u8> + Debug;
    fn reset(&mut self) -> &mut Self;
    fn result(&self) -> Self::Output;
}
