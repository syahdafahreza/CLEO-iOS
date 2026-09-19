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
    // Addresses are for GTA III v1.3.2 (armv7 / 32-bit) on iPhone 5, iOS 10.x.
    // TEXT segment base: 0x00001000  DATA segment base: 0x001BA000
    // Binary: GTA III (com.rockstargames.gta3ios) 1.3.2
    // -------------------------------------------------------------------------

    use super::{c_char, create_hard_target, create_soft_target, Object, Sel};

    // Main script update loop — CTheScripts::Process()
    #[cfg(target_pointer_width = "32")]
    create_soft_target!(script_tick, 0x00049734, fn());

    // Touch/input handling — called by EAGLView touchesBegan/Moved/Ended/Cancelled
    #[cfg(target_pointer_width = "32")]
    create_soft_target!(process_touch, 0x000054e0, extern "C" fn(u32, u32, u32, u32, u32));

    // GXT string lookup — CText::Get(this, key)
    #[cfg(target_pointer_width = "32")]
    create_soft_target!(
        get_gxt_string,
        0x00057e80,
        fn(usize, *const c_char) -> *const u16
    );

    // Legal splash screen hooks
    create_soft_target!(legal_splash, 0x00097ad4, fn(*mut Object, sel: Sel));
    create_soft_target!(legal_splash_german, 0x000328b2, fn(*mut Object, sel: Sel));

    // Store crash fix — [IOSBillingObserver productsRequest:didReceiveResponse:]
    create_soft_target!(
        store_crash_fix,
        0x00008895,
        fn(*mut Object, Sel) -> *const Object
    );

    // License plate generator (GTA 3 uses static textures; kept for compatibility)
    create_soft_target!(gen_plate, 0x00000000, fn(*mut u8, i32) -> bool);

    // Game state machine — CGame::bGameState state machine
    create_soft_target!(do_game_state, 0x00032268, fn());

    // End dragging (UI gesture handler)
    create_soft_target!(
        end_dragging,
        0x000432c0,
        fn(*const Object, Sel, *mut Object, bool)
    );

    // Loading screen messages — LoadingScreen(msg1, msg2)
    create_hard_target!(
        loading_messages,
        0x00098394,
        fn(*const c_char, *const c_char)
    );

    // Height-above-ceiling check — CVehicle::GetHeightAboveRoad()
    create_soft_target!(
        height_above_ceiling,
        0x0009c274,
        fn(usize, f32, usize) -> f32
    );

    // Game init for title/main menu — CGame::Initialise
    create_soft_target!(init_for_title, 0x0008940c, fn(*mut u8));

    // Load player/game settings — CMenuManager::LoadSettings()
    create_soft_target!(load_settings, 0x00107fe8, fn(u64));
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
