// comment the following line and uncomment out the next use statement to successfully build.
use crate::{near_bindgen, AccountId, Marketplace};
#[cfg(not(target = "wasm32-unknown-unknown"))]
use crate::MarketplaceContract;

#[near_bindgen] // <- this sucker prevents from wasm-compiling.
impl Marketplace {
    pub fn get_owner(&self) -> &AccountId {
        &self.owner
    }
}
