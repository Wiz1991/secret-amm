use common::pair::{AssetMeta, Pair, PairInitMsg as InitMsg};
use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, HumanAddr, InitResponse, Querier,
    StdError, StdResult, Storage,
};

use crate::{
    msg::{HandleMsg, QueryMsg},
    state::{config, config_read},
};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let pair = Pair::new(&msg.assets_meta, env.contract.address, deps.api);

    config(&mut deps.storage).save(&pair)?;

    Ok(InitResponse::default())
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Pair {} => to_binary(&query_pair(&deps)?),
        QueryMsg::Pool {} => to_binary(&query_pool(&deps)?),
    }
}

pub fn query_pair<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<Pair> {
    config_read(&deps.storage).load()
}

pub fn query_pool<S: Storage, A: Api, Q: Querier>(_deps: &Extern<S, A, Q>) -> StdResult<String> {
    Ok(String::from("test"))
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    _deps: &Extern<S, A, Q>,
    _env: Env,
    _msg: HandleMsg,
) -> StdResult<HandleResponse> {
    Ok(HandleResponse::default())
}

pub fn try_add_liquidity<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    _assets: [AssetMeta; 2],
    _receiver: Option<HumanAddr>,
) -> StdResult<HandleResponse> {
    let _config = config_read(&deps.storage).load()?;

    Ok(HandleResponse::default())
}

#[cfg(test)]
mod tests {
    use common::pair::{AssetMeta, Pair, PairInitMsg};
    use cosmwasm_std::{
        from_binary,
        testing::{mock_dependencies, mock_env, MOCK_CONTRACT_ADDR},
        HumanAddr,
    };

    use crate::msg::QueryMsg;

    use super::{init, query};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(20, &[]);
        let env = mock_env("creator", &[]);

        let assets_meta = [
            AssetMeta::Token {
                contract_addr: HumanAddr::from(MOCK_CONTRACT_ADDR),
            },
            AssetMeta::Token {
                contract_addr: HumanAddr::from(MOCK_CONTRACT_ADDR),
            },
        ];
        let msg = PairInitMsg {
            assets_meta: assets_meta.clone(),
        };

        let _res = init(&mut deps, env, msg).unwrap();
        let msg = QueryMsg::Pair {};

        let res = query(&deps, msg).unwrap();
        let _value: Pair = from_binary(&res).unwrap();
    }
}
