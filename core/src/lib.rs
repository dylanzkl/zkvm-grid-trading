use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct Account {
    pub usd: i32,
    pub eth: i32,
}

impl Hash for Account {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.usd.hash(state);
        self.eth.hash(state);
    }
}

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}

#[derive(Clone, Deserialize, Serialize)]
pub struct GridRequest {
    pub acc: Account,
    pub ts: Vec<[i32; 2]>,
}
