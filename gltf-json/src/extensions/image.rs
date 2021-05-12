use std::collections::HashMap;

use gltf_derive::Validate;
use serde_derive::{Deserialize, Serialize};
use serde_json::value::Value;

/// Image data used to create a texture.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Image {
    #[serde(default, flatten)]
    pub others: HashMap<String, Value>,
}
