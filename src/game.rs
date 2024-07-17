use std::collections::HashMap;
use sdl2::render::{Texture, WindowCanvas};
use crate::entities::Entity;
use crate::render::draw_pp_texture;

// The data type that holds all game data.
pub struct Game {
    pub statics : HashMap<String, *mut Entity>,
    pub mobiles : Vec<Entity>,
    pub player : usize
    
}

impl Game {


    // what happens every game loop
    pub fn cycle(&mut self, delta : f32) {
        
        for e in self.mobiles.iter_mut() {

            e.physics(delta);
        }

    }

    pub unsafe fn render(&mut self, canvas: &mut WindowCanvas, sf: i32, textures : &Vec<Texture>) {

        let mut player = self.mobiles.get(0).unwrap();

        for e in self.mobiles.iter() {
            let screen_coords = &e.screen(player);
            let asset_data = &e.asset_data;
            draw_pp_texture(screen_coords.0 as i32, screen_coords.1 as i32, asset_data, canvas, sf, textures)
        }


        /*
        for e in self.statics.iter() {
            let screen_coords = unsafe { &e.1.as_ref().unwrap().screen(self.mobiles.get("player").unwrap().as_mut().unwrap())};
            let asset_data =unsafe { &e.1.as_ref().unwrap().asset_data};
            draw_pp_texture(screen_coords.0 as i32, screen_coords.1 as i32, asset_data, canvas, sf, textures)
        }

         */

    }
    
    pub fn initiate() -> Self {
        
        Self{
            statics: HashMap::new(),
            mobiles: vec![],
            player : 0
        }
        
    }

    
}

