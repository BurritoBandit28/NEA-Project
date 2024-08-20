use once_cell::sync::OnceCell;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::iter::Copied;
use std::sync::Mutex;
use log::warn;
use sdl2::keyboard::Scancode::I;
use sdl2::libc::stat;
use sdl2::pixels::Color;
use crate::render;

pub const DIMENSIONS: (u32, u32) = (320, 180);

// todo, make clone implementation
pub struct AssetData {
    pub(crate) uv: Option<Rect>,
    pub(crate) origin: (u32, u32),
    pub (crate) identifier: ResourceLocation,
}

impl AssetData {
    pub fn empty() -> Self {
        Self {
            uv: None,
            origin: (0, 0),
            identifier : ResourceLocation::empty()
        }
    }
}

#[derive(Debug)]
pub struct ResourceLocation {
    pub namespace : String,
    pub path : String,
}

impl PartialEq<Self> for ResourceLocation {
    fn eq(&self, other: &Self) -> bool {
        self.path.eq(&other.path) && self.path.eq(&other.namespace)
    }
}

impl Eq for ResourceLocation {

}

impl ResourceLocation {
    pub fn new(namespace : &str, path : &str) -> Self {
        Self {
            namespace : namespace.to_string(),
            path : path.to_string(),
        }
    }
    pub fn empty() -> Self {
        Self {
            namespace : "game".to_string(),
            path : "missing.png".to_string(),
        }
    }

    pub fn set_namespace(&mut self, namespace : String) {
        self.namespace = namespace;
    }

    pub fn set_path(&mut self, path : String) {
        self.path = path;
    }

    pub fn to_string(&self) -> String {
        format!("{}:{}", self.namespace, self.path)
    }
}

impl Clone for ResourceLocation {
    fn clone(&self) -> Self {
        Self {
            namespace: self.namespace.to_string(),
            path: self.path.to_string(),
        }
    }
}


pub fn get_missing_list() -> &'static Mutex<Vec<String>>{
    static INSTANCE : OnceCell<Mutex<Vec<String>>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let vec : Vec<String> = vec![];
        Mutex::new(vec)
    })
}


pub fn get_icons() -> &'static Mutex<HashMap<&'static str, AssetData>> {
    static INSTANCE: OnceCell<Mutex<HashMap<&'static str, AssetData>>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert(
            "cursor",
            AssetData {
                uv: Option::from(Rect::new(0, 0, 16, 16)),
                origin: (0, 0),
                identifier: ResourceLocation::new("game", "sprites\\gui\\icons.png"),
            },
        );
        m.insert(
            "finger",
            AssetData {
                uv: Option::from(Rect::new(32, 0, 16, 16)),
                origin: (0, 0),
                identifier: ResourceLocation::new("game", "sprites\\gui\\icons.png"),
            },
        );
        m.insert(
            "cursor_old",
            AssetData {
                uv: Option::from(Rect::new(16, 0, 16, 16)),
                origin: (0, 0),
                identifier: ResourceLocation::new("game", "sprites\\gui\\icons.png"),
            }
        );
        Mutex::new(m)
    })
}




pub fn draw_pp_texture(x: i32, y: i32, ass: &AssetData, mut canvas: &mut WindowCanvas, sf: i32, textures : &HashMap<String, Texture>) {
    let uv = ass.uv.unwrap();
    let tex_rect = Rect::new(x - ass.origin.0 as i32, y - ass.origin.1 as i32, uv.w as u32, uv.h as u32);
    let mut id = ass.identifier.clone();

    canvas
        .set_scale(sf as f32, sf as f32)
        .expect("TODO: panic message");

    let mut texture = textures.get(&id.to_string());

    if texture.is_none(){
        if !get_missing_list().lock().unwrap().contains(&&id.to_string()) {
            warn!("Texture at {} could not be found!", id.to_string())
        }
        get_missing_list().lock().unwrap().push(id.clone().to_string());
        texture = textures.get(&ResourceLocation::new("game", "sprites\\missing.png").to_string());
    }

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
