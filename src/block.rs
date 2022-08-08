use crate::block_hash::BlockHash;
use crate::hashable::Hashable;
use super::*;

use std::fmt::{ Debug, Result, Formatter };

pub struct Block {

    pub index: u32,
    pub timestamp: u64,
    pub hash: BlockHash,
    pub prev_block_hash: BlockHash,
    pub nonce: u64,
    pub payload: String,
}
impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Block [{}] {} {} {}", &self.index, &hex::encode(&self.hash), &self.timestamp, &self.payload)
    }
}

impl Block {
    pub fn new(index: u32,timestamp: u64,prev_block_hash: BlockHash, nonce: u64,payload: String) -> Self {
        Block {
            index,
            timestamp,
            hash: vec![0; 32],
            prev_block_hash,
            nonce,
            payload
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
        return bytes;
    }
}