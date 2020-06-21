use blockchainlib::*;

fn main () {
  let difficulty = 0x000fffffffffffffffffffffffffffff;
  let mut block = Block::new(0, 0, vec![0; 32], 118318, "Genesis block!".to_owned(), difficulty);

  block.mine();
  println!("Mined genesis block {:?}", &block);

  let mut last_hash = block.hash.clone();

  let mut blockchain = Blockchain {
    blocks: vec![block],
  };

  println!("Verify: {}", &blockchain.verify());

  for i in 1..=10 {
    let mut block = Block::new(i, now(), last_hash, 0, "another block".to_owned(), difficulty);

    block.mine();
    println!("Mined genesis block {:?}", &block);

    last_hash = block.hash.clone();
   
    blockchain.blocks.push(block);
    
    println!("Verify: {}", &blockchain.verify());
  }

  blockchain.blocks[3].previous_block_hash[18] = 8;

  println!("Verify: {}", &blockchain.verify());

}
