use crate::block_hash::BlockHash;
use crate::hashable::Hashable;
use super::*;

use std::fmt::{ Debug, Result, Formatter };

pub fn check_difficulty(hash: &BlockHash, difficulty:  u128) -> bool {
    difficulty > difficulty_bytes_as_u128(hash)
}

pub struct Block {

    pub index: u32,
    pub timestamp: u64,
    pub hash: BlockHash,
    pub prev_block_hash: BlockHash,
    pub nonce: u64,
    pub payload: String,
    pub difficulty: u128,
}
impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Block [{}] {} {} {} nonce:{}", &self.index, &hex::encode(&self.hash), &self.timestamp, &self.payload, &self.nonce)
    }
}

impl Block {
    pub fn new(index: u32,timestamp: u64,prev_block_hash: BlockHash, nonce: u64,payload: String, difficulty: u128) -> Self {
        Block {
            index,
            timestamp,
            hash: vec![0; 32],
            prev_block_hash,
            nonce,
            payload,
            difficulty
        }
    }
    pub fn mine(&mut self) {

        for nonce_attempt in 0..(u64::max_value()) {
            self.nonce = nonce_attempt;
            let hash = self.hash();
            if check_difficulty(&hash, self.difficulty) {
                self.hash = hash;
                return ;
            }
        }

    }
}

impl Hashable for Block {

    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u64_bytes(&self.timestamp));
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&u64_bytes(&self.nonce));
        bytes.extend(&u128_bytes(&self.difficulty));
        bytes
    }
}
