use std::collections::HashMap;
use sdl2::event::Event;
use sdl2::render::{Texture, WindowCanvas};
use crate::game::Game;
use crate::screen::Screen;
use crate::widget::{Alignment, Widget};
use crate::widgets::editor_widget::EditorWidget;
use crate::widgets::play_widget::PlayWidget;
use crate::widgets::player_health_widget::PlayerHealthWidget;
use crate::widgets::source_widget::SourceWidget;

pub struct HudScreen {
    game : *mut Game,
    widgets : Vec<Vec<Box<dyn Widget>>>,
}

impl Screen for HudScreen {
    fn get_widgets(&mut self) -> &mut Vec<Vec<Box<dyn Widget>>> {
        &mut self.widgets
    }

    fn get_game(&mut self) -> *mut Game {
        self.game
    }

    fn set_game(&mut self, game: *mut Game) {
        self.game = game;
    }

    fn create(game: &mut Game) -> Box<Self>
    where
        Self: Sized
    {
        let mut ret = Self{
            widgets: vec![],
            game,
        };
        println!("{}", game.dims.0);
        ret.add_widget(PlayerHealthWidget::create(Alignment::TOP, (game.dims.0/2) as i32, 0, game), 0, 0);
        Box::new(ret)
    }


}
