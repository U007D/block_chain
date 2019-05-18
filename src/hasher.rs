use std::fmt::Debug;

pub trait Hasher: Clone + Debug {
    type Output;

    fn hash<D: AsRef<[u8]>>(&mut self, data: D) -> &mut Self;
    fn reset(&mut self) -> &mut Self;
    fn result(&self) -> Self::Output;
}
