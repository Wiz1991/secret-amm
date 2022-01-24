use common::pair::{AssetsRaw, PairInitMsg as InitMsg, TokenRaw};
use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, InitResponse, Querier, StdResult, Storage,
};

use crate::{
    msg::QueryMsg,
    state::{config, config_read, Pair},
};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let assets_raw: AssetsRaw = [
        TokenRaw {
            contract_addr: deps.api.canonical_address(&msg.assets[0].contract_addr)?,
        },
        TokenRaw {
            contract_addr: deps.api.canonical_address(&msg.assets[1].contract_addr)?,
        },
    ];

    config(&mut deps.storage).save(&Pair {
        assets: assets_raw,
        contract_addr: deps.api.canonical_address(&env.contract.address)?,
    })?;

    Ok(InitResponse::default())
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Pair {} => to_binary(&query_pair(&deps)?),
        QueryMsg::Pool {} => todo!(),
    }
}

pub fn query_pair<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<Pair> {
    config_read(&deps.storage).load()
}

#[cfg(test)]
mod tests {
    use common::pair::{PairInitMsg, Token};
    use cosmwasm_std::{
        from_binary,
        testing::{mock_dependencies, mock_env, MOCK_CONTRACT_ADDR},
        HumanAddr,
    };

    use crate::{msg::QueryMsg, state::Pair};

    use super::{init, query};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(20, &[]);
        let env = mock_env("creator", &[]);

        let assets = [
            Token {
                contract_addr: HumanAddr::from(MOCK_CONTRACT_ADDR),
            },
            Token {
                contract_addr: HumanAddr::from(MOCK_CONTRACT_ADDR),
            },
        ];
        let msg = PairInitMsg {
            assets: assets.clone(),
        };

        let _res = init(&mut deps, env, msg).unwrap();
        let msg = QueryMsg::Pair {};

        let res = query(&deps, msg).unwrap();
        let _value: Pair = from_binary(&res).unwrap();
    }
}
