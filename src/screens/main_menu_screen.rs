use sdl2::event::Event;
use crate::game::{DyslexiaMode, Game};
use crate::screen::Screen;
use crate::widget::{Alignment, Widget};
use crate::widgets::editor_widget::EditorWidget;
use crate::widgets::enum_widget::{EnumWidget, WidgetEnum};
use crate::widgets::play_widget::PlayWidget;
use crate::widgets::quit_widget::QuitWidget;
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


    fn create(game: &mut Game) -> Box<Self>
    where
        Self: Sized
    {
        let mut ret = Self {
            widgets: vec![],
            game,
        };
        ret.add_widget(SourceWidget::create(Alignment::LEFT, 20, 0, game), 0, 0);
        ret.add_widget(PlayWidget::create(Alignment::LEFT, 60, 0, game), 0, 0);
        ret.add_widget(QuitWidget::create(Alignment::LEFT, 60, -40, game), 0, 0);
        ret.add_widget(EditorWidget::create(Alignment::LEFT, 20, 30, game), 0, 0);
        ret.add_widget(EnumWidget::create(Alignment::LEFT, 20, -60, game, DyslexiaMode::OFF, 18, 32), 0, 0);
        Box::new(ret)
    }

    fn tick(&mut self, mousex: u32, mousey: u32, events: Vec<Event>) {

        let mut dyslexia = unsafe { &mut *self.game }.dyslexia_mode.clone();

        for widgets in self.get_widgets() {
            for w in widgets {
                if w.get_resource_location().to_string() == String::from("game:widgets/enum/dyslexia_mode") {
                    dyslexia = DyslexiaMode::get_from_index(w.return_integer_data().unwrap())
                }
            }
        }

        unsafe { &mut *self.game }.dyslexia_mode = dyslexia;

    }
}