use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Lock {
    /// Last lock update.
    pub since_height: u64,

    /// Current lock owner. None if no lock.
    pub owner: Option<Addr>,
}

impl Lock {
    /// Sugar around owner verification
    pub fn is_locked(&self) -> bool {
        self.owner.is_some()
    }
}

pub const LOCK: Item<Lock> = Item::new("lock");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Whitelist {
    /// Vector of addresses authorized to use the lock.
    pub members: Vec<Addr>,
}

impl Whitelist {
    /// Proxy call to inner member.
    pub fn contains(&self, address: &Addr) -> bool {
        self.members.contains(address)
    }
}

pub const WHITELIST: Item<Whitelist> = Item::new("whitelist");
