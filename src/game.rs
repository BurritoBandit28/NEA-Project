use std::collections::HashMap;
use std::option::Iter;
use std::sync::{Arc, Mutex};
use image::codecs::jpeg::PixelDensityUnit;
use sdl2::event::{Event, EventPollIterator};
use sdl2::EventPump;
use sdl2::keyboard::Scancode;
use sdl2::render::{Texture, WindowCanvas};
use crate::entities::player::Player;
use crate::entity::{Entity, Mobile, Renderable, Static};
use crate::render::draw_pp_texture;
use crate::utils::order_sort;

// The data type that holds all game data.
pub struct Game {
    pub mobiles: Vec<Box<Mutex<dyn Mobile>>>, // new (uses traits) (better)
    pub statics : Vec<Box<Mutex<dyn Static>>>,
    pub player : Option<usize>,
    pub events: Vec<Event>,
    pub held_keys : Vec<Scancode>,
    pub running : bool
    
}

impl Game {

    // what happens every game loop
    pub fn cycle(&mut self, delta : f32) {

        // new (traits)
        for e in self.mobiles.iter() {
            e.lock().unwrap().physics(delta)
        }

    }

    pub unsafe fn render(&mut self, canvas: &mut WindowCanvas, sf: i32, textures : &Vec<Texture>) {

        //todo get render order by sorting all entities by their y coordinate, rendering the highest one first

        let order  = order_sort(&mut self.statics, &mut self.mobiles);

        let mut m = &mut self.mobiles;

        let player = m.get(self.player.unwrap()).unwrap();
        let player_coords= player.lock().unwrap().get_coords();

        for x in order {
            match x.0 {
                0 => {
                    let mut list = &mut self.statics;
                    let mut obj = list.get(x.1).unwrap().lock().unwrap();
                    let screen_coords = &obj.screen(player_coords);
                    let asset_data = &obj.get_asset_data();
                    draw_pp_texture(screen_coords.0, screen_coords.1, &asset_data, canvas, sf, textures)
                }
                _ => {
                    let mut list = &mut self.mobiles;
                    let mut obj = list.get(x.1).unwrap().lock().unwrap();
                    let screen_coords = &obj.screen(player_coords);
                    let asset_data = &obj.get_asset_data();
                    draw_pp_texture(screen_coords.0, screen_coords.1, &asset_data, canvas, sf, textures)
                }
            }

        }

        /*
        for e in renderables.iter() {
            let screen_coords = &e.lock().unwrap().screen(player_coords);
            let asset_data = e.lock().unwrap().get_asset_data();
            draw_pp_texture(screen_coords.0 as i32, screen_coords.1 as i32, &asset_data, canvas, sf, textures)
        }

           for e in m.iter() {
            let screen_coords = &e.lock().unwrap().screen(player_coords);
            let asset_data = e.lock().unwrap().get_asset_data();
            draw_pp_texture(screen_coords.0 as i32, screen_coords.1 as i32, &asset_data, canvas, sf, textures)
        }
         */






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
            mobiles: vec![],
            statics: vec![],
            player : None,
            events: vec![],
            held_keys : vec![],
            running : true
        }
        
    }

    
}

