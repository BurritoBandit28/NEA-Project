use crate::game::Game;
use crate::widget::Widget;

pub trait Screen {

    #[must_use]
    fn get_widgets(&mut self) -> &mut Vec<Vec<Box<dyn Widget>>>;

    fn add_widget(&mut self, widget : Box<dyn Widget>, x : usize, y : usize) {
        self.get_widgets().get_mut(y).unwrap().insert(x, widget);
    }

    #[must_use]
    fn get_game(&mut self) -> *mut Game;

    #[must_use]
    fn set_game(&mut self, game : *mut Game);

    #[must_use]
    fn box_clone(&self) -> Box<dyn Screen>;

    #[must_use]
    fn create() -> Box<Self> where Self: Sized;

}

impl Clone for Box<dyn Screen> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}


