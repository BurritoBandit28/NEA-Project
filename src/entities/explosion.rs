use std::sync::Mutex;
use image::math::Rect;
use num::clamp;
use uuid::Uuid;
use crate::entity::Entity;
use crate::game::Game;
use crate::render::AssetData;
use crate::resource_location::ResourceLocation;
use crate::utils::create_uuid;

/// When created it will play an explosion animation
pub struct Explosion {
    coords : (f32, f32),
    timer : f32,
    frames: Vec<AssetData>,
    resource_location: ResourceLocation,
    index : usize,
    game : *mut Game,
    uuid : Uuid

}

impl Entity for Explosion {
    fn is_static(&self) -> bool {
        true
    }

    fn get_coords(&mut self) -> (f32, f32) {
        self.coords
    }

    fn set_coords(&mut self, coords: (f32, f32)) {
        self.coords = coords
    }

    fn get_health(&mut self) -> f32 {
        1.0
    }

    fn change_health(&mut self, amount: f32) {
        // no
    }

    fn tick(&mut self, delta: f32) {
        if self.timer > 0.75 {
            let game = unsafe { &mut *self.game };
            game.entities.remove(self.index - 1);
        }
        self.timer += delta;
    }

    fn get_resource_location(&self) -> &ResourceLocation {
        &self.resource_location
    }

    fn get_index(&self) -> usize {
        self.index
    }

    fn get_velocity(&mut self) -> (f32, f32) {
        (0.0, 0.0)
    }

    fn set_velocity(&mut self, velocity: (f32, f32)) {
        // no
    }

    fn get_asset_data(&mut self) -> AssetData {
        let frame = clamp((self.timer / 0.125) as usize, 0, 5);
        self.frames[frame].clone()
    }
}

impl Explosion {
    pub fn create(game : &mut Game, coords : (f32, f32)) {
        let frame1 = AssetData {
            uv: Option::from(sdl2::rect::Rect::new(0, 0, 64, 64)),
            origin: (32, 32),
            resource_location: ResourceLocation::new("game", "entity/explosion/explosion.png"),
        };
        let frame2 = AssetData {
            uv: Option::from(sdl2::rect::Rect::new(0, 64, 64, 64)),
            origin: (32, 32),
            resource_location: ResourceLocation::new("game", "entity/explosion/explosion.png"),
        };
        let frame3 = AssetData {
            uv: Option::from(sdl2::rect::Rect::new(0, 128, 64, 64)),
            origin: (32, 32),
            resource_location: ResourceLocation::new("game", "entity/explosion/explosion.png"),
        };
        let frame4 = AssetData {
            uv: Option::from(sdl2::rect::Rect::new(0, 192, 64, 64)),
            origin: (32, 32),
            resource_location: ResourceLocation::new("game", "entity/explosion/explosion.png"),
        };
        let frame5 = AssetData {
            uv: Option::from(sdl2::rect::Rect::new(0, 256, 64, 64)),
            origin: (32, 32),
            resource_location: ResourceLocation::new("game", "entity/explosion/explosion.png"),
        };
        let frame6 = AssetData {
            uv: Option::from(sdl2::rect::Rect::new(0, 320, 64, 64)),
            origin: (32, 32),
            resource_location: ResourceLocation::new("game", "entity/explosion/explosion.png"),
        };

        let mut frames = vec![];
        frames.push(frame1);
        frames.push(frame2);
        frames.push(frame3);
        frames.push(frame4);
        frames.push(frame5);
        frames.push(frame6);

        let uuid = create_uuid();

        let mut explosion = Self {
            coords,
            timer: 0.0,
            uuid,
            game,
            resource_location : ResourceLocation::new("game", "entity/explosion"),
            index : game.entities.len(),
            frames,
        };

        let ret = Box::new(Mutex::new(explosion));

        game.entities.push(ret);
        game.play_sound(ResourceLocation::new("game", "sounds/entity/explosion/explosion.ogg"))
    }
}