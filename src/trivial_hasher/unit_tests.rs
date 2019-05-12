use crate::{Hasher, TrivialHasher};

#[test]
fn hash_hashes_single_input() {
    // given input data and a hasher
    let data = String::from("hello");
    let mut sut = TrivialHasher::new();

    // when invoked
    let result = sut.hash(data.clone());

    // then the result should have an asterisk appended
    assert_eq!((data + "*").as_bytes(), &*result.hash);
}

#[test]
fn hash_hashes_two_inputs() {
    // given input data and a hasher
    let data = String::from("hello");
    let data2 = String::from("world");
    let mut sut = TrivialHasher::new();

    // when invoked
    let result = sut.hash(data.clone()).hash(data2.clone());

    // then the result should have an asterisk appended
    assert_eq!((data + &data2 + "*").as_bytes(), &*result.hash);
}

#[test]
fn reset_clears_existing_hash_value() {
    // given input data and a hasher
    let data = String::from("hello");
    let data2 = String::from("world");
    let mut sut = TrivialHasher::new();

    // when invoked
    let result = sut.hash(data.clone()).reset().hash(data2.clone());

    // then the result should have an asterisk appended
    assert_eq!((data2 + "*").as_bytes(), &*result.hash);
}

#[test]
fn result_returns_hash_value() {
    // given input data and a hasher
    let data = String::from("hello");
    let mut hasher = TrivialHasher::new();

    // when invoked
    let result = hasher.hash(data.clone()).result();

    // then the result should have an asterisk appended
    assert_eq!((data + "*").as_bytes(), &*result);
}
