use std::time::{ SystemTime };
use blockchainlib::*;

fn main() {

    let now = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };

    let mut block = Block::new(
        1, 
        now, 
        vec![0;32], 
        1, 
        "Genesis Block".to_owned()
    );
    let h = block.hash();
    println!("block={:?} hash={:?}", &block, h);
    block.hash = h;
    println!("block={:?}, hash={:?}", &block, &block.hash);
}