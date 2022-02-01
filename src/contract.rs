#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, LockResponse, QueryMsg, WhitelistResponse};
use crate::state;

const CONTRACT_NAME: &str = "crates.io:contract-mutex";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let lock = state::Lock {
        since_height: env.block.height,
        owner: None,
    };
    state::LOCK.save(deps.storage, &lock)?;

    let whitelist = state::Whitelist {
        members: msg.whitelist.clone(),
    };
    state::WHITELIST.save(deps.storage, &whitelist)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("whitelist", format!("{:?}", msg.whitelist)))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Lock {} => execute_lock(deps, env, info),
        ExecuteMsg::Unlock {} => execute_unlock(deps, env, info),
    }
}

/// Try to get the lock.
///
/// Fails if lock is already used by someone (including caller).
fn execute_lock(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let mut lock = state::LOCK.load(deps.storage)?;
    let whitelist = state::WHITELIST.load(deps.storage)?;

    // Only whitelisted addresses can lock.
    if !whitelist.contains(&info.sender) {
        return Err(ContractError::Unauthorized {
            message: "not whitelisted".to_string(),
        });
    }

    // Only if it's not already locked.
    if lock.is_locked() {
        return Err(ContractError::AlreadyLocked {
            address: info.sender,
        });
    }

    lock.since_height = env.block.height;
    lock.owner = Some(info.sender.clone());

    state::LOCK.save(deps.storage, &lock)?;

    Ok(Response::new()
        .add_attribute("method", "lock")
        .add_attribute("owner", info.sender)
        .add_attribute("since_height", env.block.height.to_string()))
}

/// Try to unlock the lock.
///
/// Fails if not locked or ender is not the owner of the lock.
fn execute_unlock(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let mut lock = state::LOCK.load(deps.storage)?;

    // Only if locked.
    if !lock.is_locked() {
        return Err(ContractError::AlreadyUnlocked);
    }

    // Only if owner of the lock.
    if info.sender != lock.owner.unwrap() {
        return Err(ContractError::Unauthorized {
            message: "not owner of the lock".to_string(),
        });
    }

    lock.since_height = env.block.height;
    lock.owner = None;

    state::LOCK.save(deps.storage, &lock)?;

    Ok(Response::new()
        .add_attribute("method", "unlock")
        .add_attribute("owner", "none")
        .add_attribute("since_height", env.block.height.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Lock {} => to_binary(&query_lock(deps)?),
        QueryMsg::Whitelist {} => to_binary(&query_whitelist(deps)?),
    }
}

/// Query the current lock.
pub fn query_lock(deps: Deps) -> StdResult<LockResponse> {
    let lock = state::LOCK.load(deps.storage)?;

    Ok(LockResponse {
        since_height: lock.since_height,
        owner: lock.owner,
    })
}

/// Query the whitelist.
pub fn query_whitelist(deps: Deps) -> StdResult<WhitelistResponse> {
    let whitelist = state::WHITELIST.load(deps.storage)?;

    Ok(WhitelistResponse {
        members: whitelist.members,
    })
}
