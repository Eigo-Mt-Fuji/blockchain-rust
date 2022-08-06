
use std::fmt::{ self, Debug, Formatter };
use crate::BlockHash;

pub struct Block {

    pub index: u32,
    pub timestamp: u64,
    pub hash: BlockHash,
    pub prev_block_hash: BlockHash,
    pub nonce: u64,
    pub payload: String,
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
impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Block")
    }
}
