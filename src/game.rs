use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::ops::DerefMut;
use std::sync::Mutex;
use chrono::Month;
use kira::manager::{AudioManager, AudioManagerSettings, DefaultBackend};
use kira::sound::static_sound::StaticSoundData;
use log::info;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::mouse::MouseButton;
use sdl2::render::{Texture, WindowCanvas};
use crate::entities::{enemy, player, turret};
use crate::entity::{Entity};
use crate::level::{Level, TileGraph};
use crate::{entities, render, sound};
use crate::entities::floaty_bomb::FloatyBomb;
use crate::render::draw_pp_texture;
use crate::resource_location::ResourceLocation;
use crate::screen::Screen;
use crate::sound::Sound;
use crate::tile::{Tile, TileSize};
use crate::utils::order_sort;
use crate::widget::Widget;

/// An object that manages a game instance. It holds all game data and manages the render, physics and screen loops
pub struct Game {
    pub entities: Vec<Box<Mutex<dyn Entity>>>, // new (uses traits) (better)
    pub player : Option<usize>,
    pub events: Vec<Event>,
    pub held_keys : Vec<Scancode>,
    pub running : bool,
    pub current_level : Option<Level>,
    pub current_screen : Option<Box<dyn Screen>>,
    pub tiles :  HashMap<String, Tile>,
    pub sounds : HashMap<String, Sound>,
    pub draw_mouse : bool,
    pub sf : i32,
    pub use_finger : bool,
    pub dims : (u32,u32),
    pub score : f32,
    debug : bool
}

impl Game {

    /// Physics and inputs
    pub fn cycle(&mut self, delta : f32, mousex : u32, mousey : u32, dims : (u32, u32)) {


        if !self.entities.is_empty() {
            self.score += delta;
        }

        // Run physics for every entity
        for entity in self.entities.iter() {
            entity.lock().unwrap().physics(delta)
        }

        // if there is a current screen, run its cycle function
        let _ = if self.current_screen.is_some() {
            self.current_screen.as_mut().unwrap().cycle(mousex, mousey, dims, self.events.clone())
        };

        // handle user inputs
        for event in self.events.clone() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    info!("Quitting game!");
                    // close game on Escape, or app closure
                    self.running=false
                },
                Event::KeyDown {
                    keycode: Some(Keycode::L),
                    ..
                } => {
                    self.debug= !self.debug
                },
                Event::MouseButtonDown {
                    mouse_btn : MouseButton::Left,
                    ..
                } => {
                    // on left click, check if the mouse is over a widget, if so, execute its on_click function
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

                _ => {
                    // do nothing
                }
            }
        }

    }

    /// Returns the entity assigned as the "player", may not always be [`Player`]
    ///
    /// [`Player`]: player::Player
    pub fn get_player(&mut self) -> Option<&mut Box<Mutex<dyn Entity>>> {
        self.entities.get_mut(self.player.unwrap())
    }

    /// Generates the demo level and loads it into the game instance. the actual level generations is found in [`Level::create_demo_level`]
    pub fn load_demo_level(&mut self) {

        // test entities
        player::Player::create(self);
        turret::Turret::create(self);
        FloatyBomb::create(self, (86.0,40.0));
        //FloatyBomb::create(self, (120.0,40.0));
        //enemy::Enemy::create(self);

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
            .set_coords((128.0, 36.0));
        self.current_level = Some(Level::create_demo_level(&self.tiles));
    }

    /// The render loop for entities, screens and the mouse. The entity rendering is done here, for specifics on other elements see the render functions for [`Screens`]|[`Levels/Tiles`]|[`Widgets`]
    ///
    /// [`Screens`]: Screen::render
    /// [`Levels/Tiles`]: Level::render
    /// [`Widgets`]: Widget::render
    pub fn render(&mut self, canvas: &mut WindowCanvas, sf: i32, textures : &HashMap<String, Texture>, dims : (u32, u32), mousex : u32, mousey : u32) {

        // if there are entities, render them to screen
        if !self.entities.is_empty() {

            // calculate the order the entities are rendered in - it is essential that the order of entities in the list isn't changed
            let order = order_sort(&mut self.entities);

            // get the player and its coordinates
            let player = self.get_player().unwrap();
            let player_coords = player.lock().unwrap().get_coords();

            // get the level
            let mut level = &mut self.current_level;
            // make sure the level isn't None, and render it to screen
            if level.is_some() {
                level.as_mut().unwrap().render(player_coords, textures, canvas, sf, self.debug);
            }

            // iterate through the order
            for x in order {
                let mut list = &mut self.entities;
                let mut obj = list.get(x.1).unwrap().lock().unwrap();
                let screen_coords = &obj.screen(player_coords);
                let asset_data = &obj.get_asset_data();
                draw_pp_texture(screen_coords.0, screen_coords.1, &asset_data, canvas, sf, textures);
            }
        }

        // get the current screen
        let scrn = &mut self.current_screen;
        // make sure the screen isn't None, and render it to screen
        if scrn.is_some() {
            scrn.as_mut().unwrap().render(textures, sf, canvas, dims);
        }

        // draw the mouse, unless instructed otherwise
        if self.draw_mouse {
            if self.use_finger {
                draw_pp_texture(
                    mousex as i32,
                    mousey as i32,
                    &render::get_icons().lock().unwrap().get("finger").unwrap(),
                    canvas,
                    sf,
                    &textures
                );
            }
            else {
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

    }

    /// Plays a sound file given a [`ResourceLocation`]
    pub fn play_sound(&mut self, resource_location : ResourceLocation) {
        // get sound file
        let sound_data = StaticSoundData::from_file(&self.sounds.get(&resource_location.to_string()).unwrap().path).unwrap();
        sound::get_audio_manager().lock().unwrap().play(sound_data.clone()).unwrap();
    }

    /// Create a [`Game`] instance
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
            sounds : Default::default(),
            draw_mouse : true,
            sf : 6,
            use_finger : false,
            dims: (0,0),
            score: 0.0,
            debug : false
        }
        
    }

    
}

