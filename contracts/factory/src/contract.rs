use crate::{
    msg::{HandleMsg, InitMsg, QueryMsg},
    state::{config, config_read, State},
};
use common::pair::{AssetMeta, Pair, PairInitMsg};
use cosmwasm_std::{
    log, to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdError,
    StdResult, Storage, WasmMsg,
};
use cosmwasm_storage::{PrefixedStorage, ReadonlyPrefixedStorage};
use secret_toolkit::storage::{AppendStore, AppendStoreMut};
pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State {
        pair_code_id: msg.pair_code_id,
    };

    config(&mut deps.storage).save(&state)?;

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::CreatePair { assets_meta } => try_handle_create_pair(deps, env, assets_meta),
    }
}
pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::Pair { assets_meta } => to_binary(&query_pair(deps, assets_meta)?),
    }
}

pub fn try_handle_create_pair<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    assets: [AssetMeta; 2],
) -> StdResult<HandleResponse> {
    let config = config_read(&deps.storage).load()?;

    let mut pairs_store: PrefixedStorage<S> =
        PrefixedStorage::multilevel(&[b"pairs"], &mut deps.storage);
    let mut pairs_store: AppendStoreMut<Pair, PrefixedStorage<S>> =
        AppendStoreMut::attach_or_create(&mut pairs_store)?;

    if let Some(_) = pairs_store
        .iter()
        .rev()
        .find(|x| x.as_ref().unwrap().assets == assets)
    {
        return Err(StdError::generic_err("Pair already exists"));
    }
    let pair = Pair::new(&assets, env.contract.address.clone(), deps.api);

    pairs_store.push(&pair)?;

    let init_msg = WasmMsg::Instantiate {
        code_id: config.pair_code_id,
        callback_code_hash: env.contract_code_hash,
        send: vec![],
        label: format!("pair-{}-{}", pair.assets[0], pair.assets[1]),
        msg: to_binary(&PairInitMsg {
            assets_meta: assets.clone(),
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

pub fn query_config<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<State> {
    config_read(&deps.storage).load()
}
pub fn query_pair<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    assets: [AssetMeta; 2],
) -> StdResult<Pair> {
    let pairs_store: ReadonlyPrefixedStorage<S> =
        ReadonlyPrefixedStorage::multilevel(&[b"pairs"], &deps.storage);
    let pairs_store = AppendStore::<Pair, _, _>::attach(&pairs_store)
        .unwrap_or_else(|| return Err(StdError::generic_err("No pairs created")))?;

    let pair = pairs_store
        .iter()
        .rev()
        .find(|x| x.as_ref().unwrap().assets == assets);

    if let Some(pair) = pair {
        Ok(pair?)
    } else {
        return Err(StdError::not_found("Pair not found"));
    }
}

#[cfg(test)]
mod tests {
    use common::pair::AssetMeta;
    use cosmwasm_std::{
        from_binary,
        testing::{mock_dependencies, mock_env, MOCK_CONTRACT_ADDR},
        HumanAddr,
    };

    use super::*;

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(20, &[]);

        let msg = InitMsg {
            pair_code_id: 23123123,
        };
        let env = mock_env("creator", &[]);

        let _res = init(&mut deps, env, msg).unwrap();
    }

    #[test]
    fn create_pair() {
        let mut deps = mock_dependencies(20, &[]);

        let msg = InitMsg {
            pair_code_id: 23123123,
        };
        let env = mock_env("creator", &[]);

        let _res = init(&mut deps, env.clone(), msg).unwrap();
        let assets_meta = [
            AssetMeta::Token {
                contract_addr: HumanAddr::from(MOCK_CONTRACT_ADDR),
            },
            AssetMeta::Token {
                contract_addr: HumanAddr::from(MOCK_CONTRACT_ADDR),
            },
        ];
        let msg = HandleMsg::CreatePair {
            assets_meta: assets_meta.clone(),
        };

        let _res = handle(&mut deps, env, msg).unwrap();

        let msg = QueryMsg::Pair {
            assets_meta: assets_meta.clone(),
        };

        let res = query(&deps, msg).unwrap();
        let value: Pair = from_binary(&res).unwrap();
        println!("{:?}", value);
        assert_eq!(assets_meta[0], value.assets[0]);
    }
}
