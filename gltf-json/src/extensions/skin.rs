use std::collections::HashMap;

use gltf_derive::Validate;
use serde_derive::{Deserialize, Serialize};
use serde_json::value::Value;

/// Joints and matrices defining a skin.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Skin {
    #[serde(default, flatten)]
    pub others: HashMap<String, Value>,
}
