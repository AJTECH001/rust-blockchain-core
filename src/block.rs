pub struct Block {
    timestamp: i64,
    pre_block_hash: String,
    hash: String,
    transactions: Vec<transaction>,
    nonce: i64,
    height: usize,
}

pub fn new_block (pre_block_hash: String, transactions: &[transaction], height: usize) -> Block {
    let mut block = Block {
        timestamp: create::current_timestamp(),
        pre_block_hash,
        hash: String::new(),
        transactions: transactions.to_vec(),
        nonce: 0,
        height,
    };

    let pow = ProofOfWork::new_proof_of_work(block.colne());
    let (nonce, hash) = pow.run();
    block.nonce = nonce;
    block.hash = hash;
    return block;
}