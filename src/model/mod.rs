extern  crate blake2;

use std::vec::Vec;
use std::collections::HashMap;
use std::convert::Into;
use std::time::SystemTime;
use blake2::{Blake2b, Digest};
use std::string::String;
use std::convert::From;


#[derive(Debug, Clone)]
pub struct Blockchain {
    /// Stores all the blocks which are accepted already within the blockchain
    pub blocks: Vec<Block>,

    /// Lookup from AccountID (will be a public key later) to Account.
    pub accounts: HashMap<String, Account>,

    /// Will store transactions which should be added to the chain but aren't yet
    pending_transactions: Vec<Transaction>
}


pub trait WorldState {
    fn get_user_ids(&self) -> Vec<String>;

    fn get_account_by_id_mut(&mut self, id: &String) -> Option<&Account>;
}