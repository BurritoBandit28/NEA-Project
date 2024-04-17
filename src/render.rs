use crate::{main, AssetData, TextureType};
use phf::{phf_map, Map};
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator, WindowCanvas};
use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::{Lazy, OnceCell};

pub struct IconsContainer {
    pub icons: HashMap<&'static str, AssetData>,
}

pub fn get_icons() -> &'static Mutex<HashMap<&'static str, AssetData>> {
    static INSTANCE: OnceCell<Mutex<HashMap<&'static str, AssetData>>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert("cursor", AssetData{
            UV: Option::from(Rect::new(0, 0, 16, 16)),
            Origin: (0, 0),
            texture_type: TextureType::icon,
        });
        Mutex::new(m)
    })
}

pub fn draw_pp_texture(x: i32, y: i32, texture: &Texture, uv: Rect, canvas: WindowCanvas) {}

pub fn load_assets(texture_creator: TextureCreator<WindowCanvas>) {
    //texture_creator.load_texture();
}

pub fn load_icons() {

}
