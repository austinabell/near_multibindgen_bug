// comment the following line and uncomment out the next use statement to successfully build.
use crate::{near_bindgen, AccountId, Marketplace, MarketplaceContract};
//use crate::*; //::{near_bindgen, AccountId, Marketplace, MarketplaceContract};

#[near_bindgen] // <- this sucker prevents from wasm-compiling.
impl Marketplace {
    pub fn get_owner(&self) -> &AccountId {
        &self.owner
    }
}
