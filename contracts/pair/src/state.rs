use common::pair::Pair;
use cosmwasm_std::Storage;
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

pub static CONFIG_KEY: &[u8] = b"pool";

pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, Pair> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, Pair> {
    singleton_read(storage, CONFIG_KEY)
}
