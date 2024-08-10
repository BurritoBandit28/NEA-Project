use sdl2::rect::Rect;
use std::sync::{Arc, Mutex};
use crate::entity::{Entity, Mobile};
use crate::game::Game;
use crate::render::{AssetData, TextureType};


/// this file contains the code for the Player entity

pub struct Player {
    coords: (f32, f32),
    pub asset_data: AssetData,
    // hitbox : matrix,
    velocity : (f32, f32),
    uuid : String,
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
        let asset_data = AssetData {
            UV : Option::from(Rect::new(0, 0, 16, 16)),
            Origin : (8,8),
            texture_type : TextureType::in_game_sprite
        };

        let mut player = Self{
            coords: (0.0,0.0),
            asset_data,
            velocity: (0.0, 0.0),
            uuid: "player".to_string(),
            game: game,
        };

        let ret = Arc::new(Mutex::new(player));

        game.mobiles.push(ret)
    }
}

impl Mobile for Player {
    fn get_velocity(&mut self) -> (f32, f32) {
        self.velocity
    }

    fn set_velocity(&mut self, velocity: (f32, f32)) {
        self.velocity = velocity;
    }
}