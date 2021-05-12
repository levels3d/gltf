use std::collections::HashMap;

use gltf_derive::Validate;
use serde_derive::{Deserialize, Serialize};
use serde_json::value::Value;

/// A buffer points to binary data representing geometry, animations, or skins.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Buffer {
    #[serde(default, flatten)]
    pub others: HashMap<String, Value>,
}

/// A view into a buffer generally representing a subset of the buffer.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct View {
    #[serde(default, flatten)]
    pub others: HashMap<String, Value>,
}
