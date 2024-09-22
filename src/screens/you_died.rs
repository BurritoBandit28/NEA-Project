use crate::game::Game;
use crate::screen::Screen;
use crate::widget::{Alignment, Widget};
use crate::widgets::death_message::DeathMessage;
use crate::widgets::player_health_widget::PlayerHealthWidget;
use crate::widgets::source_widget::SourceWidget;

pub struct DeathScreen {
    game : *mut Game,
    widgets : Vec<Vec<Box<dyn Widget>>>,
}

impl Screen for DeathScreen{
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
        ret.add_widget(DeathMessage::create(Alignment::NONE, 0, 0, game), 0, 0);
        Box::new(ret)
    }


}
