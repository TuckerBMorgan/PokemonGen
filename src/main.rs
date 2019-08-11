extern crate rand;
use storm::time::*;
use storm::*;
use storm::cgmath::*;
use std::ops::Add;
mod pokemon_mod;
use std::fs;

use pokemon_mod::*;

use rand::{
    Rng,
};

/// Run with: cargo run --example pokemon --release
fn main() {
    Engine::start(
        WindowSettings {
            title: String::from("Storm: Texture"),
            display_mode: DisplayMode::Windowed {
                width: 1280,
                height: 1024,
                resizable: true,
            },
            vsync: Vsync::Disabled,
        },
        game,
    );
}


fn game(mut engine: Engine) {

    let pokemon_1 = Pokemon::new(10, String::from("Foo"), 1, [BattleMove::new(10, String::from("HEALTH"), true, 1)]);
    let pokemon_2 = Pokemon::new(10, String::from("Bar"), 1, [BattleMove::new(10, String::from("HEALTH"), true, 1)]);
    let mut battle = Battle::new([pokemon_1, pokemon_2]);

    let mut clock = Clock::new(144);

    let mut world  = [[Tile::new(); 30];30];

    let textures = vec![engine.texture_create(include_bytes!("../PokemonSprites/36.png").as_ref(), TextureFormat::PNG),
                        engine.texture_create(include_bytes!("../PokemonSprites/32.png").as_ref(), TextureFormat::PNG),
                        engine.texture_create(include_bytes!("../PokemonSprites/17.png").as_ref(), TextureFormat::PNG),
                        engine.texture_create(include_bytes!("../PokemonSprites/24.png").as_ref(), TextureFormat::PNG)];

    engine.window_clear_color(storm::color::BLACK);
    let screen = engine.batch_create(&BatchSettings::default());
    battle.tick_battle();
    let contents = fs::read_to_string("testworld.json")
        .expect("Something went wrong reading the file");
    let world_model = serde_json::from_str(&contents).unwrap();
    let mut overworld = Overworld::new(world_model);

    engine.sprite_set(&screen, &overworld.sprites);


    let mut is_active = true;
    while is_active {

        while let Some(message) = engine.input_poll() {
            match message {
                InputMessage::CloseRequested => is_active = false,
                InputMessage::KeyPressed(key) => {
                    match key {
                        KeyboardButton::Left => {
                        },
                        KeyboardButton::Right => {
                        },
                        KeyboardButton::Up => {
                        },
                        KeyboardButton::Down => {
                        }
                        KeyboardButton::Q => {
                        },
                        KeyboardButton::E => {
                        },

                        KeyboardButton::Escape => is_active = false,
                        _ => {},
                    }
                },
                InputMessage::KeyReleased(key) => match key {
                    KeyboardButton::Left => {
                    },
                    KeyboardButton::Right => {
                    },
                    KeyboardButton::Up => {
                    },
                    KeyboardButton::Down => {
                    },
                    _ => {

                    }
                },
                _ => {

                }
            }
        }

        

        for x in 0..30 {
            for y in 0..30 {
               overworld.sprites[x * 30 + y].texture = textures[overworld.map_tiles[x][y].get_top_tile_slot().texture_index()];
            }
        }

        engine.sprite_set(&screen, &overworld.sprites);
//    
        engine.window_commit();
        clock.tick();
    }
}
