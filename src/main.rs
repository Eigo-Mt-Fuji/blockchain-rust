use std::time::{ SystemTime };
use blockchainlib::*;

fn make_genesis_block(difficulty: u128) -> Block {

    let mut block = Block::new(
        0, 
        0, 
        vec![0;32], 
        0, 
        "Genesis Block".to_owned(),
        difficulty
    );
    block.mine();
    println!("Mined genesis block={:?}, hash={:?}", &block, &block.hash);
    return block;
}

fn main() {

    let difficulty = 0x000fffffffffffffffffffffffffffff;

    let block = make_genesis_block(difficulty);
    let mut last_hash = block.hash.clone();
    let mut chain = Blockchain {
        blocks: vec![block]
    };
    
    for i in 1..3 {
        let duration = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let now = duration.as_secs() as u64 * 1000 + duration.subsec_millis() as u64;

        let mut block = Block::new(i, now, last_hash, 0, "Another block!".to_owned(), difficulty);

        block.mine();
        println!("Mined block: {:?}", &block);

        last_hash = block.hash.clone();

        chain.blocks.push(block);
        println!("Verify={}", chain.verify());
    }
//    chain.blocks[2].index += 1;
    println!("{:?} verify_result={}", &chain.blocks, chain.verify());
}
