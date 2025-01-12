use std::collections::HashMap;

use gltf_derive::Validate;
use serde_derive::{Deserialize, Serialize};
use serde_json::value::Value;

/// A set of primitives to be rendered.
///
/// A node can contain one or more meshes and its transform places the meshes in
/// the scene.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Mesh {
    #[serde(default, flatten)]
    pub others: HashMap<String, Value>,
}

/// Geometry to be rendered with the given material.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Primitive {
    #[serde(default, flatten)]
    pub others: HashMap<String, Value>,
}
