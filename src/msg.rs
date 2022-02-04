use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Contract instantiation message.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    /// Initial whitelisted addresses authorized to use the lock.
    pub whitelist: Vec<Addr>,
}

/// Contract execution messages.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Try locking the mutex.
    Lock {},

    /// Release the mutex lock.
    Unlock {},
}

/// Contract query messages.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Get current lock information.
    Lock {},

    /// Get whitelisted addresses.
    Whitelist {},
}

/// Deposited balance response.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LockResponse {
    pub since_height: u64,
    pub owner: Option<Addr>,
}

/// Whitelist response.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WhitelistResponse {
    pub members: Vec<Addr>,
}
