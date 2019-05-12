use super::*;
use crate::{Hasher, TrivialHasher};

#[derive(Clone, Debug, PartialEq)]
struct MyData {
    data: String,
}

#[test]
fn new_block_hashed_with_trivial_hasher_algorithm() {
    // given an arbitrary data type, an `impl Hasher` and a constructor
    let payload = String::from("abcdefghijklmnopqrstuvwxyz");
    let data = MyData { data: payload };
    let hasher = TrivialHasher::new();

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
