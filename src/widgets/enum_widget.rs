use log::warn;
use sdl2::keyboard::Keycode::N;
use sdl2::rect::Rect;
use crate::game::Game;
use crate::render::AssetData;
use crate::resource_location::ResourceLocation;
use crate::widget::{Alignment, Widget};
use crate::widgets::err_widget::ErrWidget;

pub struct EnumWidget<T : WidgetEnum> {
    enum_type : T,
    current_indx : usize,
    selected : bool,
    alignment: Alignment,
    coords : (i32, i32),
    asset_data: AssetData,
    asset_data_selected : AssetData,
    game : *mut Game,
}

impl<T : WidgetEnum + 'static + Clone> EnumWidget<T> {

    fn get_value(&mut self) -> T {
        self.enum_type.clone()
    }

    pub fn create(alignment: Alignment, x: i32, y: i32, game: *mut Game, enumt : T) -> Box<dyn Widget> {
        let ret = Self {
            enum_type:enumt ,
            current_indx: 0,
            selected: false,
            asset_data: AssetData {
                uv: Some(Rect::new(0, 0, 20, 20)),
                origin: (0, 0),
                resource_location: ResourceLocation::empty(),
            },
            asset_data_selected: AssetData {
                uv: Some(Rect::new(0, 20, 20, 20)),
                origin: (0, 0),
                resource_location: ResourceLocation::empty(),
            },
            alignment,
            coords: (x, y),
            game

        };
        Box::new(ret)
    }
}

impl<T : WidgetEnum> Widget for EnumWidget<T> {

    /// Cycle through the enum values when clicked
    fn on_click(&mut self) {
        // If on the last field, when clicked loop back to the first one
        if self.current_indx == self.enum_type.count() -1 {
            self.current_indx = 0;
        }
        else {
            // otherwise increment by one
            self.current_indx +=1;
        }
        // Use the integer value to get the Enum field.
        self.enum_type = T::get_from_index(self.current_indx);
    }

    // basic widget boilerplate
    fn get_selected(&mut self) -> bool {
        self.selected
    }

    fn set_selected(&mut self, tf: bool) {
        self.selected = tf
    }

    fn get_screen_coordinates(&mut self) -> (i32, i32) {
        self.coords.clone()
    }

    fn set_screen_coordinates(&mut self, x: i32, y: i32) {
        self.coords = (x, y)
    }

    // The asset data should be auto generated for the enum type.
    fn get_asset_data(&mut self) -> AssetData {

        // using the name of the enums different values to get the textures allows dynamic generation of the resource location
        let rl = ResourceLocation::new("game", format!("gui/widgets/enum/{}/{}.png", self.enum_type.name(), self.enum_type.get_as_string()).as_str());
        self.asset_data.resource_location = rl.clone();
        self.asset_data_selected.resource_location = rl;

        if self.selected {
            self.asset_data_selected.clone()
        }
        else {
            self.asset_data.clone()
        }
    }

    // more boilerplate
    fn set_asset_data(&mut self, ass: AssetData) {
        self.asset_data = ass
    }

    fn get_resource_location(&mut self) -> ResourceLocation {
        ResourceLocation::new("game", format!("widgets/enum/{}", self.enum_type.name()).as_str())
    }

    fn get_allignment(&mut self) -> Alignment {
        self.alignment.clone()
    }

    fn set_allignment(&mut self, alignment: Alignment) {
        self.alignment = alignment
    }

    fn get_game(&mut self) {
        // why is there a getter that returns nothing...? what was I on
    }

    fn return_integer_data(&mut self) -> Option<usize> {
        Some(self.current_indx)
    }

}

pub trait WidgetEnum {

    /// Gets the name of the Enum field as a string
    fn get_as_string(&mut self) -> String;

    /// Get an Enum value from an integer
    fn get_from_index(index : usize) -> Self;

    /// Number of fields within the Enum
    fn count(&mut self) -> usize;

    /// The name of the Enum that contains all the fields, for example "tile_size", which contains "large", "medium" and "small"
    fn name(&mut self) -> String;
}