use iced::Theme;

use crate::app::App;

pub mod app;
pub mod currency;
pub mod sim;

fn main() {
    iced::application("OpenEco", App::update, App::view)
        .theme(|_| Theme::Dark)
        .run_with(App::new)
        .expect("Failed to run the application");
}
