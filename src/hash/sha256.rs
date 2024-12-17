pub struct Sha256{
    pub hash: Vec<u8>
}

impl Sha256{
    pub fn new(hash: Vec<u8>) -> Self{
        Sha256{ hash }
    }
    pub fn write_string(&mut self, data: String) -> &mut Self{
        self.hash.extend(data.as_bytes());
        self
    }
    pub fn write_bytes(&mut self, data: Vec<u8>) -> &mut Self{
        self.hash.extend(data);
        self
    }
    pub fn extend(&mut self, data: Sha256) -> &mut Self{
        self.hash.extend(data.hash);
        self
    }
    pub fn finalize(&self) -> String{
        sha256::digest(self.hash.clone())
    }
}