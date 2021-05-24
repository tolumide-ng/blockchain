use crate::Block;
use crate::HashHable;

use super::block;

pub struct BlockChain {
    pub blocks: Vec<Block>,
}

impl BlockChain {
    pub fn verify(&self) -> bool {
        for (i, block) in self.blocks.iter().enumerate() {
            if block.index != i as u32 {
                println!("Index mismatch {} != {}", block.index, i);
                return false;
            } else if !block::check_difficulty(&block.hash(), block.difficulty) {
                println!("Difficulty failed");
                return false;
            } else if i != 0 {
                let prev_block = &self.blocks[i - 1];
                if block.timestamp <= prev_block.timestamp {
                    println!("Time did not increase");
                    return false;
                } else if block.prev_block_hash != prev_block.hash {
                    println!("Hash Mismatch");
                    return false;
                }
            }
        }

        return true;
    }
}
