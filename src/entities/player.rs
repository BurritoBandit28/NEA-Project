use sdl2::rect::Rect;
use std::sync::{Arc, Mutex};
use sdl2::event::{Event, EventPollIterator};
use sdl2::EventPump;
use sdl2::keyboard::{Keycode, Scancode};
use crate::entity::{Entity, Mobile};
use crate::game::Game;
use crate::render::{AssetData, TextureType};
use crate::utils::{mul_vec, normalise_vec};

/// this file contains the code for the Player entity

pub struct Player {
    coords: (f32, f32),
    pub asset_data: AssetData,
    // hitbox : matrix,
    velocity : (f32, f32),
    uuid : String,
    event_pump: Option<EventPump>,
    game : *mut Game
}

impl Entity for Player {
    fn get_coords(&mut self) -> (f32, f32) {
        self.coords
    }

    fn set_coords(&mut self, coords : (f32, f32)) {
        self.coords = coords;
    }

    fn get_asset_data(&self) -> AssetData {
        AssetData {
            UV: self.asset_data.UV.clone(),
            Origin: self.asset_data.Origin.clone(),
            texture_type: self.asset_data.texture_type.clone(),
        }
    }


}

impl Player {
    pub fn create(game: &mut Game) {
        if game.player.is_none() {
            let asset_data = AssetData {
                UV: Option::from(Rect::new(0, 0, 16, 16)),
                Origin: (8, 8),
                texture_type: TextureType::in_game_sprite
            };

            let mut player = Self {
                coords: (0.0, 0.0),
                asset_data,
                velocity: (0.0, 0.0),
                uuid: "player".to_string(),
                event_pump: None,
                game: game,
            };

            let ret = Box::new(Mutex::new(player));

            game.player = Some(game.mobiles.len());
            game.mobiles.push(ret);
            //game.player = Some(ret.clone());
        }
        else {
            println!("Player already exists in instance! @ index {}", game.player.unwrap())
        }
    }

    pub fn add_pump(&mut self, ep : EventPump) {
        self.event_pump = Some(ep);
    }

    pub fn movement(&mut self, keys: Vec<Scancode>) {
        // replace with an actual drag constant in the physics loop
        //self.set_velocity((0.0, 0.0));
        let mut ret_vel = (0.0, 0.0);

        for key in keys {
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
        unsafe { self.movement((*game).held_keys.clone()) }
        let x = self.get_coords().0 + self.get_velocity().0 * delta;
        let y = self.get_coords().1 + self.get_velocity().1 * delta;
        self.set_coords( (x,y) );
    }
}