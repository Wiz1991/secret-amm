use core::fmt;

use cosmwasm_std::{Api, CanonicalAddr, HumanAddr, StdResult, Storage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

const CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct State {
    pub pair_code_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Token {
    pub contract_addr: HumanAddr,
}

impl Token {
    pub fn to_raw<A: Api>(&self, api: A) -> StdResult<TokenRaw> {
        Ok(TokenRaw {
            contract_addr: api.canonical_address(&self.contract_addr)?,
        })
    }
}
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.contract_addr)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenRaw {
    pub contract_addr: CanonicalAddr,
}

impl TokenRaw {
    pub fn as_bytes(&self) -> &[u8] {
        self.contract_addr.as_slice()
    }
}

pub type Assets = [Token; 2];
pub type AssetsRaw = [TokenRaw; 2];

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PairInfo {
    pub id: Vec<u8>,
    pub assets: Assets,
}

impl PairInfo {
    pub fn create_id(assets: &AssetsRaw) -> Vec<u8> {
        let assets = assets.to_vec();

        [assets[0].as_bytes(), assets[1].as_bytes()].concat()
    }
}

pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
    singleton_read(storage, CONFIG_KEY)
}
