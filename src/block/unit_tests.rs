use super::*;
use hashbrown::hash_map::DefaultHashBuilder;
use std::hash::{BuildHasher, Hasher};

#[derive(Debug, Hash, PartialEq)]
struct MyData {
    data: String,
}

impl MyData {
    pub const fn as_bytes(&self) -> &[u8] {
        union Slice<'a> {
            my_data: &'a MyData,
            slice: &'a [u8],
        }
        unsafe { Slice { my_data: self }.slice }
    }
}

#[test]
fn new_block_hashed_with_hashbrown_default_hash_algorithm() {
    // given an `impl Hash` arbitrary data type and an `impl Hasher` and a constructor
    let payload = String::from("test data");
    let data = MyData { data: payload };
    let builder = DefaultHashBuilder::default();
    let mut hasher = builder.build_hasher();

    let mut prev_hasher = builder.build_hasher();
    prev_hasher.write(String::new().as_bytes());
    let prev_hash = prev_hasher.finish();

    let sut = Block::new;

    // when constructor is invoked
    let result = sut(data, prev_hash.clone(), hasher);

    // then the result should be as expected
    let mut test_hasher = builder.build_hasher();
    test_hasher.write(data.as_bytes());
    test_hasher.write_u64(prev_hash);
    assert_eq!(
        Block {
            data: payload,
            prev_hash,
            hash: test_hasher.finish()
        },
        result
    );
}
