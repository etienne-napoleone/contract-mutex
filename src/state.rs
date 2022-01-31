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

#[cfg(test)]
mod tests {
    use cosmwasm_std::Addr;

    use super::{Lock, Whitelist};

    #[test]
    fn lock_is_locked() {
        let locked_lock = Lock {
            since_height: 123,
            owner: Some(Addr::unchecked("terra12345")),
        };

        assert!(locked_lock.is_locked());

        let unlocked_lock = Lock {
            since_height: 123,
            owner: None,
        };

        assert!(!unlocked_lock.is_locked());
    }

    #[test]
    fn whitelist_contains() {
        let whitelist = Whitelist {
            members: vec![Addr::unchecked("terra12345"), Addr::unchecked("terra16789")],
        };

        assert!(whitelist.contains(&Addr::unchecked("terra12345")));
        assert!(!whitelist.contains(&Addr::unchecked("absent")));
    }
}
