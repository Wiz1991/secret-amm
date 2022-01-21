use cosmwasm_std::{Api, Env, Extern, HandleResponse, InitResponse, Querier, StdResult, Storage};

use crate::msg::{HandleMsg, InitMsg, Token};

pub fn init<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>,
    _env: Env,
    _msg: InitMsg,
) -> StdResult<InitResponse> {
    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::CreatePair { assets } => try_handle_create_pair(deps, env, assets),
    }
}

pub fn try_handle_create_pair<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>,
    _env: Env,
    _assets: [Token; 2],
) -> StdResult<HandleResponse> {


    Ok(HandleResponse::default())
}
