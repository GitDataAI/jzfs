
#[derive(Debug, Clone)]
pub struct Chunk {
    pub offset: u64,
    pub size: usize,
    pub hash: u64,
}

impl Chunk {
    pub fn new(offset: u64, size: usize, hash: u64) -> Self {
        Chunk { offset, size, hash }
    }

    pub fn offset(&self) -> u64 { self.offset }
    pub fn size(&self) -> usize { self.size }
    pub fn hash(&self) -> u64 { self.hash }
}
