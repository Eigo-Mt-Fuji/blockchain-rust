use std::fmt::{ self, Debug, Formatter };
use std::time::{ SystemTime };
pub type BlockHash = Vec<u8>;
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
fn main() {

    let now = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };

    let block = Block::new(1, now, vec![0;32], 10000, "hoge".to_string());

    println!("Blockchain rust {:?}", block);
}