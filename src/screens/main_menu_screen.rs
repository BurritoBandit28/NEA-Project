use crate::game::Game;
use crate::screen::Screen;
use crate::widget::{Alignment, Widget};
use crate::widgets::editor_widget::EditorWidget;
use crate::widgets::play_widget::PlayWidget;
use crate::widgets::source_widget::SourceWidget;

pub struct MainMenuScreen{
    widgets : Vec<Vec<Box<dyn Widget>>>,
    game : *mut Game,

}

impl Screen for MainMenuScreen {
    fn get_widgets(&mut self) -> &mut Vec<Vec<Box<dyn Widget>>> {
        &mut self.widgets
    }

    fn get_game(&mut self) -> *mut Game {
        self.game
    }

    fn set_game(&mut self, game: *mut Game) {
        self.game = game;
    }


    fn create(game : &mut Game) -> Box<Self>
    where
        Self: Sized
    {
        let mut ret = Self{
            widgets: vec![],
            game,
        };
        ret.add_widget(SourceWidget::create(Alignment::LEFT, 20, 0, game), 0, 0);
        ret.add_widget(PlayWidget::create(Alignment::LEFT, 60, 0, game), 0, 0);
        ret.add_widget(EditorWidget::create(Alignment::LEFT, 20, 30, game), 0,0);
        Box::new(ret)
    }

}