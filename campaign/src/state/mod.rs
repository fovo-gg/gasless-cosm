// state/mod.rs for campaign

extern crate cosmwasm_std;
extern crate cw_storage_plus;
extern crate serde;
use self::cosmwasm_std::{Addr, Coin};
use self::cw_storage_plus::{Item, Map};
use self::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Campaign {
    pub creator: Addr,
    pub name: String,
    pub tasks: Vec<String>,
    pub reward_pool: Coin,
    pub expiration: u64,
}

pub const CAMPAIGN: Item<Campaign> = Item::new("campaign");
pub const ENGAGED_USERS: Map<Addr, bool> = Map::new("engaged_users");
