//! Sets up CLEO when the library is loaded.

#![feature(map_try_insert)]

use ctor::ctor;
use objc::runtime::Object;
use objc::runtime::Sel;
use std::os::raw::c_char;

mod game;
mod hook;
mod logging;
mod meta;

mod targets {
    #![allow(clippy::unreadable_literal)]

    // -------------------------------------------------------------------------
    // Addresses are for GTA III v1.x (armv7 / 32-bit) on iPhone 5, iOS 10.x.
    // TEXT segment base: 0x00001000  (to be confirmed after IPA analysis)
    // All addresses below are PLACEHOLDERS — fill in after binary analysis!
    // Use: otool -l GTA3.app/GTA3 | grep -A3 "__TEXT"
    // -------------------------------------------------------------------------

    use super::{c_char, create_hard_target, create_soft_target, Object, Sel};

    // Main script update loop — called every frame by CGame::Process
    // TODO: fill in GTA 3 address (look for CTheScripts::Process or similar)
    #[cfg(target_pointer_width = "32")]
    create_soft_target!(script_tick, 0x00000000, fn());

    // Touch/input handling — called when screen is touched
    // TODO: fill in GTA 3 address
    #[cfg(target_pointer_width = "32")]
    create_soft_target!(process_touch, 0x00000000, extern "C" fn(u32, u32, u32, u32, u32));

    // GXT string lookup — used for text display
    // TODO: fill in GTA 3 address (CText::Get or similar)
    #[cfg(target_pointer_width = "32")]
    create_soft_target!(
        get_gxt_string,
        0x00000000,
        fn(usize, *const c_char) -> *const u16
    );

    // Legal splash screen hook — used to init CLEO early
    // TODO: fill in GTA 3 address
    create_soft_target!(legal_splash, 0x00000000, fn(*mut Object, sel: Sel));
    create_soft_target!(legal_splash_german, 0x00000000, fn(*mut Object, sel: Sel));

    // Store crash fix — prevents crash on app store check
    // TODO: fill in GTA 3 address
    create_soft_target!(
        store_crash_fix,
        0x00000000,
        fn(*mut Object, Sel) -> *const Object
    );

    // License plate generator — used in vehicle spawning
    // TODO: fill in GTA 3 address (CPlate::GeneratePlate or similar)
    create_soft_target!(gen_plate, 0x00000000, fn(*mut u8, i32) -> bool);

    // Game state machine — controls loading/in-game state transitions
    // TODO: fill in GTA 3 address (CGame::Process or CFrontEnd::Process)
    create_soft_target!(do_game_state, 0x00000000, fn());

    // End dragging (UI gesture handler)
    // TODO: fill in GTA 3 address
    create_soft_target!(
        end_dragging,
        0x00000000,
        fn(*const Object, Sel, *mut Object, bool)
    );

    // Loading screen messages — hook to show CLEO loading info
    // TODO: fill in GTA 3 address
    create_hard_target!(
        loading_messages,
        0x00000000,
        fn(*const c_char, *const c_char)
    );

    // Height-above-ceiling check — used in vehicle spawn ground detection
    // TODO: fill in GTA 3 address
    create_soft_target!(
        height_above_ceiling,
        0x00000000,
        fn(usize, f32, usize) -> f32
    );

    // Game init for title/main menu
    // TODO: fill in GTA 3 address
    create_soft_target!(init_for_title, 0x00000000, fn(*mut u8));

    // Load player/game settings
    // TODO: fill in GTA 3 address
    create_soft_target!(load_settings, 0x00000000, fn(u64));
}

#[ctor]
fn load() {
    // Load the logging system before everything else so we can log from constructors.
    logging::init();

    if hook::can_hook() {
        log::info!("hook test successful! CLEO should work ok :)");
    } else {
        #[cfg(target_pointer_width = "32")]
        log::warn!("hook test failed on armv7 — this may be a false negative due to Thumb mode function sizing. Continuing anyway...");
        #[cfg(target_pointer_width = "64")]
        log::error!("hook test failed! CLEO probably won't work :( please report this error!");
    }

    log::info!(
        r#"

                         Welcome to CLEO iOS!
             Written by @squ1dd13 (squ1dd13dev@gmail.com).
        Made with ❤️ in Great Britain. Proudly written in Rust.
  Check out the GitHub repo at https://github.com/squ1dd13/CLEO-iOS.
 Need support? Join the Discord server! https://discord.gg/cXwkTUasJU
"#
    );

    // todo: Log game version.
    log::info!("Cargo package version is {}", env!("CARGO_PKG_VERSION"));

    log::info!(
        "game ASLR slide is {:#x}",
        crate::hook::get_game_aslr_offset(),
    );

    // Set up CLEO first.
    meta::init();

    // Load all of our game systems.
    game::init();
}
