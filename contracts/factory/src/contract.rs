use crate::{
    msg::{HandleMsg, InitMsg},
    state::{config_read, Assets, PairInfo},
};
use cosmwasm_std::{
    Api, Env, Extern, HandleResponse, InitResponse, Querier, StdError, StdResult, Storage,
};
use cosmwasm_storage::PrefixedStorage;
use secret_toolkit::storage::AppendStoreMut;
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
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    assets: Assets,
) -> StdResult<HandleResponse> {
    let _config = config_read(&deps.storage);

    let mut pairs_store: PrefixedStorage<S> =
        PrefixedStorage::multilevel(&[b"pairs"], &mut deps.storage);
    let mut pairs_store: AppendStoreMut<PairInfo, PrefixedStorage<S>> =
        AppendStoreMut::attach_or_create(&mut pairs_store)?;

    if let Some(_) = pairs_store
        .iter()
        .rev()
        .find(|x| x.as_ref().unwrap().assets == assets)
    {
        return Err(StdError::generic_err("Pair already exists"));
    }

    let assets_raw = [assets[0].to_raw(deps.api)?, assets[1].to_raw(deps.api)?];
    let id = PairInfo::create_id(&assets_raw);

    pairs_store.push(&PairInfo { assets, id })?;

    Ok(HandleResponse::default())
}
