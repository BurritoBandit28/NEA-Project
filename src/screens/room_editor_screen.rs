use std::collections::HashMap;
use std::sync::Mutex;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Keycode::D;
use sdl2::mouse::MouseButton;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use crate::entities::dummy::DummyEntity;
use crate::entity::Entity;
use crate::game::Game;
use crate::level::{Level, TileGraph};
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
    highlight_index : usize,
    tiles : HashMap<String, Tile>,

}

impl RoomEditorScreen {

    pub fn add_test_tiles(&mut self, tiles : &HashMap<String, Tile>) {
        let tile = Some(tiles.get("game:tiles/wall.json").unwrap().clone());
        let tile3 = Some(tiles.get("game:tiles/dirt.json").unwrap().clone());
        let tile4 = Some(tiles.get("game:tiles/orange.json").unwrap().clone());
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
        dummy_player.set_coords((0.0, 0.0));
        game.entities.push(Box::new(Mutex::new(dummy_player)));
        game.player = Some(game.entities.len() -1 );
        let highlight_index = game.entities.len();
        game.entities.push(Box::new(Mutex::new(highlight_object)));

        let tile  = game.tiles.get("game:tiles/wall.json");

        let mut level = Level {
            tile_big: TileGraph::create(TileSize::BIG),
            tile_medium: TileGraph::create(TileSize::MEDIUM),
            tile_small: TileGraph::create(TileSize::SMALL),
            tile_nav: TileGraph::create(TileSize::SMALL),
        };

        let mut ret = Self{
            sf : game.sf,
            widgets: vec![],
            game,
            //editor_level : level,
            selected_scale : TileSize::SMALL,
            centre : (0.0, 0.0),
            highlight_index,
            tiles : game.tiles.clone()
        };

        ret.add_widget(EnumWidget::create(Alignment::TOP, 6, 18, game, TileSize::SMALL),0 ,0);

        game.current_level = Some(level);

        ret.add_test_tiles(&game.tiles);

        Box::new(ret)
    }

    fn cycle(&mut self, mousex : u32, mousey : u32, dims : (u32, u32), events: Vec<Event>) {



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

        let mut player_coords = unsafe{(*self.game).entities.get_mut((*self.game).player.unwrap()).unwrap().lock().unwrap().get_coords()};

        unsafe {(*self.game).entities.get_mut((*self.game).player.unwrap()).unwrap().lock().unwrap().set_coords((player_coords.0, player_coords.1))}

        player_coords = unsafe{(*self.game).entities.get_mut((*self.game).player.unwrap()).unwrap().lock().unwrap().get_coords()};

        let px = (if player_coords.0 < 0.0 {player_coords.0 - 1.0} else { player_coords.0 }) as i32;
        let py = (if player_coords.1 < 0.0 {player_coords.1 - 1.0} else { player_coords.1 }) as i32;
        let mouse_coord_x = mousex as i32 - (160 - px);
        let mouse_coord_y = mousey as i32 - (90 - py);

        let mouse_x_fixed = if mouse_coord_x < 0 {(mouse_coord_x - self.selected_scale.get().0 as i32)/ self.selected_scale.get().0 as i32} else { (mouse_coord_x)/ self.selected_scale.get().0 as i32 };
        let mouse_y_fixed = if mouse_coord_y < 0 {(mouse_coord_y - self.selected_scale.get().0 as i32)/ self.selected_scale.get().0 as i32} else { (mouse_coord_y)/ self.selected_scale.get().0 as i32 };


        let x = (((mouse_x_fixed) + (self.selected_scale.get().0 as i32 * mouse_x_fixed)) - mouse_x_fixed) as f32;
        let y = (((mouse_y_fixed) + (self.selected_scale.get().0 as i32 * mouse_y_fixed)) - mouse_y_fixed) as f32;

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

        for e in events {
            match e {
                Event::KeyDown {
                    keycode: Some(Keycode::B),
                    ..
                } => {

                },
                Event::MouseButtonDown {
                    mouse_btn : MouseButton::Left,
                    ..
                } => {
                    unsafe {(*self.game).current_level.as_mut().unwrap().tile_small.append_from_wolrd_space(
                        self.tiles.get(&ResourceLocation::new("game", "tiles/cardboard_box.json").to_string()).unwrap().clone(),
                        (x as i32,y as i32),
                        vec![])}
                }
                _ => {}
            }
        }

    }
}