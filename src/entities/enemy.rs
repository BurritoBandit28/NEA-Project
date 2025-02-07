use sdl2::rect::Rect;
use std::sync::Mutex;
use uuid::Uuid;
use crate::entity::{Entity};
use crate::game::Game;
use crate::render::AssetData;
use crate::resource_location::ResourceLocation;
use crate::utils::create_uuid;

// This was used during development and doesn't do anything
// it is very similar to the other entity objects

pub struct Enemy {
    coords: (f32, f32),
    pub asset_data: AssetData,
    velocity : (f32, f32),
    uuid : Uuid,
    game : *mut Game,
    resource_location: ResourceLocation,
    health : f32,
    index : usize,
}

impl Entity for Enemy {
    fn get_coords(&mut self) -> (f32, f32) {
        self.coords
    }

    fn set_coords(&mut self, coords : (f32, f32)) {
        self.coords = coords;
    }

    fn get_asset_data(&mut self) -> AssetData {
        AssetData {
            uv: self.asset_data.uv.clone(),
            origin: self.asset_data.origin.clone(),
            resource_location: self.asset_data.resource_location.clone()
        }
    }

    fn get_velocity(&mut self) -> (f32, f32) {
        self.velocity
    }

    fn set_velocity(&mut self, velocity: (f32, f32)) {
        self.velocity = velocity;
    }

    fn get_health(&mut self) -> f32 {
        self.health
    }

    fn get_resource_location(&self) -> &ResourceLocation {
        &self.resource_location
    }

    fn get_index(&self) -> usize {
        self.index
    }

    fn change_health(&mut self, amount: f32) {
        todo!()
    }
}

impl Enemy {
    pub fn create(game: &mut Game) {
        let asset_data = AssetData {
            uv: Option::from(Rect::new(0, 0, 32, 32)),
            origin: (16, 22),
            resource_location: ResourceLocation::new("game", "entity/enemy.png"),
        };

        let uuid = create_uuid();

        let mut entity = Self{
            coords: (0.0,0.0),
            asset_data,
            velocity: (0.0, 0.0),
            uuid, // will be from hash function
            game,
            resource_location: ResourceLocation::new("game", "entity/enemy"),
            health: 15.0,
            index: game.entities.len(),
        };
        let ret = Box::new(Mutex::new(entity));
        game.entities.push(ret);
    }
}