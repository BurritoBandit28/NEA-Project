mod render;

use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode::Mute;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator, WindowCanvas};
use sdl2::surface::Surface;
use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;

use sdl2::video::WindowContext;

pub struct AssetData {
    UV: Option<Rect>,
    Origin: (u32, u32),
    texture_type: TextureType,
}

pub enum TextureType {
    icon,
    in_game_sprite,
    idk,
}

pub const DIMENSIONS: (u32, u32) = (320, 180);

fn main() {
    let sdl_ctx = sdl2::init().unwrap();
    let scale_factor =
        sdl_ctx.video().unwrap().current_display_mode(0).unwrap().w / DIMENSIONS.0 as i32;
    let video_subsys = sdl_ctx.video().unwrap();

    sdl_ctx.mouse().show_cursor(false);

    let mut window = video_subsys
        .window("[Game name here]", DIMENSIONS.0, DIMENSIONS.1)
        .vulkan()
        .fullscreen_desktop()
        .build()
        .unwrap();

    &window.set_mouse_grab(true);

    let mut canvas = &mut window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();

    let background_test = texture_creator.load_texture("bgrnd2.png").unwrap();

    let icons_texture = texture_creator
        .load_texture("assets/gui/icons.png")
        .unwrap();

    let mut event_pump = sdl_ctx.event_pump().unwrap();
    let mut output_texture = texture_creator
        .create_texture_target(PixelFormatEnum::ARGB32, DIMENSIONS.0, DIMENSIONS.1)
        .unwrap();
    canvas.present();
    'running: loop {
        canvas.clear();
        canvas
            .copy_ex(&background_test, None, None, 0.0, None, false, false)
            .expect("TODO: panic message");
        for event in event_pump.poll_iter() {
            match event {
                _ => {}
            }
        }
        draw_mouse(
            event_pump.mouse_state().x(),
            event_pump.mouse_state().y(),
            &icons_texture,
            canvas,
            scale_factor,
        );

        canvas.present()
    }
}

// Pixel perfect mouse rendering
pub fn draw_mouse(x: i32, y: i32, icons: &Texture, canvas: &mut WindowCanvas, sf: i32) {
    let uv = &render::get_icons().lock().unwrap().get("cursor").unwrap().UV.unwrap();

    let mouse_rect = Rect::new(x / sf, y / sf, uv.w as u32, uv.h as u32);

    canvas
        .set_scale(sf as f32, sf as f32)
        .expect("TODO: panic message");

    canvas
        .copy_ex(icons, *uv, mouse_rect, 0.0, None, false, false)
        .expect("TODO: panic message");
}
