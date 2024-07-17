use std::fmt::Pointer;
use sdl2::rect::Rect;
use crate::game::Game;
use crate::render::{AssetData, TextureType};

pub struct Entity {

    coords: (f32, f32),
    pub asset_data: AssetData,
    // hitbox : matrix,
    velocity : (f32, f32),
    uuid : String,
    game : *mut Game


}

impl Entity {

    // convert world space coordinates to screen coordinates.
    // TODO add support for negative coordinates less than -160 and -90
    pub fn screen(&self, player : &Entity) -> (u32, u32) {
        let x = self.coords.0;
        let y = self.coords.1;
        let px = player.coords.0 as i32;
        let py = player.coords.1 as i32;
        return (((160i32 - px) as f32 + x) as u32, ((90i32 - py ) as f32 + y) as u32)

    }

    // sets the velocity of an object.
    pub fn set_velocity(&mut self, x : f32, y : f32) {
        self.velocity.0 = x;
        self.velocity.1 = y;
    }

    // physics loop for the entity - different entities could have separate physics loops
    // delta refers to the time between each frame. this allows the velocity to correctly run in units of pixels/second
    pub fn physics(&mut self, delta : f32) {
        self.coords.0 += self.velocity.0 * delta;
        self.coords.1 += self.velocity.1 * delta;

    }

    //temp - create an object to test elements of code.
    pub fn create_obj(game: &mut Game, coords : (f32, f32)) {

        let asset_data = AssetData {
            UV : Option::from(Rect::new(16, 0, 16, 16)),
            Origin : (8,8),
            texture_type : TextureType::in_game_sprite
        };

        let mut entity = Self{
            coords,
            asset_data,
            velocity: (0.0, 0.0),
            uuid: "100".to_string(), // will be from hash function
            game,
        };

        game.mobiles.push(entity)
    }

    // the player entity, like all game entities, will be its own struct in the future, at the moment this exists so that the screen() function can run correctly
    pub fn create_player(game: &mut Game) {

        let asset_data = AssetData {
            UV : Option::from(Rect::new(0, 0, 16, 16)),
            Origin : (8,8),
            texture_type : TextureType::in_game_sprite
        };

        let mut player = Self{
            coords: (0f32,0f32),
            asset_data,
            velocity: (0.0, 0.0),
            uuid: "player".to_string(),
            game: game,
        };

        game.mobiles.push(player)
    }

}