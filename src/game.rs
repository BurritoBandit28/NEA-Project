use std::collections::HashMap;
use std::sync::Mutex;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::render::{Texture, WindowCanvas};
use crate::entity::{Entity};
use crate::level::Level;
use crate::render::draw_pp_texture;
use crate::screen::Screen;
use crate::utils::order_sort;

// The data type that holds all game data.
pub struct Game {
    pub entities: Vec<Box<Mutex<dyn Entity>>>, // new (uses traits) (better)
    pub player : Option<usize>,
    pub events: Vec<Event>,
    pub held_keys : Vec<Scancode>,
    pub running : bool,
    pub current_level : Option<Level>,
    pub current_screen : Option<Box<dyn Screen>>
    
}

impl Game {

    // what happens every game loop
    pub fn cycle(&mut self, delta : f32) {

        // new (traits)
        for e in self.entities.iter() {
            e.lock().unwrap().physics(delta)
        }

        for event in self.events.clone() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {self.running=false},

                _ => {}
            }
        }

    }

    pub unsafe fn render(&mut self, canvas: &mut WindowCanvas, sf: i32, textures : &HashMap<String, Texture>) {


        if !self.entities.is_empty() {
            let order = order_sort(&mut self.entities);

            let mut m = &mut self.entities;

            let player = m.get(self.player.unwrap()).unwrap();
            let player_coords = player.lock().unwrap().get_coords();

            let mut level = &mut self.current_level;
            if level.is_some() {
                level.as_mut().unwrap().render(player_coords, textures, canvas, sf);
            }

            for x in order {
                let mut list = &mut self.entities;
                let mut obj = list.get(x.1).unwrap().lock().unwrap();
                let screen_coords = &obj.screen(player_coords);
                let asset_data = &obj.get_asset_data();
                draw_pp_texture(screen_coords.0, screen_coords.1, &asset_data, canvas, sf, textures);
            }
        }

    }
    
    pub fn initiate() -> Self {
        
        Self{
            entities: vec![],
            player : None,
            events: vec![],
            held_keys : vec![],
            running : true,
            current_level : None,
            current_screen : None
        }
        
    }

    
}

