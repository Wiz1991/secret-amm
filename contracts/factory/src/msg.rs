use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::Assets;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub pair_code_id: String,
}

pub enum HandleMsg {
    CreatePair { assets: Assets },
}

//should seaprate into common package
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PairInitMsg {
    pub assets: Assets,
}
