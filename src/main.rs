mod entities;
mod entity;
mod game;
mod render;
mod utils;
mod level;
mod tile;
mod resource_location;

use std::collections::HashMap;
use std::{env, fs};
use std::hash::Hash;
use std::path::Path;
use crate::entities::{enemy, player};
use crate::entity::Entity;
use crate::game::Game;
use num::clamp;
use render::DIMENSIONS;
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::sys::KeyPress;
use std::time::Instant;
use log::info;
use sdl2::event::Event::KeyDown;
use sdl2::render::Texture;
use walkdir::WalkDir;
use resource_location::ResourceLocation;
use crate::level::Level;
use crate::render::AssetData;
use crate::tile::{Tile, TileSize, TileType};

fn main() {

    // initial set up

    // start logger
    utils::init_logger();

    info!("Initialising SDL2");

    // start SDL2
    let sdl_ctx = sdl2::init().unwrap();
    let scale_factor =
        sdl_ctx.video().unwrap().current_display_mode(0).unwrap().w / DIMENSIONS.0 as i32;
    let video_subsys = sdl_ctx.video().unwrap();

    let scale_offset = (sdl_ctx.video().unwrap().current_display_mode(0).unwrap().h / scale_factor as i32) - 180;
    let half_scale_offset = scale_offset / 2;
    //todo : get the difference in height and display 2 black bars to give 16:9 ratio screen

    //hide mouse
    sdl_ctx.mouse().show_cursor(false);

    // create window
    let mut window = video_subsys
        .window("[Game name here]", DIMENSIONS.0, DIMENSIONS.1)
        .vulkan()
        .fullscreen_desktop()
        .build()
        .unwrap();

    info!("complete");

    // lock mouse to window
    &window.set_mouse_grab(true);

    //create canvas
    let mut canvas = &mut window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let background_test = texture_creator.load_texture("./assets/game/bgrnd.png").unwrap();

    // counter to count how many objects are loaded for the debug logs
    let mut counter = 0;

    // initialise textures - new
    info!("Loading textures...");
    // create hashmap
    let mut textures : HashMap<String, Texture> = HashMap::new();
    // iterate through the assets directory
    for dir in WalkDir::new(".\\assets\\") {
        let path = String::from(dir.unwrap().path().to_str().unwrap());
        // if the file is an image, save it - in future there will likely be a hashmap for other files, like animation data or other bits idk yet
        if path.clone().to_lowercase().ends_with(".png") {
            // create the resource location
            let mut rl = ResourceLocation::empty();

            // split the path by \s
            let split : Vec<_> = path.split("\\").collect();

            // name space is in ./assets/>>namespace<<, so it is the third element in the list
            let namespace = &split[2];
            rl.set_namespace(namespace.to_string());

            // the path is just everything after the namespace
            let path = path.split(format!("\\{}\\", namespace).as_str()).collect::<Vec<_>>()[1];
            rl.set_path(path.to_string());

            //load the texture
            let texture = texture_creator.load_texture(format!(".\\assets\\{}\\{}", namespace, path).as_str());

            // insert the hashmap
            textures.insert(rl.clone().to_string(), texture.unwrap());

            info!("Loaded texture : {}", rl.to_string());
            counter+=1;
        }
    }
    info!("{} textures loaded!", counter);

    counter = 0;
    // entirely data driven tile system

    // initialise tiles
    info!("Loading tiles...");

    //create hashmap
    let mut tiles: HashMap<String, Tile> = HashMap::new();
    // get the immediate subdirectories for the name spaces
    let namespaces = fs::read_dir(".\\data").unwrap();

    // iterate through the namespaces
    for namepath in namespaces {

        // get the actual namespace
        let mut namespace = String::from(namepath.unwrap().path().to_str().unwrap());
        //                                            .\data\>>namespace<<
        namespace = namespace.split("\\").collect::<Vec<_>>()[2].to_string();

        if !namespace.clone().contains(".") {

            for dir in WalkDir::new(format!(".\\data\\{}\\tiles\\", namespace.clone())) {
                let path = String::from(dir.unwrap().path().to_str().unwrap());
                // if the file is tile data, continue
                if path.clone().to_lowercase().ends_with(".json") {
                    // get the json file as a string
                    let json = fs::read_to_string(path.clone()).unwrap();

                    // read the values from the json file
                    let name = gjson::get(json.as_str(), "name");

                    let resource_location = ResourceLocation::new(
                        &*namespace.clone(),
                        path.split(format!("\\{}\\", namespace).as_str()).collect::<Vec<_>>()[1]);

                    let texture = ResourceLocation::parse(
                        gjson::get(json.as_str(), "resource_location")
                        .to_string());

                    let uv : (u32, u32) = (
                        gjson::get(json.as_str(), "uv.x").to_string().parse::<u32>().unwrap(),
                        gjson::get(json.as_str(), "uv.y").to_string().parse::<u32>().unwrap()
                    );

                    let ttype = TileType::parse(gjson::get(json.as_str(), "type").to_string());

                    let size = TileSize::parse(gjson::get(json.as_str(), "size").to_string().as_str());


                    let origin : (i32, i32) = (
                        gjson::get(json.as_str(), "origin.x").to_string().parse::<i32>().unwrap(),
                        gjson::get(json.as_str(), "origin.y").to_string().parse::<i32>().unwrap()
                    );

                    let collision : bool = gjson::get(json.as_str(), "collision").to_string().parse::<bool>().unwrap();
                    let mut collison_box : Option<(u32,u32)>;

                    if collision {
                        collison_box = Some((
                            gjson::get(json.as_str(), "collision_box.x").to_string().parse::<u32>().unwrap(),
                            gjson::get(json.as_str(), "collision_box.y").to_string().parse::<u32>().unwrap()
                        ));
                    }
                    else {
                        collison_box = None
                    }

                    let tile = Tile::create(name.to_string(), resource_location.clone(), texture, uv, ttype, size, origin, collision, collison_box);
                    tiles.insert(resource_location.to_string(), tile);

                    info!("Loaded tile : {}", resource_location.to_string());

                    counter += 1;
                }

            }
        }
    }

    info!("{} tiles loaded!", counter);

    let mut event_pump = sdl_ctx.event_pump().unwrap();

    canvas.present();

    info!("Initiating game controller");

    let mut game = Game::initiate();
    // test entities
    //EntityTest::create_player(&mut game);
    //EntityTest::create_obj(&mut game, (-30f32, 70f32));
    player::Player::create(&mut game);
    enemy::Enemy::create(&mut game);

    let _ = game
        .mobiles
        .get_mut(0)
        .unwrap()
        .lock()
        .unwrap()
        .set_coords((10.0, 10.0));

    let _ = game
        .mobiles
        .get_mut(1)
        .unwrap()
        .lock()
        .unwrap()
        .set_coords((20.0, 20.0));

    let mut delta: f32 = 0.0;

    // load the test level
    game.current_level = Some(Level::create_test_level(&tiles));

    info!("Game instance initiated!");

    while game.running {
        canvas.clear();

        let start = Instant::now();

        canvas
            .copy_ex(
                &textures.get("game:background.png").unwrap(),
                None,
                Rect::new(0, half_scale_offset, 320, 180),
                0.0,
                None,
                false,
                false,
            )
            .expect("TODO: panic message");


        game.held_keys = vec![];
        for key in event_pump.keyboard_state().pressed_scancodes() {
            game.held_keys.push(key);
        }

        game.events = vec![];
        for event in event_pump.poll_iter() {
            game.events.push(event.clone());
        }

        game.cycle(delta);

        unsafe {
            game.render(canvas, scale_factor, &textures);
        }

        //TODO
        // gameplay_loop(event_pump)
        render::draw_pp_texture(
            event_pump.mouse_state().x() / scale_factor,
            event_pump.mouse_state().y() / scale_factor,
            &render::get_icons().lock().unwrap().get("cursor").unwrap(),
            canvas,
            scale_factor,
            &textures
        );

        canvas.present();
        delta = start.elapsed().as_secs_f32();
    }
}
