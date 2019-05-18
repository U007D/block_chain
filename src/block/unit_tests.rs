use super::*;
use crate::{as_bytes_iter::AsBytesIter, worlds_worst_hasher::WorldsWorstHasher, Hasher};
use std::slice::Iter;

#[derive(Clone, Debug, PartialEq)]
struct MyData {
    data: String,
}

impl<'a> AsBytesIter<'a> for MyData {
    type Output = Iter<'a, u8>;

    fn as_bytes_iter(&self) -> Self::Output {
        self.data.as_bytes().iter()
    }
}

#[test]
fn new_block_hashed_with_worlds_worst_hasher() {
    // given an arbitrary data type, an `impl Hasher` and a constructor
    let payload = String::from("test data");
    let data = MyData { data: payload }.as_bytes_iter();
    let hasher = WorldsWorstHasher::new();

    let prev_hash = hasher.clone().hash(String::new().as_bytes()).result();

    let sut = Block::new;

    // when constructor is invoked
    let result = sut(data.clone(), prev_hash.clone(), hasher);

    // then the result should be as expected
    let expected_block = assert_eq!(
        Block {
            hash: as_bytes(&data)
                .iter()
                .chain(as_bytes(&prev_hash).iter())
                .chain(&['*' as u8])
                .map(|&v| v)
                .collect(),
            data,
            prev_hash,
        },
        result
    );
}
