use crate::{
    msg::{HandleMsg, InitMsg, PairInitMsg},
    state::{config_read, Assets, PairInfo},
};
use cosmwasm_std::{
    log, to_binary, Api, Env, Extern, HandleResponse, InitResponse, Querier, StdError, StdResult,
    Storage, WasmMsg,
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
    env: Env,
    assets: Assets,
) -> StdResult<HandleResponse> {
    let config = config_read(&deps.storage).load()?;

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

    pairs_store.push(&PairInfo {
        assets: assets.clone(),
        id,
    })?;

    let init_msg = WasmMsg::Instantiate {
        code_id: config.pair_code_id,
        callback_code_hash: env.contract_code_hash,
        send: vec![],
        label: "".to_string(),
        msg: to_binary(&PairInitMsg {
            assets: assets.clone(),
        })?,
    };

    Ok(HandleResponse {
        messages: vec![init_msg.into()],
        log: vec![
            log("action", "create_pair"),
            log("pair", format!("{}-{}", &assets[0], &assets[1])),
        ],
        data: None,
    })
}
