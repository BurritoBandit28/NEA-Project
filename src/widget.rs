use std::collections::HashMap;
use log::warn;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use crate::game::Game;
use crate::render;
use crate::render::AssetData;
use crate::resource_location::ResourceLocation;

/// Used to create buttons and widget UI displays.
pub trait Widget {

    /// The action to run when the widget is clicked
    #[must_use]
    fn on_click(&mut self);

    /// Get if the widget is being hovered over by the mouse
    #[must_use]
    fn get_selected(&mut self) -> bool;

    /// Set its selected state
    #[must_use]
    fn set_selected(&mut self, tf : bool);

    /// Get the widget coordinates
    #[must_use]
    fn get_screen_coordinates(&mut self) -> (i32, i32);

    /// Set the widget coordinates
    #[must_use]
    fn set_screen_coordinates(&mut self, x : i32, y : i32);

    /// Correct the widget coordinates to screen space coordinates given alignment - it's no CSS, but I tried...
    /// In future widgets may have their layout handled by [Clay](https://github.com/nicbarker/clay)
    fn correct_coords(&mut self) -> (i32, i32){
        let coords = self.get_screen_coordinates();
        let dims = render::get_actual_dimensions().lock().unwrap().get();
        match self.get_allignment() {
            Alignment::RIGHT => {
                // centre along the vertical middle of the screen, locked to the right of the screen
                (dims.0 as i32 + coords.0 , (dims.1 / 2) as i32 - coords.1)
            }
            Alignment::TOP => {
                // centre along the horizontal middle of the screen, locked to the top of the screen
                ((dims.0/2) as i32 + coords.0 , coords.1)
            }
            Alignment::BOTTOM => {
                // centre along the horizontal middle of the screen, locked to the right of the screen
                ((dims.0/2) as i32 + coords.0 , dims.1 as i32 - coords.1)
            }
            Alignment::CENTRE => {
                // centre along the vertical middle of the screen, and the horizontal middle
                ((dims.0/2) as i32 + coords.0 , (dims.1 / 2) as i32 - coords.1)
            }
            Alignment::LEFT => {
                // centre along the vertical middle of the screen, locked to the left of the screen
                (coords.0 , (dims.1 / 2) as i32 - coords.1)
            }
            _ => {
                coords
            }
        }
    }

    /// Returns the asset data
    #[must_use]
    fn get_asset_data(&mut self) -> AssetData;

    /// Returns the asset data for when debug render is enabled
    fn get_debug_asset_data(&mut self) -> AssetData {
        let mut ass = self.get_asset_data().clone();
        ass.uv = Some(Rect::new(0,0,ass.uv.unwrap().width(),ass.uv.unwrap().height()));
        ass.resource_location = ResourceLocation::new("game", "gui/widgets/debug_background.png");
        ass
    }

    /// Sets the widget asset data
    #[must_use]
    fn set_asset_data(&mut self, ass : AssetData);

    /// Gets the widget resource location - Not fully implemented todo
    #[must_use]
    fn get_resource_location(&mut self) -> ResourceLocation;

    /// Get widget alignment
    #[must_use]
    fn get_allignment(&mut self) -> Alignment;

    /// Set widget alignment
    #[must_use]
    fn set_allignment(&mut self, alignment: Alignment);

    /// Useless
    #[must_use]
    fn get_game(&mut self);

    /// Returns data in an integer form. If the data can be presented as an integer, this method can be used to extract said data.
    /// This is used by WidgetEnum, but could be used to return a string as an integer to then be serialised
    fn return_integer_data(&mut self) -> Option<usize> {
        None
    }

    /// Renders the widget to the screen, with the debug texture behind it should ``debug`` be true
    fn render(&mut self, textures : &HashMap<String, Texture>, sf : i32, canvas : &mut WindowCanvas, debug : bool) {
        let coords = self.correct_coords();
        if debug {
            render::draw_pp_texture(coords.0, coords.1, &self.get_debug_asset_data(), canvas, sf, textures)
        }
        render::draw_pp_texture(coords.0, coords.1, &self.get_asset_data(), canvas, sf, textures)
    }
}


/// Alignment enum for widget positioning
pub enum Alignment {
    LEFT,
    RIGHT,
    TOP,
    BOTTOM,
    CENTRE,
    NONE
}

// Implementing functions for Alignment
impl Alignment {

    /// Get alignment from string - initially made for data driven screens which was never fully realised.
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