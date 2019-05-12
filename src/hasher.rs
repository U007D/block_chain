use std::fmt::Debug;

pub trait Hasher {
    type Output: Debug;
    fn hash<D: AsRef<[u8]>>(&mut self, data: D) -> &mut Self;
    fn reset(&mut self) -> &mut Self;
    fn result(&self) -> Self::Output;
}
