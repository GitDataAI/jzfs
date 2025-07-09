use crate::chunk::Chunk;

#[derive(Debug)]
pub struct Node {
    pub chunks: Vec<Chunk>,
    pub children: Vec<Box<Node>>,
    pub is_leaf: bool,
}

impl Node {
    pub fn new(is_leaf: bool) -> Self {
        Node {
            chunks: Vec::new(),
            children: Vec::new(),
            is_leaf,
        }
    }
}
