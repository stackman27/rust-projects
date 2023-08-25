use serde_derive::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt::Write;

use chrono::prelude::*;

#[derive(Debug, Clone, Serialize)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f32,
}

#[derive(Serialize, Debug)]
pub struct BlockHeader {
    timestamp: i64,
    nonce: u32,
    pre_hash: String,
    merkle: String,
    difficulty: u32,
}

#[derive(Serialize, Debug)]
pub struct Block {
    header: BlockHeader,
    count: u32,
    transactions: Vec<Transaction>,
}

pub struct Chain {
    chain: Vec<Block>,
    curr_trans: Vec<Transaction>,
    difficulty: u32,
    miner_addr: String,
    reward: f32,
}

impl Chain {
    pub fn new(miner_addr: String, difficulty: u32) -> Chain {
        let mut chain = Chain {
            chain: Vec::new(),
            curr_trans: Vec::new(),
            difficulty,
            miner_addr,
            reward: 100.0,
        };

        chain.generate_new_block();
        chain
    }

    pub fn new_transaction(&mut self, sender: String, receiver: String, amount: f32) -> bool {
        let new_trans = Transaction {
            sender,
            receiver,
            amount,
        };

        self.curr_trans.push(new_trans);
        true
    }

    // last_block_hash gets the last element in the list and gets the block header, if block not found then return all 0
    pub fn last_block_hash(&self) -> String {
        let block = match self.chain.last() {
            Some(block) => block,
            None => return String::from_utf8(vec![48; 64]).unwrap(),
        };
        Chain::hash(&block.header)
    }

    pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
        self.difficulty = difficulty;
        true
    }

    pub fn update_reward(&mut self, reward: f32) -> bool {
        self.reward = reward;
        true
    }

    // generate_new_block() generates new block with new timestamp
    pub fn generate_new_block(&mut self) -> bool {
        let header = BlockHeader {
            timestamp: Utc::now().timestamp_millis(),
            nonce: 0,
            pre_hash: self.last_block_hash(),
            merkle: String::new(),
            difficulty: self.difficulty,
        };

        let reward_trans = Transaction {
            sender: String::from("Root"),
            receiver: self.miner_addr.clone(), // we use clone here because of how rust handles ownership, so .clone() creates a new copy of the miner_addr
            amount: self.reward,
        };

        let mut block = Block {
            header,
            count: 0,
            transactions: vec![],
        };

        block.transactions.push(reward_trans); // push adds single value to the end of array
        block.transactions.append(&mut self.curr_trans); // appends an entire vector to the end of the array
        block.count = block.transactions.len() as u32;
        // this take our block transactions and generate a merkle proof
        block.header.merkle = Chain::get_merkle(block.transactions.clone());

        Chain::proof_of_work(&mut block.header);

        println!("{:#?}", &block);
        self.chain.push(block);
        true
    }

    fn get_merkle(curr_trans: Vec<Transaction>) -> String {
        let mut merkle = Vec::new();

        for t in &curr_trans {
            // calculate hash of each transaction
            let hash = Chain::hash(t);
            merkle.push(hash); // merkle vector will be vector of hashes
        }

        // if we have off number of hashes we clone the last one to make it even
        if merkle.len() % 2 == 1 {
            let last = merkle.last().cloned().unwrap();
            merkle.push(last);
        }

        // this will iterate until we have a single hash with all transactions "h1+h2+h3..."
        while merkle.len() > 1 {
            let mut h1 = merkle.remove(0);
            let mut h2 = merkle.remove(0);
            h1.push_str(&mut h2); // this is putting both hash together "h1+h2"

            let nh = Chain::hash(&h1); // hash "h1+h2"
            merkle.push(nh);
        }
        merkle.pop().unwrap()
    }

    // perform proof of work by increment nounce by 1 until we found the right nonce
    // for ex:
    // if difficulty = 2
    // nonce = 100
    // hash = 00324351312312... (hash need to be leading with 2 zeros) because our difficulty is 2
    pub fn proof_of_work(header: &mut BlockHeader) {
        loop {
            let hash = Chain::hash(header);
            let slice = &hash[..header.difficulty as usize]; // append the difficulty [00...]

            match slice.parse::<u32>() {
                Ok(val) => {
                    // [0000] of u32 is just 0
                    if val != 0 {
                        header.nonce += 1;
                    } else {
                        println!("Block hash: {}", hash);
                        break;
                    }
                }
                Err(_) => {
                    header.nonce += 1;
                    continue;
                }
            }
        }
    }

    pub fn hash<T: serde::Serialize>(item: &T) -> String {
        let input = serde_json::to_string(&item).unwrap();
        let mut hasher = Sha256::new();

        hasher.update(input.as_bytes());
        let res = hasher.finalize();
        let vec_res = res.to_vec();

        Chain::hex_to_string(vec_res.as_slice())
    }

    pub fn hex_to_string(vec_res: &[u8]) -> String {
        let mut s = String::new();

        for b in vec_res {
            write!(&mut s, "{:x}", b).expect("unable to write")
        }
        s
    }
}
