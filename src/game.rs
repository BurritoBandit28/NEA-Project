use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use image::codecs::jpeg::PixelDensityUnit;
use sdl2::render::{Texture, WindowCanvas};
use crate::entity::{Entity, EntityTest, Mobile, Static};
use crate::render::draw_pp_texture;

// The data type that holds all game data.
pub struct Game {
    pub statics_old: HashMap<String, *mut EntityTest>, // old old (didnt even slightly work)
    pub mobiles_old: Vec<EntityTest>, // old (worked but used types
    pub mobiles: Vec<Arc<Mutex<dyn Mobile>>>, // new (uses traits) (better)
    pub statics : Vec<Arc<dyn Static>>,
    pub player : usize
    
}

impl Game {

    // what happens every game loop
    pub fn cycle(&mut self, delta : f32) {

        // old (types)
        for e in self.mobiles_old.iter_mut() {

            e.physics(delta);
        }

        // new (traits)
        for e in self.mobiles.iter() {
            e.lock().unwrap().physics(delta)
        }

    }

    pub unsafe fn render(&mut self, canvas: &mut WindowCanvas, sf: i32, textures : &Vec<Texture>) {

        let mut m = &mut self.mobiles;

        let player = m.get(0).unwrap();
        let player_coords= player.lock().unwrap().get_coords();

        for e in m.iter() {
            let screen_coords = &e.lock().unwrap().screen(player_coords);
            let asset_data = e.lock().unwrap().get_asset_data();
            draw_pp_texture(screen_coords.0 as i32, screen_coords.1 as i32, &asset_data, canvas, sf, textures)
        }


        /*
        let mut player = self.mobiles_old.get(0).unwrap();

        for e in self.mobiles_old.iter() {
            let screen_coords = &e.screen(player);
            let asset_data = &e.asset_data;
            draw_pp_texture(screen_coords.0 as i32, screen_coords.1 as i32, asset_data, canvas, sf, textures)
        }

        for e in self.statics.iter() {
            let screen_coords = unsafe { &e.1.as_ref().unwrap().screen(self.mobiles.get("player").unwrap().as_mut().unwrap())};
            let asset_data =unsafe { &e.1.as_ref().unwrap().asset_data};
            draw_pp_texture(screen_coords.0 as i32, screen_coords.1 as i32, asset_data, canvas, sf, textures)
        }

         */

    }
    
    pub fn initiate() -> Self {
        
        Self{
            statics_old: HashMap::new(),
            mobiles_old: vec![],
            mobiles: vec![],
            statics: vec![],
            player : 0
        }
        
    }

    
}

