//! Replaces parts of the game's streaming system to allow the loading of replacement files inside IMGs,
//! and also manages the loaded replacements.

// hack: The `stream` module is messy, poorly documented and full of hacky code.
// bug: Opcode 0x04ee seems to break when animations have been swapped.

#[cfg(target_pointer_width = "64")]
mod game;
#[cfg(target_pointer_width = "64")]
mod load;
#[cfg(target_pointer_width = "64")]
mod stream;

#[cfg(target_pointer_width = "64")]
pub use load::load_replacements;

#[cfg(target_pointer_width = "64")]
pub fn init() {
    stream::hook();
    load::hook();
}

#[cfg(target_pointer_width = "32")]
pub fn load_replacements(_image_name: &str, _paths: impl Iterator<Item = std::path::PathBuf>) {
    // Archive replacement loading is disabled on 32-bit iPhone 5.
}

#[cfg(target_pointer_width = "32")]
pub fn init() {
    log::info!("Streaming hooks disabled on 32-bit architecture.");
}
