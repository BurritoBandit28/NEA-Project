use std::collections::HashMap;
use log::warn;
use sdl2::render::{Texture, WindowCanvas};
use crate::game::Game;
use crate::render;
use crate::render::AssetData;
use crate::resource_location::ResourceLocation;

pub trait Widget {
    #[must_use]
    fn on_click(&mut self);

    #[must_use]
    fn get_selected(&mut self) -> bool;

    #[must_use]
    fn set_selected(&mut self, tf : bool);

    #[must_use]
    fn get_screen_coordinates(&mut self) -> (i32, i32);

    #[must_use]
    fn set_screen_coordinates(&mut self, x : i32, y : i32);

    fn correct_coords(&mut self, dims : (u32, u32)) -> (i32, i32){
        let coords = self.get_screen_coordinates();
        match self.get_allignment() {
            Alignment::RIGHT => {
                (dims.0 as i32 + coords.0 , (dims.1 / 2) as i32 - coords.1)
            }
            Alignment::TOP => {
                ((dims.0/2) as i32 + coords.0 , coords.1)
            }
            Alignment::BOTTOM => {
                ((dims.0/2) as i32 + coords.0 , dims.1 as i32 - coords.1)
            }
            Alignment::CENTRE => {
                ((dims.0/2) as i32 + coords.0 , (dims.1 / 2) as i32 - coords.1)
            }
            Alignment::LEFT => {
                (coords.0 , (dims.1 / 2) as i32 - coords.1)
            }
            _ => {
                coords
            }
        }
    }

    #[must_use]
    fn get_asset_data(&mut self) -> AssetData;

    #[must_use]
    fn set_asset_data(&mut self, ass : AssetData);

    #[must_use]
    fn get_resource_location(&mut self) -> ResourceLocation;

    #[must_use]
    fn get_allignment(&mut self) -> Alignment;

    #[must_use]
    fn set_allignment(&mut self, alignment: Alignment);

    #[must_use]
    fn get_game(&mut self);


    fn render(&mut self, textures : &HashMap<String, Texture>, sf : i32, canvas : &mut WindowCanvas, dims : (u32, u32)) {
        let coords = self.correct_coords(dims);
        render::draw_pp_texture(coords.0, coords.1, &self.get_asset_data(), canvas, sf, textures)
    }
}



pub enum Alignment {
    LEFT,
    RIGHT,
    TOP,
    BOTTOM,
    CENTRE,
    NONE
}

impl Alignment {
    pub fn parse(val : String) -> Self {
        match val.to_lowercase().as_str() {
            "left" => {
                Self::LEFT
            }
            "right" => {
                Self::RIGHT
            }
            "top" => {
                Self::TOP
            }
            "bottom" => {
                Self::BOTTOM
            }
            "centre" => {
                Self::CENTRE
            }
            _ => {
                warn!("Alignment {} could not be found!", val);
                Self::NONE
            }
        }
    }
}

impl Clone for Alignment {
    fn clone(&self) -> Self {
        match self {
            Alignment::LEFT => {Self::LEFT}
            Alignment::RIGHT => {Self::RIGHT}
            Alignment::TOP => {Self::TOP}
            Alignment::BOTTOM => {Self::BOTTOM}
            Alignment::CENTRE => {Self::CENTRE}
            Alignment::NONE => {Self::NONE}
        }
    }
}