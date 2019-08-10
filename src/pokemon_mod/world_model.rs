use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WorldModel {
    width: usize,
    height: usize,
    sprite_indexes: Vec<usize>
}

