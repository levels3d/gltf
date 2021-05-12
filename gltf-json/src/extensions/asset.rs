use std::collections::HashMap;

use gltf_derive::Validate;
use serde_derive::{Deserialize, Serialize};
use serde_json::value::Value;

/// Metadata about the glTF asset.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Asset {
    #[serde(default, flatten)]
    pub others: HashMap<String, Value>,
}
