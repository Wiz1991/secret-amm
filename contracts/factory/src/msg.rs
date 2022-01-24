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
