use crate::Block;
use crate::Hash;
use crate::block::check_difficulty;
use crate::hashable::Hashable;
use std::collections::HashSet;

// what derive, what derive(Debug)
#[derive(Debug)]
pub enum BlockValidationErr {
    MismatchIndex,
    InvalidHash,
    AchronologicalTimestamp,
    MismatchedPrevHash,
    InvalidGenesisBlock,
    InvalidInput,
    InsufficientInputValue,
    InvalidCoinbaseTransaction,
}

pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub output_hashes: HashSet<Hash>,
    pub unspent_outputs: HashSet<Hash>,
}

impl Blockchain {
    
    pub fn new() -> Self {
        Blockchain{ 
            blocks: vec![],
            output_hashes: HashSet::new(),
            unspent_outputs: HashSet::new(),
        }
    }
    pub fn update_with_block(&mut self, block: Block) -> Result<(), BlockValidationErr> {
    
        let i = self.blocks.len();

        if block.index != i as u32 {
            println!("Index mismatch: {} != {}", 
                &block.index, 
                &i
            );
            return Err(BlockValidationErr::MismatchIndex);
        }else if !check_difficulty(&block.hash(), block.difficulty) {
            println!("Difficulty fail.");
            return Err(BlockValidationErr::InvalidHash);
        } else if i != 0 {
            let prev_block = &self.blocks[i - 1];

            if block.timestamp <= prev_block.timestamp {
                println!("Time did not increase");
                return Err(BlockValidationErr::AchronologicalTimestamp);
            }else if block.prev_block_hash != prev_block.hash {
                println!("prev_block_hash did not correct.");
                return Err(BlockValidationErr::MismatchedPrevHash);
            }
        } else if block.prev_block_hash != vec![0;32] {
            return Err(BlockValidationErr::InvalidGenesisBlock);
        }
        
        // see: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.split_first
        if let Some(( coinbase, transactions) ) = block.transactions.split_first() {

            if !coinbase.is_coinbase() {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
            }
            let mut block_spent: HashSet<Hash> = HashSet::new();
            let mut block_created: HashSet<Hash> = HashSet::new();
            let mut total_fee = 0;
            for transaction in transactions {
                
                let input_hashes = transaction.input_hashes();
                
                // TODO: how work subtract between HashSet 
                if (&input_hashes - &self.unspent_outputs).is_empty() || !(&input_hashes & &block_spent).is_empty() {
                    return Err(BlockValidationErr::InvalidInput);
                }

                let input_value = transaction.input_value();
                let output_value = transaction.output_value();
                
                if input_value < output_value {
                    println!("Insufficient input value");
                    return Err(BlockValidationErr::InsufficientInputValue);
                }

                let fee = transaction.input_value() - transaction.output_value();
                total_fee += fee;    

                block_spent.extend(input_hashes);
                block_created.extend(transaction.output_hashes());
            }

            if coinbase.output_value() < total_fee {

                return Err(BlockValidationErr::InsufficientInputValue);
            }
            // TODO: what is retain of HashSet
            self.unspent_outputs.retain( |output| !block_spent.contains(output) );
            self.unspent_outputs.extend(block_created);
            self.blocks.push(block);
        }        

        Ok(())
    }
}