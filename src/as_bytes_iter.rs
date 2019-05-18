pub trait AsBytesIter<'a> {
    type Output: IntoIterator<Item = &'a u8>;

    fn as_bytes_iter(&self) -> Self::Output;
}
