use blockchain;

use blockchain::{Block, HashHable};

fn main() {
    let mut block = Block::new(
        0,
        0,
        vec![0; 32],
        0,
        "Genesis Block".to_string(),
        0x00ffffffffffffffffffffffffffff,
    );

    block.hash = block.hash();

    println!("{:#?}", &block);

    block.mine();

    println!("{:#?}", &block);
    // println!("the hash {:?}", &h);
}
