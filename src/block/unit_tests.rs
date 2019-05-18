use super::*;
use crate::{as_bytes::AsBytes, worlds_worst_hasher::WorldsWorstHasher, Hasher};

#[derive(Clone, Debug, PartialEq)]
struct MyData {
    data: String,
}

impl AsBytes for MyData {
    fn as_bytes(&self) -> &[u8] {
        self.data.as_bytes()
    }
}

#[test]
fn new_block_hashed_with_worlds_worst_hasher() {
    // given an arbitrary data type, an `impl Hasher` and a constructor
    let payload = String::from("abc");
    let data = MyData { data: payload };
    let hasher = WorldsWorstHasher::new();

    let prev_hash = hasher.clone().hash(String::new().as_bytes()).result();

    let sut = Block::new;

    // when constructor is invoked
    let result = sut(data.clone(), prev_hash.clone(), hasher);

    // then the result should be as expected
    assert_eq!(
        data.as_bytes()
            .iter()
            .chain(prev_hash.iter())
            .chain(&['*' as u8])
            .map(|&v| v)
            .collect::<Vec<_>>(),
        result.hash
    );
}
