use std::sync::Mutex;
use num::abs;
use sdl2::keyboard::Keycode::N;
use sdl2::rect::Rect;
use uuid::Uuid;
use crate::entities::explosion::Explosion;
use crate::entity::Entity;
use crate::game::Game;
use crate::render::AssetData;
use crate::resource_location::ResourceLocation;
use crate::utils::{create_uuid, get_dist, mul_vec, normalise_vec};

pub struct FloatyBomb {
    coords : (f32, f32),
    health : f32,
    velocity : (f32,f32),
    asset_data: AssetData,
    resource_location: ResourceLocation,
    index : usize,
    uuid : Uuid,
    game : *mut Game,
    target : Option<(f32, f32)>
}

impl Entity for FloatyBomb {
    fn get_coords(&mut self) -> (f32, f32) {
        self.coords.clone()
    }

    fn set_coords(&mut self, coords: (f32, f32)) {
        self.coords = coords
    }

    fn get_health(&mut self) -> f32 {
        self.health
    }

    fn change_health(&mut self, amount: f32) {
        self.health += amount
    }

    fn tick(&mut self, delta: f32) {
        // get the game instance
        let game = unsafe { &mut *self.game };
        // if there is no current target OR current target = current coords
        if self.target.is_none() || self.target == Some((
            (((self.coords.0 as i32 / 16) * 16) + 8) as f32,
            (((self.coords.1 as i32 / 16) * 16) + 8) as f32)
        ) {
            // get player coords
            let player = game.get_player().unwrap().lock().unwrap().get_coords();
            // get player in the tile coords (nav tiles x16)
            let player_tile = (
                ((player.0 as i32 / 16) * 16) + 8,
                ((player.1 as i32 / 16) * 16) + 8 );
            // generate the path
            let path = game.current_level.as_mut().unwrap()
                .tile_nav.path_to(
                    self.coords.0 as i32,
                    self.coords.1 as i32, player_tile.0, player_tile.1);
            // if the second position exists, set that to current target
            if path.get(1).is_some() {
                self.target = Some((
                    (path.get(1).unwrap().clone().0 + 8) as f32,
                    (path.get(1).unwrap().clone().1 + 8) as f32));
            }
            // if the second position does not exist, and distance is less
            // than 20 pixels
            if path.get(1).is_none() || get_dist(&self.coords, &player) < 20 {
                // spawn an explosion
                Explosion::create(game, self.coords);
                // remove 10 health points from the player
                game.get_player().unwrap().lock().unwrap().change_health(-10.0);
                // remove the floaty bomb from the entity list
                game.entities.remove(self.index);
            }

        }
        // create a normalised vector in the direction from the current
        // position to the target position
        let mut normalised = normalise_vec((self.target.unwrap().0 - self.coords.0, self.target.unwrap().1 - self.coords.1));
        // multiply by 20
        mul_vec(&mut normalised, 20.0);
        // set the current velocity
        self.set_velocity(normalised);
    }

    fn get_resource_location(&self) -> &ResourceLocation {
        &self.resource_location
    }

    fn get_index(&self) -> usize {
        self.index
    }

    fn get_velocity(&mut self) -> (f32, f32) {
        self.velocity
    }

    fn set_velocity(&mut self, velocity: (f32, f32)) {
        self.velocity = velocity
    }

    fn get_asset_data(&mut self) -> AssetData {
        self.asset_data.clone()
    }
}

impl FloatyBomb {

    pub fn create(game: &mut Game, coords : (f32, f32)) {

        let asset_data = AssetData {
            uv: Option::from(Rect::new(0, 0, 16, 16)),
            origin: (8, 8),
            resource_location: ResourceLocation::new("game", "entity/floaty_bomb/floaty_bomb.png"),
        };

        let uuid = create_uuid();

        let mut floaty_bomb = Self {
            coords,
            asset_data,
            velocity: (0.0, 0.0),
            uuid,
            game,
            health : 12.0,
            resource_location : ResourceLocation::new("game", "entity/floaty_bomb"),
            index : game.entities.len(),
            target: None,
        };

        let ret = Box::new(Mutex::new(floaty_bomb));

        game.entities.push(ret);

    }


}