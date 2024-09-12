use std::collections::HashMap;
use std::sync::Mutex;
use chrono::Month;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::mouse::MouseButton;
use sdl2::render::{Texture, WindowCanvas};
use crate::entities::{enemy, player};
use crate::entity::{Entity};
use crate::level::Level;
use crate::render;
use crate::render::draw_pp_texture;
use crate::screen::Screen;
use crate::tile::{Tile, TileSize};
use crate::utils::order_sort;
use crate::widget::Widget;

// The data type that holds all game data.
pub struct Game {
    pub entities: Vec<Box<Mutex<dyn Entity>>>, // new (uses traits) (better)
    pub player : Option<usize>,
    pub events: Vec<Event>,
    pub held_keys : Vec<Scancode>,
    pub running : bool,
    pub current_level : Option<Level>,
    pub current_screen : Option<Box<dyn Screen>>,
    pub tiles :  HashMap<String, Tile>,
    pub draw_mouse : bool,
    pub sf : i32
}

impl Game {

    // what happens every game loop
    pub fn cycle(&mut self, delta : f32, mousex : u32, mousey : u32, dims : (u32, u32)) {

        // new (traits)
        for e in self.entities.iter() {
            e.lock().unwrap().physics(delta)
        }

        let _ = if self.current_screen.is_some() {
            self.current_screen.as_mut().unwrap().cycle(mousex, mousey, dims, self.events.clone())
        };

        for event in self.events.clone() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {self.running=false},

                Event::MouseButtonDown {
                    mouse_btn : MouseButton::Left,
                    ..
                } => {

                    if self.current_screen.is_some() {
                        for wl in self.current_screen.as_mut().unwrap().get_widgets() {
                            for w in wl {
                                if w.get_selected() {
                                    w.on_click()
                                }
                            }
                        }
                    }
                }

                _ => {}
            }
        }

    }

    pub fn load_test_level(&mut self) {

    // test entities
    player::Player::create(self);
    enemy::Enemy::create(self);

    let _ = self
        .entities
        .get_mut(0)
        .unwrap()
        .lock()
        .unwrap()
        .set_coords((10.0, 10.0));

    let _ = self
        .entities
        .get_mut(1)
        .unwrap()
        .lock()
        .unwrap()
        .set_coords((20.0, 20.0));
        self.current_level = Some(Level::create_test_level(&self.tiles));
    }

    pub fn load_demo_level(&mut self) {

        // test entities
        player::Player::create(self);
        enemy::Enemy::create(self);

        let _ = self
            .entities
            .get_mut(0)
            .unwrap()
            .lock()
            .unwrap()
            .set_coords((16.0, 80.0));

        let _ = self
            .entities
            .get_mut(1)
            .unwrap()
            .lock()
            .unwrap()
            .set_coords((20.0, 20.0));
        self.current_level = Some(Level::create_demo_level(&self.tiles));
    }

    pub unsafe fn render(&mut self, canvas: &mut WindowCanvas, sf: i32, textures : &HashMap<String, Texture>, dims : (u32, u32), mousex : u32, mousey : u32) {


        if !self.entities.is_empty() {
            let order = order_sort(&mut self.entities);

            let mut m = &mut self.entities;

            let player = m.get(self.player.unwrap()).unwrap();
            let player_coords = player.lock().unwrap().get_coords();

            let mut level = &mut self.current_level;
            if level.is_some() {
                level.as_mut().unwrap().render(player_coords, textures, canvas, sf, false);
            }

            for x in order {
                let mut list = &mut self.entities;
                let mut obj = list.get(x.1).unwrap().lock().unwrap();
                let screen_coords = &obj.screen(player_coords);
                let asset_data = &obj.get_asset_data();
                draw_pp_texture(screen_coords.0, screen_coords.1, &asset_data, canvas, sf, textures);
            }
        }
        let scrn = &mut self.current_screen;

        if scrn.is_some() {
            scrn.as_mut().unwrap().render(textures, sf, canvas, dims);
        }
        if self.draw_mouse {
            draw_pp_texture(
                mousex as i32,
                mousey as i32,
                &render::get_icons().lock().unwrap().get("cursor").unwrap(),
                canvas,
                sf,
                &textures
            );
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
            current_screen : None,
            tiles: Default::default(),
            draw_mouse : true,
            sf : 6
        }
        
    }

    
}

