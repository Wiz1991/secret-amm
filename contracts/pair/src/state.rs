use common::pair::AssetsRaw;
use cosmwasm_std::{CanonicalAddr, Storage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub static CONFIG_KEY: &[u8] = b"pool";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Pair {
    pub assets: AssetsRaw,
    pub contract_addr: CanonicalAddr,
}
pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, Pair> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, Pair> {
    singleton_read(storage, CONFIG_KEY)
}
