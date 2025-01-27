use std::collections::HashMap;
use sdl2::keyboard::Keycode::N;
use sdl2::rect::Rect;
use crate::game::{DyslexiaMode, Game};
use crate::render::AssetData;
use crate::resource_location::ResourceLocation;
use crate::screen::Screen;
use crate::screens::hud_screen::HudScreen;
use crate::screens::room_editor_screen::RoomEditorScreen;
use crate::widget::Alignment;
use crate::widget::Widget;


// When designing a widget, it is a good idea to keep in mind things like colour and contrast.
// Some users may have issues looking at certain colours, so keeping a high contrast is important.
// If you do not want to have high contrast by default, you can add it as a config option
pub struct PlayWidget {
    selected : bool,
    asset_data: AssetData,
    asset_data_selected : AssetData,
    asset_data_dyslexia: AssetData,
    asset_data_selected_dyslexia : AssetData,
    alignment: Alignment,
    coords : (i32, i32),
    game : *mut Game
}

impl PlayWidget {

    /*
    pub fn create(asset_data: AssetData, alignment: Alignment, x : i32, y : i32) -> Self {
        Self {
            selected: false,
            asset_data,
            alignment,
            coords: (x, y),
        }
    }
     */

    pub fn create(alignment: Alignment, x : i32, y : i32, game : *mut Game) -> Box<Self>
    where
        Self: Sized
    {

        let ret = Self {
            selected: false,
            asset_data: AssetData {
                uv: Some(Rect::new(0, 0, 112, 39)),
                origin: (0, 0),
                resource_location: ResourceLocation::new("game", "gui/widgets/play.png"),
            },
            asset_data_selected: AssetData {
                uv: Some(Rect::new(0, 39, 112, 39)),
                origin: (0, 0),
                resource_location: ResourceLocation::new("game", "gui/widgets/play.png"),
            },
            asset_data_dyslexia: AssetData {
                uv: Some(Rect::new(0, 0, 112, 39)),
                origin: (0, 0),
                resource_location: ResourceLocation::new("game", "gui/widgets/play_easy.png"),
            },
            asset_data_selected_dyslexia: AssetData {
                uv: Some(Rect::new(0, 39, 112, 39)),
                origin: (0, 0),
                resource_location: ResourceLocation::new("game", "gui/widgets/play_easy.png"),
            },
            alignment,
            coords: (x, y),
            game
        };
        Box::new(ret)
    }
}

impl Widget for PlayWidget {
    fn on_click(&mut self) {
        unsafe{(*self.game).load_demo_level()}
        unsafe{(*self.game).current_screen = Some(HudScreen::create(&mut *self.game))}
        //(*self.game).unwrap().current_screen = None;
    }

    fn get_selected(&mut self) -> bool {
        self.selected
    }

    fn set_selected(&mut self, tf : bool) {
        self.selected = tf;
    }

    fn get_screen_coordinates(&mut self) -> (i32, i32) {
        self.coords.clone()
    }

    fn set_screen_coordinates(&mut self, x: i32, y: i32) {
        self.coords = (x, y)
    }

    fn get_asset_data(&mut self) -> AssetData {

        let dyslexia = unsafe { &mut *self.game }.dyslexia_mode.clone();

        match dyslexia {
            DyslexiaMode::ON => {
                if self.selected {
                    self.asset_data_selected_dyslexia.clone()
                }
                else {
                    self.asset_data_dyslexia.clone()
                }
            }
            DyslexiaMode::OFF => {
                if self.selected {
                    self.asset_data_selected.clone()
                }
                else {
                    self.asset_data.clone()
                }
            }
        }


    }

    fn set_asset_data(&mut self, ass: AssetData) {
        self.asset_data = ass
    }

    fn get_resource_location(&mut self) -> ResourceLocation {
        ResourceLocation::new("game", "widgets/play")
    }


    fn get_allignment(&mut self) -> Alignment {
        self.alignment.clone()
    }

    fn set_allignment(&mut self, alignment: Alignment) {
        self.alignment = alignment;
    }

    fn get_game(&mut self) {
        self.game;
    }


}