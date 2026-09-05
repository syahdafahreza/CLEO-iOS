//! CLEO's own systems that are added to the game. These do not modify game systems other than to
//! integrate CLEO into the game.

mod github;
pub mod gui;
pub mod language;
pub mod menu;
pub mod resources;
pub mod settings;
pub mod touch;
mod update;

pub const COPYRIGHT_NAMES: &str =
    "squ1dd13, AYZM, Bruno Melo, Flylarb, ODIN, RAiZOK, tharryz, wewewer1";

pub fn init() {
    settings::init();
    language::init();
    update::init();
    gui::init();
    menu::init();
    touch::init();
    resources::init();

    // Background update check thread is disabled on iPhone 5
    // github::start_update_check_thread();
}
