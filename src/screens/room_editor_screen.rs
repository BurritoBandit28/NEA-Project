use std::collections::HashMap;
use std::sync::Mutex;
use sdl2::event::Event;
use sdl2::keyboard::Keycode::D;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use crate::entities::dummy::DummyEntity;
use crate::game::Game;
use crate::level::Level;
use crate::render::AssetData;
use crate::resource_location::ResourceLocation;
use crate::screen::Screen;
use crate::tile::TileSize;
use crate::widget::{Alignment, Widget};
use crate::widgets::enum_widget::EnumWidget;

pub struct RoomEditorScreen {
    sf : i32,
    widgets : Vec<Vec<Box<dyn Widget>>>,
    game : *mut Game,
    //editor_level : Level,
    selected_scale : TileSize,
    centre : (f32, f32),
    highlight_index : usize
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

        let dummy_player = DummyEntity::create(game, AssetData::empty());
        let highlight_object = DummyEntity::create(game,
                                                   AssetData {
                                                       uv: Some(Rect::new(0,0,16, 16)),
                                                       origin: (0, 0),
                                                       resource_location: ResourceLocation::new("game", "tiles\\highlight.png"),
                                                   }
        );

        game.entities.push(Box::new(Mutex::new(dummy_player)));
        game.player = Some(game.entities.len() -1 );
        let highlight_index = game.entities.len();
        game.entities.push(Box::new(Mutex::new(highlight_object)));

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

        Box::new(ret)
    }

    fn cycle(&mut self, mousex : u32, mousey : u32, dims : (u32, u32), events: Vec<Event>) {

        unsafe {
            (*self.game).entities.get_mut(self.highlight_index).unwrap().lock().unwrap().set_coords(
                (
                    ((((mousex as i32 / self.selected_scale.get().0 as i32) - (dims.0 / 2) as i32) + (16 * (mousex as i32 / self.selected_scale.get().0 as i32) as u32) as i32) - (1i32 * mousex as i32 / self.selected_scale.get().0 as i32)) as f32,
                    ((((mousey as i32 / self.selected_scale.get().0 as i32) - (dims.1 / 2) as i32) + (16 * (mousey as i32 / self.selected_scale.get().0 as i32) as u32) as i32) - (1i32 * mousey as i32 / self.selected_scale.get().0 as i32)) as f32,
                )
            )
        }

        for widgets in self.get_widgets() {
            for w in widgets {
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
    }
}