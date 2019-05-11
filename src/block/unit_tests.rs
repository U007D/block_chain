use super::*;
use hashbrown::hash_map::DefaultHashBuilder;
#[allow(deprecated)]
use std::hash::{BuildHasher, Hasher, SipHasher};

#[derive(Clone, Debug, Hash, PartialEq)]
struct MyData {
    data: String,
}

#[test]
fn new_block_hashed_with_hashbrown_default_fx_hash_algorithm() {
    // given an `impl Hash` arbitrary data type, an `impl Hasher` and a constructor
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

#[test]
fn new_block_hashed_with_sip_hash_algorithm() {
    // given an `impl Hash` arbitrary data type, a `SipHasher` and a constructor
    let payload = String::from("test data");
    let data = MyData { data: payload };

    // Note: `SipHash` has been deprecated.  Do not use this hash for production code.  `SipHash` has been used here
    // to test that `Block`'s behavior is correct and invariant over changes in the hashing algorithm.  Any algorithm
    // other than `Hashbrown`'s current default of `FxHash` could be use for this test.  As of the time of this
    // writing, `SipHash`, although deprecated, appears to be the only alternative available by name within `std`.
    #[allow(deprecated)]
    let builder = SipHasher::default;
    let hasher = builder();

    let mut prev_hasher = builder();
    prev_hasher.write(String::new().as_bytes());
    let prev_hash = prev_hasher.finish();

    let sut = Block::new;

    // when constructor is invoked
    let result = sut(data.clone(), prev_hash.clone(), hasher);

    // then the result should be as expected
    let mut test_hasher = builder();
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
