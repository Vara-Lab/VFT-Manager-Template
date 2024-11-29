use sails_rs::prelude::*;

// Struct to handle the state of the contract
#[derive(Default)]
pub struct VFTManagerState {
    // Vec to store admins that can do special actions
    pub admins: Vec<ActorId>,
    // contract id from the extended vft contract
    pub vft_contract_id: Option<ActorId>,
    // Min tokens to mint to the contract
    pub min_tokens_to_add: u128,
    // Amount of tokens to swap from tokens to varas
    pub tokens_per_vara: u128,
}

impl VFTManagerState {
    pub fn new(admin: ActorId) -> Self {
        let mut temp = Self::default();
        temp.admins.push(admin);
        temp
    }

    // Helper function that returns if an address is an admin
    pub fn is_admin(&self, address: &ActorId) -> bool {
        self.admins.contains(address)
    }
}
