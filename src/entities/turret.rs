use std::sync::Mutex;
use num::integer::sqrt;
use sdl2::rect::Rect;
use uuid::Uuid;
use crate::entity::Entity;
use crate::game::Game;
use crate::render::AssetData;
use crate::resource_location::ResourceLocation;
use crate::utils::create_uuid;

pub struct Turret {
    coords: (f32, f32),
    asset_data: AssetData,
    velocity : (f32, f32),
    uuid : Uuid,
    game : *mut Game,
    health : f32,
    facing : Facing,
    resource_location: ResourceLocation
}


impl Turret {
    pub fn create(game: &mut Game) {
        let asset_data = AssetData {
            uv: Option::from(Rect::new(0, 0, 32, 32)),
            origin: (16, 22),
            resource_location: ResourceLocation::new("game", "entity/turret/turret_se.png"),
        };

        let uuid = create_uuid();

        let mut entity = Self{
            coords: (0.0,0.0),
            asset_data,
            velocity: (0.0, 0.0),
            uuid, // will be from hash function
            game,
            resource_location: ResourceLocation::new("game", "entity/turret"),
            health: 15.0,
            facing: Facing::SE,
        };
        let ret = Box::new(Mutex::new(entity));
        game.entities.push(ret);
    }
}

impl Entity for Turret {
    fn is_static(&self) -> bool {
        true
    }

    fn get_coords(&mut self) -> (f32, f32) {
        self.coords
    }

    fn set_coords(&mut self, coords: (f32, f32)) {
        self.coords = coords;
    }

    fn get_health(&mut self) -> &f32 {
        &self.health
    }

    fn tick(&mut self, delta: f32) {
        let game = unsafe { &mut *self.game };
        let player = game.get_player().get_mut().unwrap();
        let dist = f32::sqrt((player.get_coords().0 - self.coords.0)*(player.get_coords().0 - self.coords.0) + ((player.get_coords().1 - self.coords.1) * (player.get_coords().1 - self.coords.1)));
        //println!("{}", dist);
        if player.get_coords().1 >= self.coords.1 && dist < 150.0 {
            let angle= f32::atan2(player.get_coords().1 - self.coords.1, player.get_coords().0 - self.coords.0);
            // 0 -> (1/12 * PI)
            if (0.0..0.263).contains(&angle) {
                self.facing = Facing::E
            }

            // (1/12 * PI) -> (PI/2) - (1/12 * PI)
            if (0.263..1.32).contains(&angle) {
                self.facing = Facing::SE
            }

            // (PI/2) - (1/12 * PI) -> (PI/2) + (1/12 * PI)
            if (1.32..1.84).contains(&angle) {
                self.facing = Facing::S
            }

            // (PI/2) + (1/12 * PI) -> (11/12 * PI)
            if (1.84..2.88).contains(&angle) {
                self.facing = Facing::SW
            }

            // (11/12 * PI) -> PI
            if (2.88..3.14).contains(&angle) {
                self.facing = Facing::W
            }
        }
    }

    fn get_resource_location(&self) -> &ResourceLocation {
        &self.resource_location
    }

    fn get_velocity(&mut self) -> (f32, f32) {
        (0.0,0.0)
    }

    fn set_velocity(&mut self, velocity: (f32, f32)) {}

    fn get_asset_data(&mut self) -> AssetData {
        match self.facing {
            Facing::E => {
                self.asset_data.resource_location = ResourceLocation::new("game", "entity/turret/turret_e.png")
            }
            Facing::SE => {
                self.asset_data.resource_location = ResourceLocation::new("game", "entity/turret/turret_se.png")
            }
            Facing::S => {
                self.asset_data.resource_location = ResourceLocation::new("game", "entity/turret/turret_s.png")
            }
            Facing::SW => {
                self.asset_data.resource_location = ResourceLocation::new("game", "entity/turret/turret_sw.png")
            }
            Facing::W => {
                self.asset_data.resource_location = ResourceLocation::new("game", "entity/turret/turret_w.png")
            }
        }
        self.asset_data.clone()
    }
}

enum Facing {
    E,
    SE,
    S,
    SW,
    W
}