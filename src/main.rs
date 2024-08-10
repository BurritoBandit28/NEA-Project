mod render;
mod game;
mod entity;
mod entities;

use std::time::Instant;
use sdl2::image::LoadTexture;
use render::DIMENSIONS;
use crate::entities::{enemy, player};
use crate::entity::{Entity, EntityTest};
use crate::game::Game;

fn main() {

    // initial set up

    // start SDL2
    let sdl_ctx = sdl2::init().unwrap();
    let scale_factor =
        sdl_ctx.video().unwrap().current_display_mode(0).unwrap().w / DIMENSIONS.0 as i32;
    let video_subsys = sdl_ctx.video().unwrap();

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
    let background_test = texture_creator.load_texture("assets/bgrnd.png").unwrap();

    // initiallise textures
    let textures = vec![
                        texture_creator.load_texture("assets/gui/icons.png").unwrap(),
                        texture_creator.load_texture("assets/sprites/sprite.png").unwrap(),
                        texture_creator.load_texture("assets/missing.png").unwrap()
    ];

    let mut event_pump = sdl_ctx.event_pump().unwrap();

    canvas.present();

    let mut game = Game::initiate();
    // test entities
    //EntityTest::create_player(&mut game);
    //EntityTest::create_obj(&mut game, (-30f32, 70f32));
    player::Player::create(&mut game);
    enemy::Enemy::create(&mut game);

    let _ = game.mobiles.get_mut(0).unwrap().lock().unwrap().set_coords((10.0, 10.0));

    let mut delta : f32 = 0.0;

    'running: loop {
        canvas.clear();

        let start = Instant::now();

        canvas
            .copy_ex(&background_test, None, None, 0.0, None, false, false)
            .expect("TODO: panic message");
        for event in event_pump.poll_iter() {
            match event {
                _ => {}
            }
        }

        game.cycle(delta);

        unsafe { game.render(canvas, scale_factor, &textures); }

        ///TODO
        /// gameplay_loop(event_pump)

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

fn world_space_screen_space_test() {

    let mut game = Game::initiate();
    EntityTest::create_player(&mut game);
    EntityTest::create_obj(&mut game, (-80f32, 0f32));
    let out = game.mobiles_old.get(1).unwrap().screen(game.mobiles_old.get(0).unwrap());
    println!("{:?}", out);
}
