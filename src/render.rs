use crate::{main, AssetData, TextureType};
use once_cell::sync::{Lazy, OnceCell};
use phf::{phf_map, Map};
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Mutex;

pub struct IconsContainer {
    pub icons: HashMap<&'static str, AssetData>,
}

pub fn get_icons() -> &'static Mutex<HashMap<&'static str, AssetData>> {
    static INSTANCE: OnceCell<Mutex<HashMap<&'static str, AssetData>>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert(
            "cursor",
            AssetData {
                UV: Option::from(Rect::new(0, 0, 16, 16)),
                Origin: (0, 0),
                texture_type: TextureType::icon,
            },
        );
        Mutex::new(m)
    })
}



pub fn draw_pp_texture(x: i32, y: i32, ass: &AssetData, mut canvas: &mut WindowCanvas, sf: i32) {
    let uv = ass.UV.unwrap();
    let tex_rect = Rect::new(x / sf, y / sf, uv.w as u32, uv.h as u32);

    canvas
        .set_scale(sf as f32, sf as f32)
        .expect("TODO: panic message");

    let texture_creator = canvas.texture_creator();
    let texture = {
        match ass.texture_type {
            TextureType::icon => {
                texture_creator
                    .load_texture("assets/gui/icons.png")
                    .unwrap()
            },
            TextureType::in_game_sprite => {
                texture_creator
                    .load_texture("assets/sprites/sprite.png")
                    .unwrap()
            }
            _ => texture_creator
                .load_texture("assets/missing.png")
                .unwrap(),
        }
    };
    canvas
        .copy_ex(&texture,
                 uv,
                 tex_rect,
                 0.0,
                 None,
                 false,
                 false,
        )
        .expect("TODO: panic message");
}



/*

this is the way I wanted to do textures; statically saving them as not to constantly recall the assets. I might use Bevy as a way to save the data better

pub fn get_missing_texture(texture_creator: TextureCreator<WindowContext>) -> &'static Result<Texture<'static>, String> {
    static INSTANCE: OnceCell<Result<Texture, String>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        texture_creator.load_texture("assets/missing.png")
    })
}
 */
/*
pub fn get_texture<'a>(tt : TextureType, canvas: WindowCanvas) -> Texture<'a> {
    let texture_creator = canvas.texture_creator();
    match tt {
        TextureType::icon => { let text = texture_creator.load_texture("assets/gui/icons.png").unwrap(); text}
        _ => { let text = texture_creator.load_texture("assets/missing.png").unwrap(); text }
    }
}

 */
