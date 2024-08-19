use std::fmt::Pointer;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use sdl2::event::EventPollIterator;
use sdl2::rect::Rect;
use crate::entities::player::Player;
use crate::game::Game;
use crate::render::{AssetData, TextureType};

/// This file contains all the traits and helper functions to do with Entities

//todo: delete old entity stuff

// all basic function any entity type must have
pub trait Entity {

    fn is_static(&self) -> bool {
        false
    }

    #[must_use]
    fn get_coords(&mut self) -> (f32, f32) {
        (0f32, 0f32)
    }

    #[must_use]
    fn set_coords(&mut self, coords : (f32, f32)) {

    }

    #[must_use]
    fn get_health(&mut self) -> &f32 {
        &10.0
    }

    fn change_health(&mut self, amount : f32) {
        self.get_health() + amount;
    }


}

pub trait Renderable : Entity {

    // convert world space coordinates to screen coordinates.
    // TODO add support for negative coordinates less than -160 and -90
    fn screen(&mut self, player_coords :  (f32, f32)) -> (i32, i32) {
        let x = self.get_coords().0;
        let y = self.get_coords().1;
        let px = (if player_coords.0 < 0.0 {player_coords.0 - 1.0} else { player_coords.0 }) as i32;
        let py = (if player_coords.1 < 0.0 {player_coords.1 - 1.0} else { player_coords.1 }) as i32;
        (((160i32 - px) as f32 + x) as i32, ((90i32 - py ) as f32 + y) as i32)
    }

    #[must_use]
    fn get_asset_data(&self) -> AssetData {
        AssetData::empty()
    }
}


// methods for Static entities only
pub trait Static : Entity + Renderable {
    // we'll get to this later maybe at some point
}

// methods for Mobile entities only
pub trait Mobile : Entity + Renderable {
    #[must_use]
    fn get_velocity(&mut self) -> (f32,f32) {
       (0f32, 0f32)
    }
    #[must_use]
    fn set_velocity(&mut self, velocity : (f32, f32)) {}

    // physics loop for the entity - different entities could have separate physics loops
    // delta refers to the time between each frame. this allows the velocity to correctly run in units of pixels/second
    fn physics(&mut self, delta : f32) {

        let x = self.get_coords().0 + self.get_velocity().0 * delta;
        let y = self.get_coords().1 + self.get_velocity().1 * delta;

        self.set_coords( (x,y) )
    }

}

