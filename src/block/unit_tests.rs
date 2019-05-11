use super::*;
use hashbrown::hash_map::DefaultHashBuilder;
use std::hash::{BuildHasher, Hasher};

#[derive(Clone, Debug, Hash, PartialEq)]
struct MyData {
    data: String,
}

#[test]
fn new_block_hashed_with_hashbrown_default_hash_algorithm() {
    // given an `impl Hash` arbitrary data type and an `impl Hasher` and a constructor
    let payload = String::from("test data");
    let data = MyData { data: payload };
    let builder = DefaultHashBuilder::default();
    let hasher = builder.build_hasher();

    let mut prev_hasher = builder.build_hasher();
    prev_hasher.write(String::new().as_bytes());
    let prev_hash = prev_hasher.finish();

    let sut = Block::new;

    // when constructor is invoked
    let result = sut(data.clone(), prev_hash.clone(), hasher);

    // then the result should be as expected
    let mut test_hasher = builder.build_hasher();
    test_hasher.write(to_byte_slice(&data));
    test_hasher.write_u64(prev_hash);

    assert_eq!(
        Block {
            data,
            prev_hash,
            hash: test_hasher.finish()
        },
        result
    );
}
