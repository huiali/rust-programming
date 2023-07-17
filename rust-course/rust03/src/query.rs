use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct IdQuery {
    pub id: String,
    pub m: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RIdQuery {
    pub rid: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ComponetQuery {
    pub id: String,
    pub component_id: String,
}
