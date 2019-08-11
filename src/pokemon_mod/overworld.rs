use crate::pokemon_mod::{WorldModel, Tile};
use storm::*;

pub struct Overworld {
    pub map_tiles: Vec<Vec<Tile>>,
    pub sprites: Vec<Sprite>

}

impl Overworld {
    pub fn new(world_model: WorldModel) -> Overworld {

        let mut map_tiles = vec![];
        for x in 0..world_model.width {
            map_tiles.push(vec![]);
            for y in 0..world_model.height {
                map_tiles[x].push(Tile::from_usize(world_model.sprite_indexes[x][y]));
            }
        }

        let mut sprites = vec![];

        let mut sprite = Sprite::default();
    //    sprite.texture = textures[0];
        sprite.size.x = sprite.size.x / 5;
        sprite.size.y = sprite.size.y / 5;


        for x in 0..30 {
            for y in 0..30 {
                sprite.pos.x = (x * 20) as f32 - 100.0f32;
                sprite.pos.y = (y * 20) as f32 - 400.0f32;          
                sprite.color = storm::color::WHITE;;
                sprites.push(sprite);
            }
        }
        

        Overworld {
            map_tiles,
            sprites
        }
    }
}