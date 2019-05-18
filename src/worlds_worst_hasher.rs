use crate::Hasher;

#[derive(Clone, Debug, PartialEq)]
pub struct WorldsWorstHasher {
    data: Vec<u8>,
}

impl WorldsWorstHasher {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}

impl Hasher for WorldsWorstHasher {
    type Output = Vec<u8>;

    fn hash<D: AsRef<[u8]>>(&mut self, data: D) -> &mut Self {
        self.data.pop();
        self.data.extend(data.as_ref());
        self.data.push('*' as u8);
        self
    }

    fn reset(&mut self) -> &mut Self {
        self.data.clear();
        self
    }

    fn result(&self) -> Self::Output {
        self.data.clone()
    }
}
