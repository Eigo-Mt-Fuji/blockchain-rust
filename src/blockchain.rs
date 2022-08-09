use crate::Block;
use crate::block::check_difficulty;
use crate::hashable::Hashable;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    
    pub fn verify(&self) -> bool {
        for (i, block) in self.blocks.iter().enumerate() {

            if block.index != i as u32 {
                println!("Index mismatch: {} != {}", 
                    &block.index, 
                    &i
                );
                return false;
            }else if !check_difficulty(&block.hash(), block.difficulty) {
                println!("Difficulty fail.");
                return false;
            } else if i != 0 {
                let prev_block = &self.blocks[i - 1];

                if block.timestamp <= prev_block.timestamp {
                    println!("Time did not increase");
                    return false;
                }else if block.prev_block_hash != prev_block.hash {
                    println!("prev_block_hash did not correct.");
                    return false;
                }
            } else if block.prev_block_hash != vec![0;32] {
                return false;
            }
        }
        true
    }
}