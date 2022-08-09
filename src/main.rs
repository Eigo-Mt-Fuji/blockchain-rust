
#[allow(unused_imports)]
use std::time::{ SystemTime };

use blockchainlib::*;

fn make_genesis_block(difficulty: u128) -> Block {

    let mut block = Block::new(
        0, 
        0, 
        vec![0;32], 
        0, 
        vec![
            Transaction {
                inputs: vec![
                ],
                outputs: vec![
                    Output {
                        to_addr: "Alice".to_owned(),
                        value: 100,
                    },
                    Output {
                        to_addr: "Bob".to_owned(),
                        value: 200,
                    },
                ],
            }
        ],
        difficulty
    );
    block.mine();
    println!("Mined genesis block={:?}, hash={:?}", &block, &block.hash);
    block
}

fn main() {

    let difficulty = 0x000fffffffffffffffffffffffffffff;

    let geneis_block = make_genesis_block(difficulty);
    let last_hash = geneis_block.hash.clone();
    
    // make chain and put genesis block
    let mut chain = Blockchain::new();
    chain.update_with_block(geneis_block).expect("Failed add geneis block.");

    let duration = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let now = duration.as_secs() as u64 * 1000 + duration.subsec_millis() as u64;

    let mut block = Block::new(
        1, 
        now, 
        last_hash, 
        0, 
        vec![
            Transaction {
                inputs: vec![
                ],
                outputs: vec![
                    Output {
                        //TODO: what does it means String.to_owned
                        to_addr: "Chris".to_owned(),
                        value: 100,
                    },
                ]
            },
            Transaction {
                inputs: vec![
                    // TODO: how does it work struct clone.
                    // TODO: what does it means blockchainlib::Output`, which does not implement the `Copy` trait.
                    chain.blocks[0].transactions[0].outputs[0].clone()
                ],
                outputs: vec![
                    Output {
                        //TODO: what does it means String.to_owned
                        to_addr: "Nice girl".to_owned(),
                        value: 90,
                    },
                    Output {
                        to_addr: "Good guy".to_owned(),
                        value: 10,
                    }
                ]
            }

        ]
        , difficulty);

    block.mine();
    println!("Mined block: {:?}", &block);

    chain.update_with_block(block).unwrap_or_else(|n| panic!("Failed add a block = {:?}", n));
    
    println!("{:?}", &chain.blocks);
}
