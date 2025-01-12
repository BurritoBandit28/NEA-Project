use std::cmp::PartialEq;
use sdl2::rect::Rect;
use std::sync::{Arc, Mutex};
use log::warn;
use sdl2::event::{Event, EventPollIterator};
use sdl2::EventPump;
use sdl2::keyboard::{Keycode, Scancode};
use uuid::Uuid;
use crate::entity::{Entity};
use crate::game::Game;
use crate::render::AssetData;
use crate::resource_location::ResourceLocation;
use crate::screen::Screen;
use crate::screens::you_died::DeathScreen;
use crate::tile::{TileSize, TileType};
use crate::utils::{create_uuid, mul_vec, normalise_vec};

/// this file contains the code for the Player entity

pub struct Player {
    coords: (f32, f32),
    pub asset_data: AssetData,
    // hitbox : matrix,
    velocity : (f32, f32),
    uuid : Uuid,
    game : *mut Game,
    health : f32,
    resource_location: ResourceLocation,
    index : usize,
}

impl Entity for Player {
    fn tick(&mut self, delta: f32) {

        // on death display the death screen and unload the level
        if self.health <= 0.0 {
            let game = unsafe { &mut *self.game };
            game.entities.clear();
            game.current_level = None;
            game.current_screen = Some(DeathScreen::create(game));
        }


    }

    // this function was made before the tick function was added, if I did this now I would
    // use a different approach
    fn physics(&mut self, delta: f32) {

        self.tick(delta);

        let game = self.game.clone();

        unsafe { self.handle_input((*game).held_keys.clone(), (*game).events.clone()) }

        let prev_coords = self.coords;

        let mut x = self.get_coords().0 + self.get_velocity().0 * delta;
        let mut y = self.get_coords().1 + self.get_velocity().1 * delta;

        x = if self.get_in_wall((x, prev_coords.1)) {prev_coords.0} else {x};
        y = if self.get_in_wall((prev_coords.0, y)) {prev_coords.1} else {y};

        self.set_coords((x,y))
    }

    fn get_coords(&mut self) -> (f32, f32) {
        self.coords
    }

    fn set_coords(&mut self, coords : (f32, f32)) {
        self.coords = coords;
    }

    fn get_health(&mut self) -> f32 {
        self.health
    }

    fn get_asset_data(&mut self) -> AssetData {
        self.asset_data.clone()
    }

    fn get_velocity(&mut self) -> (f32, f32) {
        self.velocity
    }

    fn set_velocity(&mut self, velocity: (f32, f32)) {
        self.velocity = velocity;
    }

    fn get_resource_location(&self) -> &ResourceLocation {
        &self.resource_location
    }

    fn get_index(&self) -> usize {
        self.index
    }

    fn change_health(&mut self, amount: f32) {
        self.health += amount;
    }
}


impl Player {
    pub fn create(game: &mut Game) {
        if game.player.is_none() {
            let asset_data = AssetData {
                uv: Option::from(Rect::new(0, 0, 32, 32)),
                origin: (16, 22),
                resource_location: ResourceLocation::new("game", "entity/player.png"),
            };

            let uuid = create_uuid();

            let mut player = Self {
                coords: (0.0, 0.0),
                asset_data,
                velocity: (0.0, 0.0),
                uuid,
                game,
                health : 20.0,
                resource_location : ResourceLocation::new("game", "entity/player"),
                index : game.entities.len()
            };

            let ret = Box::new(Mutex::new(player));

            game.player = Some(ret.lock().unwrap().index);
            game.entities.push(ret);
            //game.player = Some(ret.clone());
        }
        else {
            warn!("Player already exists in instance! @ index {}", game.player.unwrap())
        }
    }

    pub fn get_in_wall(&mut self, coords : (f32, f32)) -> bool {

        let game_level = unsafe { &mut *self.game }.current_level.as_mut();

        if game_level.is_none() {
            return true
        }

        let px = (if coords.0 < 0.0 {coords.0 - 1.0} else { coords.0 }) as i32;
        let py = (if coords.1 < 0.0 {coords.1 - 1.0} else { coords.1 }) as i32;

        if game_level.unwrap().tile_nav.get_tile(px, py).get_type() == TileType::WALL {
            return true
        }
        false
    }

    pub fn handle_input(&mut self, held_keys: Vec<Scancode>, events: Vec<Event>) {
        // replace with an actual drag constant in the physics loop
        //self.set_velocity((0.0, 0.0));
        let mut ret_vel = (0.0, 0.0);


        for key in held_keys {
            match key {
                Scancode::W => {
                    ret_vel.1 -= 1.0;
                }
                Scancode::S => {
                    ret_vel.1 += 1.0;
                }
                Scancode::D => {
                    ret_vel.0 += 1.0;
                }
                Scancode::A => {
                    ret_vel.0 -= 1.0;
                }
                _ => {}
            }
        }
        let mut norm = normalise_vec(ret_vel);
        mul_vec(&mut norm, 60.0);
        self.set_velocity(norm);
    }
}