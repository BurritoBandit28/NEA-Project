use sdl2::rect::Rect;
use std::sync::{Arc, Mutex};
use crate::entity::{Entity, Mobile, Renderable};
use crate::game::Game;
use crate::render::{AssetData, ResourceLocation};

pub struct Enemy {
    coords: (f32, f32),
    pub asset_data: AssetData,
    velocity : (f32, f32),
    uuid : String,
    game : *mut Game
}

impl Entity for Enemy {
    fn get_coords(&mut self) -> (f32, f32) {
        self.coords
    }

    fn set_coords(&mut self, coords : (f32, f32)) {
        self.coords = coords;
    }



}

impl Enemy {
    pub fn create(game: &mut Game) {
        let asset_data = AssetData {
            uv: Option::from(Rect::new(0, 0, 32, 32)),
            origin: (16, 22),
            identifier: ResourceLocation::new("game", "sprites\\entity\\enemy.png"),
        };

        let mut entity = Self{
            coords: (0.0,0.0),
            asset_data,
            velocity: (0.0, 0.0),
            uuid: "100".to_string(), // will be from hash function
            game,
        };
        let ret = Box::new(Mutex::new(entity));
        game.mobiles.push(ret);
    }
}

impl Renderable for Enemy {
    fn get_asset_data(&self) -> AssetData {
        AssetData {
            uv: self.asset_data.uv.clone(),
            origin: self.asset_data.origin.clone(),
            identifier: self.asset_data.identifier.clone()
        }
    }
}

impl Mobile for Enemy {
    fn get_velocity(&mut self) -> (f32, f32) {
        self.velocity
    }

    fn set_velocity(&mut self, velocity: (f32, f32)) {
        self.velocity = velocity;
    }

}