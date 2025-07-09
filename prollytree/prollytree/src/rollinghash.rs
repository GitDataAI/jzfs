use std::collections::VecDeque;

pub struct RollingHash {
    pub window: VecDeque<u8>,
    pub window_size: usize,
    pub hash: u64,
    pub prime: u64,
    pub base: u64,
    pub base_pow: u64,
}

impl RollingHash {
    pub fn new(window_size: usize) -> Self {
        let prime: u64 = 1099511628211;
        let base: u64 = 256;

        let mut base_pow: u64 = 1;
        for _ in 0..window_size-1 {
            base_pow = (base_pow * base) % prime;
        }

        RollingHash {
            window: VecDeque::with_capacity(window_size),
            window_size,
            hash: 0,
            prime,
            base,
            base_pow,
        }
    }

    pub fn add(&mut self, byte: u8) {
        self.hash = (self.hash * self.base + byte as u64) % self.prime;
        self.window.push_back(byte);

        if self.window.len() > self.window_size {
            let old_byte = self.window.pop_front().unwrap();
            self.hash = (self.hash + self.prime -
                ((old_byte as u64 * self.base_pow) % self.prime)) % self.prime;
        }
    }

    pub fn hash(&self) -> u64 {
        self.hash
    }

    pub fn reset(&mut self) {
        self.window.clear();
        self.hash = 0;
    }
}
