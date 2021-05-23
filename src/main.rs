use blockchain;

use blockchain::{Block, HashHable};

fn main() {
    let mut block = Block::new(0, 0, vec![0; 32], 0, "Genesis Block".to_string());

    let h = block.hash();

    block.hash = h;

    println!("{:#?}", block);
    // println!("the hash {:?}", &h);
}
