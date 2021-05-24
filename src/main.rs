use blockchain;

use crate::blockchain::now;
use blockchain::{Block, BlockChain, Output, Transaction};

fn main() {
    const DIFFICULTY: u128 = 0x000fffffffffffffffffffffffffffff;

    let mut genesis_block = Block::new(
        0,
        now(),
        vec![0; 32],
        vec![Transaction {
            inputs: vec![],
            outputs: vec![
                Output {
                    to_addr: "Alice".to_string(),
                    value: 50,
                },
                Output {
                    to_addr: "Alice".to_string(),
                    value: 7,
                },
            ],
        }],
        DIFFICULTY,
    );

    genesis_block.mine();

    println!("Mined the genesis block {:#?}", &genesis_block);

    let mut last_hash = genesis_block.hash.clone();

    let mut blockchain = BlockChain::new();

    blockchain
        .update_with_block(genesis_block)
        .expect("Failed to add genesis block");

    let mut block = Block::new(
        0,
        now(),
        last_hash,
        vec![
            Transaction {
                inputs: vec![],
                outputs: vec![Output {
                    to_addr: "Chris".to_string(),
                    value: 536,
                }],
            },
            Transaction {
                inputs: vec![blockchain.blocks[0].transactions[0].outputs[0].clone()],
                outputs: vec![
                    Output {
                        to_addr: "Alice".to_string(),
                        value: 36,
                    },
                    Output {
                        to_addr: "Bob".to_string(),
                        value: 12,
                    },
                ],
            },
        ],
        DIFFICULTY,
    );

    block.mine();

    println!("Mined the genesis block {:#?}", &block);

    let mut blockchain = BlockChain::new();

    last_hash = block.hash.clone();

    blockchain
        .update_with_block(block)
        .expect("Failed to add second block");
}
