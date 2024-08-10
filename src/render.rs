use once_cell::sync::OnceCell;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use std::collections::HashMap;
use std::iter::Copied;
use std::sync::Mutex;
use sdl2::pixels::Color;

pub const DIMENSIONS: (u32, u32) = (320, 180);

// todo, make clone implementation
pub struct AssetData {
    pub(crate) UV: Option<Rect>,
    pub(crate) Origin: (u32, u32),
    pub(crate) texture_type: TextureType,
}

impl AssetData {
    pub fn empty() -> Self {
        Self {
            UV: None,
            Origin: (0, 0),
            texture_type: TextureType::idk,
        }
    }
}

pub enum TextureType {
    icon,
    in_game_sprite,
    idk,
}
impl TextureType {

    fn to_index(&self) -> usize {
        match self {
            TextureType::icon => {0}
            TextureType::in_game_sprite => {1}
            TextureType::idk => {2}
        }
    }
}

impl Clone for TextureType {
    fn clone(&self) -> Self {
        match self.to_index() {
            0 => {TextureType::icon},
            1 => {TextureType::in_game_sprite},
            2 => {TextureType::idk}
            _ => {TextureType::idk}
        }
    }
}

impl Copy for TextureType {
    
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
        m.insert(
            "finger",
            AssetData {
                UV: Option::from(Rect::new(32, 0, 16, 16)),
                Origin: (0, 0),
                texture_type: TextureType::icon,
            },
        );
        m.insert(
            "cursor_old",
            AssetData {
                UV: Option::from(Rect::new(16, 0, 16, 16)),
                Origin: (0,0),
                texture_type: TextureType::icon
            }
        );
        Mutex::new(m)
    })
}




pub fn draw_pp_texture(x: i32, y: i32, ass: &AssetData, mut canvas: &mut WindowCanvas, sf: i32, textures : &Vec<Texture>) {
    let uv = ass.UV.unwrap();
    let tex_rect = Rect::new(x - ass.Origin.0 as i32, y - ass.Origin.1 as i32, uv.w as u32, uv.h as u32);

    canvas
        .set_scale(sf as f32, sf as f32)
        .expect("TODO: panic message");

    let texture = textures.get(ass.texture_type.to_index());

    canvas
        .copy_ex(&texture.unwrap(),
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
