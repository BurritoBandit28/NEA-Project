use sdl2::rect::Rect;
use std::sync::{Arc, Mutex};
use log::warn;
use sdl2::event::{Event, EventPollIterator};
use sdl2::EventPump;
use sdl2::keyboard::{Keycode, Scancode};
use crate::entity::{Entity, Mobile, Renderable};
use crate::game::Game;
use crate::render::{AssetData, Identifier, TextureType};
use crate::utils::{mul_vec, normalise_vec};

/// this file contains the code for the Player entity

pub struct Player {
    coords: (f32, f32),
    pub asset_data: AssetData,
    // hitbox : matrix,
    velocity : (f32, f32),
    uuid : String,
    game : *mut Game,
    health : f32
}

impl Entity for Player {
    fn get_coords(&mut self) -> (f32, f32) {
        self.coords
    }

    fn set_coords(&mut self, coords : (f32, f32)) {
        self.coords = coords;
    }

    fn get_health(&mut self) -> &f32 {
        &self.health
    }

}

impl Renderable for Player {
    fn get_asset_data(&self) -> AssetData {
        AssetData {
            uv: self.asset_data.uv.clone(),
            origin: self.asset_data.origin.clone(),
            texture_type: self.asset_data.texture_type.clone(),
            identifier: self.asset_data.identifier.clone(),
        }
    }
}

impl Player {
    pub fn create(game: &mut Game) {
        if game.player.is_none() {
            let asset_data = AssetData {
                uv: Option::from(Rect::new(0, 0, 16, 16)),
                origin: (8, 8),
                texture_type: TextureType::in_game_sprite,
                identifier: Identifier::empty(),
            };

            let mut player = Self {
                coords: (0.0, 0.0),
                asset_data,
                velocity: (0.0, 0.0),
                uuid: "player".to_string(),
                game: game,
                health : 20.0
            };

            let ret = Box::new(Mutex::new(player));

            game.player = Some(game.mobiles.len());
            game.mobiles.push(ret);
            //game.player = Some(ret.clone());
        }
        else {
            warn!("Player already exists in instance! @ index {}", game.player.unwrap())
        }
    }

    pub fn handle_input(&mut self, held_keys: Vec<Scancode>, events: Vec<Event>) {
        // replace with an actual drag constant in the physics loop
        //self.set_velocity((0.0, 0.0));
        let mut ret_vel = (0.0, 0.0);

        for event in events {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => unsafe{(*self.game).running = false},

                _ => {}
            }
        }

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
        //normalise_vec(&mut ret_vel);
        mul_vec(&mut ret_vel, 20.0);
        self.set_velocity(ret_vel);
    }
}

impl Mobile for Player {
    fn get_velocity(&mut self) -> (f32, f32) {
        self.velocity
    }

    fn set_velocity(&mut self, velocity: (f32, f32)) {
        self.velocity = velocity;
    }
    fn physics(&mut self, delta: f32) {
        let game = self.game.clone();
        unsafe { self.handle_input((*game).held_keys.clone(), (*game).events.clone()) }
        let x = self.get_coords().0 + self.get_velocity().0 * delta;
        let y = self.get_coords().1 + self.get_velocity().1 * delta;
        self.set_coords( (x,y) );
    }
}