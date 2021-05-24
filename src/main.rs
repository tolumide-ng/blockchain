use blockchain;

use crate::blockchain::now;
use blockchain::{Block, BlockChain};
use std::{thread, time};

fn main() {
    const DIFFICULTY: u128 = 0x000fffffffffffffffffffffffffffff;

    let mut block = Block::new(
        0,
        now(),
        vec![0; 32],
        0,
        "Genesis Block".to_string(),
        DIFFICULTY,
    );

    block.mine();

    let mut last_hash = block.hash.clone();

    let mut blockchain = BlockChain {
        blocks: vec![block],
    };

    let two_millis = time::Duration::from_millis(2);

    // sleep to allow difference in time between the genesis block and the next block;
    thread::sleep(two_millis);

    for index in 1..=10 {
        let mut block = Block::new(
            index,
            now(),
            last_hash,
            0,
            "Another Block".to_string(),
            DIFFICULTY,
        );

        block.mine();

        last_hash = block.hash.clone();

        println!("{:#?}", &block);

        blockchain.blocks.push(block);

        println!("Verification {:?}", blockchain.verify());
    }
}
