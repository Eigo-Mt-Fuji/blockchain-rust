use std::time::{ SystemTime };
use blockchainlib::*;

fn main() {

    let now = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };

    let mut block = Block::new(
        0, 
        0, 
        vec![0;32], 
        0, 
        "Genesis Block".to_owned(),
        0x0008ffffffffffffffffffffffffffff
    );
    println!("block={:?}", &block);
    block.hash = block.hash();
    println!("block={:?}, hash={:?}", &block, &block.hash);
    block.mine();
    println!("block={:?}, hash={:?}", &block, &block.hash);
}