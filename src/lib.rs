mod getters;

use near_sdk::collections::LookupMap;
use near_sdk::wee_alloc;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, near_bindgen, AccountId,
};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub type Token = String;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize)]
pub struct Marketplace {
    pub(crate) tokens: LookupMap<String, Token>,
    pub(crate) owner: AccountId,
}

impl Default for Marketplace {
    fn default() -> Self {
        env::panic(b"cannot default init marketplace");
    }
}

//////////////////////
// Marketplace Core //
//////////////////////
#[near_bindgen]
impl Marketplace {
    #[init]
    pub fn new(owner: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            tokens: LookupMap::new(b"m".to_vec()),
            owner,
        }
    }

    pub fn new_token(&mut self, owner_id: String, token_id: Token) {
        self.tokens.insert(&owner_id, &token_id);
    }
}
