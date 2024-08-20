mod entities;
mod entity;
mod game;
mod render;
mod utils;


use std::collections::HashMap;
use std::{env, fs};
use std::hash::Hash;
use std::path::Path;
use crate::entities::{enemy, player};
use crate::entity::{Entity};
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
use crate::render::{AssetData, ResourceLocation};

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
    let background_test = texture_creator.load_texture("./assets/game/sprites/bgrnd.png").unwrap();

    // initialise textures - new
    info!("Loading textures...");
    let mut textures : HashMap<String, Texture> = HashMap::new();
    for dir in WalkDir::new(".\\assets\\") {
        let path = String::from(dir.unwrap().path().to_str().unwrap());
        if path.clone().ends_with(".png") {
            let mut rl = ResourceLocation::empty();
            let split : Vec<_> = path.split("\\").collect();
            let namespace = &split[2];
            rl.set_namespace(namespace.to_string());
            let path = path.split(format!("\\{}\\", namespace).as_str()).collect::<Vec<_>>()[1];
            rl.set_path(path.to_string());
            let texture = texture_creator.load_texture(format!(".\\assets\\{}\\{}", namespace, path).as_str());
            textures.insert(rl.clone().to_string(), texture.unwrap());
        }
    }
    info!("Textures loaded!");

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

    let mut delta: f32 = 0.0;

    info!("Game instance initiated!");

    while game.running {
        canvas.clear();

        let start = Instant::now();

        canvas
            .copy_ex(
                &background_test,
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
