mod entities;
mod entity;
mod game;
mod render;
mod utils;

use std::collections::HashMap;
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
use sdl2::keyboard::Keycode::Hash;
use sdl2::render::Texture;
use crate::render::AssetData;

fn main() {
    // initial set up

    // start logger
    utils::init_logger();

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

    // lock mouse to window
    &window.set_mouse_grab(true);

    //create canvas
    let mut canvas = &mut window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let background_test = texture_creator.load_texture("./assets/bgrnd.png").unwrap();

    // initiallise textures - old
    let textures_old = vec![
        texture_creator
            .load_texture("assets/gui/icons.png")
            .unwrap(),
        texture_creator
            .load_texture("assets/sprites/sprite.png")
            .unwrap(),
        texture_creator.load_texture("assets/missing.png").unwrap(),
    ];

    // initialise textures - new
    info!("Loading textures...");
    let textures : HashMap<String,HashMap<String, Texture>> = HashMap::new();


    let mut event_pump = sdl_ctx.event_pump().unwrap();

    canvas.present();

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

    'running: loop {
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
            println!("{:?}", game.held_keys)
        }

        game.pressed_keys = vec![];
        for event in event_pump.poll_iter() {
            game.pressed_keys.push(event.clone());
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                _ => {}
            }
        }

        game.cycle(delta);

        unsafe {
            game.render(canvas, scale_factor, &textures_old);
        }

        //TODO
        // gameplay_loop(event_pump)
        render::draw_pp_texture(
            event_pump.mouse_state().x() / scale_factor,
            event_pump.mouse_state().y() / scale_factor,
            &render::get_icons().lock().unwrap().get("cursor").unwrap(),
            canvas,
            scale_factor,
            &textures_old,
        );

        canvas.present();
        delta = start.elapsed().as_secs_f32();
    }
}
