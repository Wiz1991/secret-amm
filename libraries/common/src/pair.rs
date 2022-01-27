use core::fmt;

use cosmwasm_std::{Api, CanonicalAddr, HumanAddr, StdResult, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct Asset {
    pub meta: AssetMeta,
    pub amount: Uint128,
}

impl fmt::Display for Asset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.meta, self.amount)
    }
}
impl Asset {
    pub fn to_raw<A: Api>(&self, api: A) -> StdResult<AssetRaw> {
        Ok(AssetRaw {
            meta: self.meta.to_raw(api)?,
            amount: self.amount,
        })
    }
}

impl PartialEq for Asset {
    fn eq(&self, other: &Self) -> bool {
        self.meta == other.meta
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AssetMeta {
    Token { contract_addr: HumanAddr },
}
impl AssetMeta {
    pub fn to_raw<A: Api>(&self, api: A) -> StdResult<AssetMetaRaw> {
        match self {
            AssetMeta::Token { contract_addr } => Ok(AssetMetaRaw::Token {
                contract_addr: api.canonical_address(contract_addr)?,
            }),
        }
    }
}
impl PartialEq for AssetMeta {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (AssetMeta::Token { contract_addr: a }, AssetMeta::Token { contract_addr: b }) => {
                a == b
            }
        }
    }
}

impl fmt::Display for AssetMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AssetMeta::Token { contract_addr } => write!(f, "{}", contract_addr),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AssetRaw {
    pub meta: AssetMetaRaw,
    pub amount: Uint128,
}

impl AssetRaw {
    pub fn to_human<A: Api>(&self, api: A) -> StdResult<Asset> {
        Ok(Asset {
            meta: self.meta.to_human(api)?,
            amount: self.amount,
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AssetMetaRaw {
    Token { contract_addr: CanonicalAddr },
}

impl AssetMetaRaw {
    pub fn to_human<A: Api>(&self, api: A) -> StdResult<AssetMeta> {
        match self {
            AssetMetaRaw::Token { contract_addr } => Ok(AssetMeta::Token {
                contract_addr: api.human_address(contract_addr)?,
            }),
        }
    }
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            AssetMetaRaw::Token { contract_addr } => contract_addr.as_slice(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct Pair {
    pub id: Vec<u8>,
    pub assets: [AssetMeta; 2],
    pub contract_addr: HumanAddr,
}

impl Pair {
    pub fn new<A: Api>(assets: &[AssetMeta; 2], contract_addr: HumanAddr, api: A) -> Self {
        let assets_raw = [
            assets[0].to_raw(api).unwrap(),
            assets[1].to_raw(api).unwrap(),
        ];
        let id = [assets_raw[0].as_bytes(), assets_raw[1].as_bytes()].concat();

        Self {
            id,
            assets: assets.clone(),
            contract_addr,
        }
    }
    pub fn to_raw<A: Api>(&self, api: A) -> StdResult<PairRaw> {
        Ok(PairRaw {
            id: self.id.clone(),
            assets: [self.assets[0].to_raw(api)?, self.assets[1].to_raw(api)?],
            contract_addr: api.canonical_address(&self.contract_addr)?,
        })
    }
}
impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.assets[0] == other.assets[0] && self.assets[1] == other.assets[1]
            || self.assets[0] == other.assets[1] && self.assets[1] == other.assets[0]
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PairRaw {
    pub id: Vec<u8>,
    pub assets: [AssetMetaRaw; 2],
    pub contract_addr: CanonicalAddr,
}

impl PairRaw {
    pub fn to_human<A: Api>(&self, api: A) -> StdResult<Pair> {
        Ok(Pair {
            id: self.id.clone(),
            assets: [self.assets[0].to_human(api)?, self.assets[1].to_human(api)?],
            contract_addr: api.human_address(&self.contract_addr)?,
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PairInitMsg {
    pub assets_meta: [AssetMeta; 2],
}
