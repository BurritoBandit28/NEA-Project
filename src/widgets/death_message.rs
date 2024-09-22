use std::collections::HashMap;
use num::clamp;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use crate::game::Game;
use crate::render;
use crate::render::AssetData;
use crate::resource_location::ResourceLocation;
use crate::widget::{Alignment, Widget};
use crate::widgets::play_widget::PlayWidget;

pub struct DeathMessage {
    selected : bool,
    asset_data: AssetData,
    alignment: Alignment,
    coords : (i32, i32),
    game : *mut Game,
}

impl DeathMessage {
    pub fn create(alignment: Alignment, x : i32, y : i32, game : *mut Game) -> Box<Self>
    where
        Self: Sized
    {

        let ret = Self {
            selected: false,
            asset_data: AssetData {
                uv: Some(Rect::new(0, 0, 320, 180)),
                origin: (0, 0),
                resource_location: ResourceLocation::new("game", "misc/you_died.png"),
            },
            alignment,
            coords: (x, y),
            game
        };
        Box::new(ret)
    }
}

impl Widget for DeathMessage {
    fn on_click(&mut self) {}

    fn get_selected(&mut self) -> bool {
        false
    }

    fn set_selected(&mut self, tf: bool) {}

    fn get_screen_coordinates(&mut self) -> (i32, i32) {
        self.coords
    }


    fn set_screen_coordinates(&mut self, x: i32, y: i32) {
        self.coords = (x, y)
    }

    fn get_asset_data(&mut self) -> AssetData {
        self.asset_data.clone()
    }

    fn set_asset_data(&mut self, ass: AssetData) {
        self.asset_data = ass
    }

    fn get_resource_location(&mut self) -> ResourceLocation {
        ResourceLocation::new("game", "widgets/dEATHDEATHDEATH")
    }

    fn get_allignment(&mut self) -> Alignment {
        self.alignment.clone()
    }

    fn set_allignment(&mut self, alignment: Alignment) {
        self.alignment = alignment
    }

    // again why is this here what was I on??
    fn get_game(&mut self) {
    }


}