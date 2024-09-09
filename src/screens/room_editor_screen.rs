use std::collections::HashMap;
use std::sync::Mutex;
use sdl2::event::Event;
use sdl2::keyboard::Keycode::D;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use crate::entities::dummy::DummyEntity;
use crate::entity::Entity;
use crate::game::Game;
use crate::level::Level;
use crate::render::AssetData;
use crate::resource_location::ResourceLocation;
use crate::screen::Screen;
use crate::tile::{Tile, TileSize};
use crate::widget::{Alignment, Widget};
use crate::widgets::enum_widget::{EnumWidget, WidgetEnum};

pub struct RoomEditorScreen {
    sf : i32,
    widgets : Vec<Vec<Box<dyn Widget>>>,
    game : *mut Game,
    //editor_level : Level,
    selected_scale : TileSize,
    centre : (f32, f32),
    highlight_index : usize
}

impl RoomEditorScreen {

    pub fn add_test_tiles(&mut self, tiles : &HashMap<String, Tile>) {
        let tile = Some(tiles.get("game:tiles/wall.json").unwrap().clone());
        let tile3 = Some(tiles.get("game:tiles/dirt.json").unwrap().clone());
        let tile4 = Some(tiles.get("game:tiles/orange.json").unwrap().clone());

        unsafe {
            (*self.game).current_level.as_mut().unwrap().tile_big.push(vec![tile]);
            (*self.game).current_level.as_mut().unwrap().tile_medium.push(vec![None, None, tile4]);
            (*self.game).current_level.as_mut().unwrap().tile_small.push(vec![None, None,None,None,None,None,tile3]);

        }
    }

}

impl Screen for RoomEditorScreen {
    fn get_widgets(&mut self) -> &mut Vec<Vec<Box<dyn Widget>>> {
        &mut self.widgets
    }


    fn get_game(&mut self) -> *mut Game {
        self.game
    }

    fn set_game(&mut self, game: *mut Game) {
        self.game = game
    }

    fn create(game: &mut Game) -> Box<Self>
    where
        Self: Sized
    {

        //game.draw_mouse = false;

        let mut dummy_player = DummyEntity::create(game, AssetData::empty());
        let highlight_object = DummyEntity::create(game,
                                                   AssetData {
                                                       uv: Some(Rect::new(0,0,16, 16)),
                                                       origin: (0, 0),
                                                       resource_location: ResourceLocation::new("game", "tiles/highlight.png"),
                                                   }
        );
        dummy_player.set_coords((160.0, 90.0));
        game.entities.push(Box::new(Mutex::new(dummy_player)));
        game.player = Some(game.entities.len() -1 );
        let highlight_index = game.entities.len();
        game.entities.push(Box::new(Mutex::new(highlight_object)));

        let tile  = game.tiles.get("game:tiles/wall.json");

        let mut level = Level {
            tile_big : vec![],
            tile_medium: vec![],
            tile_small: vec![],
            path_gird: vec![],
        };

        let mut ret = Self{
            sf : game.sf,
            widgets: vec![],
            game,
            //editor_level : level,
            selected_scale : TileSize::SMALL,
            centre : (0.0, 0.0),
            highlight_index
        };

        ret.add_widget(EnumWidget::create(Alignment::TOP, 6, 18, game, TileSize::SMALL),0 ,0);

        game.current_level = Some(level);

        ret.add_test_tiles(&game.tiles);

        Box::new(ret)
    }

    fn cycle(&mut self, mousex : u32, mousey : u32, dims : (u32, u32), events: Vec<Event>) {

        let player_coords = unsafe{(*self.game).entities.get_mut((*self.game).player.unwrap()).unwrap().lock().unwrap().get_coords()};

        let mouse_x_fixed = (mousex as i32) / self.selected_scale.get().0 as i32;
        let mouse_y_fixed = (mousey as i32) / self.selected_scale.get().0 as i32;

        let mut scale_indx = 0usize;

        for widgets in self.get_widgets() {
            for w in widgets {

                if w.get_resource_location().to_string() == String::from("game:widgets/enum/tile_size") {
                    scale_indx = w.return_enum_int().unwrap();
                }

                let _ = w.set_selected(false);
                let mut coords = w.correct_coords(dims);

                let uv = w.get_asset_data().uv;

                // test - println!("coords x : {} - {} \ncoords y : {} - {} \nmouse x : {} \nmouse y : {}", coords.0, coords.0 as u32 + uv.unwrap().width(), coords.1, coords.1 as u32 + uv.unwrap().height(), mousex, mousey);

                if (coords.0 <= mousex as i32 && coords.0 as u32 + uv.unwrap().width() > mousex) && (coords.1 <= mousey as i32 && coords.1 as u32 + uv.unwrap().height() > mousey) {
                    let _ = w.set_selected(true);
                    break
                }
            }
        }
        self.selected_scale = TileSize::get_from_index(scale_indx);

        let x = (((mouse_x_fixed - 160) + (self.selected_scale.get().0 * mouse_x_fixed as u32) as i32) - mouse_x_fixed) as f32 + player_coords.0;
        let y = (((mouse_y_fixed - 90) + (self.selected_scale.get().0 * mouse_y_fixed as u32) as i32) - mouse_y_fixed) as f32 + player_coords.1;

        unsafe {
            let mut ass = (*self.game).entities.get_mut(self.highlight_index).unwrap().lock().unwrap().get_asset_data();
            (*self.game).entities.get_mut(self.highlight_index).unwrap().lock().unwrap().set_asset_data(
                AssetData {
                    resource_location : ass.resource_location.clone(),
                    uv : {
                        let mut rect = ass.clone().uv.unwrap();
                        rect.set_width(self.selected_scale.get().0);
                        rect.set_height(self.selected_scale.get().0);
                        Some(rect)
                    },
                    origin: (0, 0),
                }
            );
            (*self.game).entities.get_mut(self.highlight_index).unwrap().lock().unwrap().set_coords(
                (
                    x,
                    y
                )
            )
        }
    }
}