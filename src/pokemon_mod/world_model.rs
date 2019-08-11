use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WorldModel {
    pub height: usize,
    pub width: usize,
    pub sprite_indexes: Vec<Vec<usize>>
}