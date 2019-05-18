use super::*;
use crate::{worlds_worst_hasher::WorldsWorstHasher, Hasher};

#[derive(Clone, Debug, PartialEq)]
struct MyData {
    data: String,
}

impl AsBytesIter for MyData {
    fn as_bytes_iter(&self) -> impl Iterator<Item = u8> {
        self.data.as_bytes().iter()
    }
}

#[test]
fn new_block_hashed_with_worlds_worst_hasher() {
    // given an arbitrary data type, an `impl Hasher` and a constructor
    let payload = String::from("test data");
    let data = MyData { data: payload }.serialize_u8();
    let hasher = WorldsWorstHasher::new();

    let prev_hash = hasher.clone().hash(String::new().as_bytes()).result();

    let sut = Block::new;

    // when constructor is invoked
    let result = sut(data.clone(), prev_hash.clone(), hasher);

    // then the result should be as expected
    assert_eq!(
        Block {
            hash: to_byte_slice(&data)
                .iter()
                .chain(to_byte_slice(&prev_hash).iter())
                .chain(&['*' as u8])
                .map(|&v| v)
                .collect(),
            data,
            prev_hash,
        },
        result
    );
}
