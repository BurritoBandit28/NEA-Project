use std::collections::HashMap;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use crate::game::Game;
use crate::render;
use crate::render::AssetData;
use crate::resource_location::ResourceLocation;
use crate::widget::{Alignment, Widget};

pub struct PlayerHealthWidget {
    selected : bool,
    asset_data: AssetData,
    asset_data_selected : AssetData,
    alignment: Alignment,
    coords : (i32, i32),
    game : *mut Game,
    half : bool
}

impl PlayerHealthWidget {
    pub fn create(alignment: Alignment, x : i32, y : i32, game : *mut Game) -> Box<Self>
    where
        Self: Sized
    {

        let ret = Self {
            selected: false,
            asset_data: AssetData {
                uv: Some(Rect::new(0, 0, 15, 15)),
                origin: (0, 0),
                resource_location: ResourceLocation::new("game", "gui/hud/hearts.png"),
            },
            asset_data_selected: AssetData {
                uv: Some(Rect::new(0, 16, 15, 15)),
                origin: (0, 0),
                resource_location: ResourceLocation::new("game", "gui/hud/hearts.png"),
            },
            alignment,
            coords: (x, y),
            game,
            half : false
        };
        Box::new(ret)
    }
}

impl Widget for PlayerHealthWidget {
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
        if self.half {
            self.asset_data_selected.clone()
        }
        else {
            self.asset_data.clone()
        }
    }

    fn set_asset_data(&mut self, ass: AssetData) {
        self.asset_data = ass
    }

    fn get_resource_location(&mut self) -> ResourceLocation {
        ResourceLocation::new("game", "widgets/player_health")
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

    fn render(&mut self, textures: &HashMap<String, Texture>, sf: i32, canvas: &mut WindowCanvas, dims: (u32, u32), debug : bool) {
        let game = unsafe { &mut *self.game };
        let health = f32::max(game.get_player().unwrap().get_mut().unwrap().get_health(), 0.0);
        let coords = self.correct_coords(dims);
        self.half = false;
        if health > 0.0 {
            for mut h in 0..(health/2.0).ceil() as u32 {
                if health % 2.0 != 0.0  && h+1 == (health/2.0).ceil() as u32 {
                    self.half = true;
                }
                if debug {
                    render::draw_pp_texture(coords.0 - (14 * (h+1)) as i32, coords.1, &Widget::get_debug_asset_data(self), canvas, sf, textures)
                }
                render::draw_pp_texture(coords.0 - (14 * (h+1)) as i32, coords.1, &self.get_asset_data(), canvas, sf, textures)
            }
        }



    }

}