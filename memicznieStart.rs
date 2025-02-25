use cosmwasm_std::{entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128};
use cw20::{Cw20ExecuteMsg, Cw20QueryMsg, Cw20ReceiveMsg, TokenInfoResponse};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub initial_supply: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: Uint128,
    pub balances: std::collections::HashMap<String, Uint128>,
}

static mut STATE: Option<State> = None;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let mut balances = std::collections::HashMap::new();
    balances.insert(info.sender.to_string(), msg.initial_supply);

    let state = State {
        name: msg.name,
        symbol: msg.symbol,
        decimals: msg.decimals,
        total_supply: msg.initial_supply,
        balances,
    };

    unsafe {
        STATE = Some(state);
    }

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: Cw20ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        Cw20ExecuteMsg::Transfer { recipient, amount } => transfer(deps, info, recipient, amount),
        _ => unimplemented!(),
    }
}

fn transfer(
    deps: DepsMut,
    info: MessageInfo,
    recipient: String,
    amount: Uint128,
) -> StdResult<Response> {
    unsafe {
        if let Some(state) = &mut STATE {
            let sender_balance = state.balances.get(&info.sender.to_string()).cloned().unwrap_or(Uint128::zero());
            if sender_balance < amount {
                return Err(cosmwasm_std::StdError::generic_err("Insufficient funds"));
            }
            
            state.balances.insert(info.sender.to_string(), sender_balance - amount);
            let recipient_balance = state.balances.get(&recipient).cloned().unwrap_or(Uint128::zero());
            state.balances.insert(recipient, recipient_balance + amount);

            Ok(Response::new().add_attribute("action", "transfer"))
        } else {
            Err(cosmwasm_std::StdError::generic_err("State not initialized"))
        }
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: Cw20QueryMsg) -> StdResult<Binary> {
    match msg {
        Cw20QueryMsg::TokenInfo {} => to_binary(&query_token_info(deps)?),
        _ => unimplemented!(),
    }
}

fn query_token_info(_deps: Deps) -> StdResult<TokenInfoResponse> {
    unsafe {
        if let Some(state) = &STATE {
            Ok(TokenInfoResponse {
                name: state.name.clone(),
                symbol: state.symbol.clone(),
                decimals: state.decimals,
                total_supply: state.total_supply,
            })
        } else {
            Err(cosmwasm_std::StdError::generic_err("State not initialized"))
        }
    }
}
