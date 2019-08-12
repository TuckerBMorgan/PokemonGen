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

    let textures = vec![engine.texture_create(include_bytes!("../PokemonSprites/0.png").as_ref(), TextureFormat::PNG),
                        engine.texture_create(include_bytes!("../PokemonSprites/42.png").as_ref(), TextureFormat::PNG),
                        engine.texture_create(include_bytes!("../PokemonSprites/62.png").as_ref(), TextureFormat::PNG),
                        engine.texture_create(include_bytes!("../PokemonSprites/43.png").as_ref(), TextureFormat::PNG),
                        engine.texture_create(include_bytes!("../PokemonSprites/32.png").as_ref(), TextureFormat::PNG),
                        engine.texture_create(include_bytes!("../PokemonSprites/35.png").as_ref(), TextureFormat::PNG),
                        engine.texture_create(include_bytes!("../PokemonSprites/23.png").as_ref(), TextureFormat::PNG),
                        engine.texture_create(include_bytes!("../PokemonSprites/55.png").as_ref(), TextureFormat::PNG),
                        engine.texture_create(include_bytes!("../PokemonSprites/57.png").as_ref(), TextureFormat::PNG),
                        engine.texture_create(include_bytes!("../PokemonSprites/58.png").as_ref(), TextureFormat::PNG),
                        engine.texture_create(include_bytes!("../PokemonSprites/79.png").as_ref(), TextureFormat::PNG),
                        engine.texture_create(include_bytes!("../PokemonSprites/81.png").as_ref(), TextureFormat::PNG),
                        engine.texture_create(include_bytes!("../PokemonSprites/76.png").as_ref(), TextureFormat::PNG),
                        engine.texture_create(include_bytes!("../PokemonSprites/78.png").as_ref(), TextureFormat::PNG),
                        engine.texture_create(include_bytes!("../PokemonSprites/77.png").as_ref(), TextureFormat::PNG),
                        engine.texture_create(include_bytes!("../PokemonSprites/80.png").as_ref(), TextureFormat::PNG),];

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

        

        for x in 0..20 {
            for y in 0..18 {
               overworld.sprites[x * 18 + y].texture = textures[overworld.map_tiles[x][y].get_top_tile_slot().texture_index()];
            }
        }

        engine.sprite_set(&screen, &overworld.sprites);

        engine.window_commit();
        clock.tick();
    }
}
