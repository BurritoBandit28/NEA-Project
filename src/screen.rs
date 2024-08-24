use std::collections::HashMap;
use sdl2::render::{Texture, WindowCanvas};
use crate::game::Game;
use crate::widget::Widget;

pub trait Screen {

    #[must_use]
    fn get_widgets(&mut self) -> &mut Vec<Vec<Box<dyn Widget>>>;

    fn add_widget(&mut self, widget : Box<dyn Widget>, x : usize, y : usize) {
        let mut listy = self.get_widgets().get_mut(y);
        if listy.is_none() {
            self.get_widgets().insert(y, vec![])
        }
        self.get_widgets().get_mut(y).unwrap().insert(x, widget);
    }

    #[must_use]
    fn get_game(&mut self) -> *mut Game;

    #[must_use]
    fn set_game(&mut self, game : *mut Game);


    #[must_use]
    fn create(game : &mut Game) -> Box<Self> where Self: Sized;

    fn cycle(&mut self, mousex : u32, mousey : u32, dims : (u32, u32)) {
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

    fn render(&mut self, textures : &HashMap<String, Texture>, sf : i32, canvas : &mut WindowCanvas, dims : (u32, u32)) {
        for widgets in self.get_widgets() {
            for w in widgets {
                w.render(textures, sf, canvas, dims)
            }
        }
    }

}


