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
    // Addresses are for GTA SA v1.09 (armv7 / 32-bit) on iPhone 5, iOS 10.3.4.
    // TEXT segment base: 0x00004000  DATA segment base: 0x004A0000
    // All community-sourced; verify via /var/mobile/Documents/CLEO/cleo.log.
    // -------------------------------------------------------------------------

    use super::{c_char, create_hard_target, create_soft_target, Object, Sel};

    create_soft_target!(script_tick, 0x00159a88, fn());

    #[cfg(target_pointer_width = "64")]
    create_soft_target!(process_touch, 0x1004e831c, fn(f32, f32, f64, f32, u64));
    #[cfg(target_pointer_width = "32")]
    create_soft_target!(process_touch, 0x003ece38, fn(u32, f32, f32, f64));

    create_soft_target!(
        get_gxt_string,
        0x0034f3c0,
        fn(usize, *const c_char) -> *const u16
    );

    create_soft_target!(legal_splash, 0x000b3e6c, fn(*mut Object, sel: Sel));
    create_soft_target!(legal_splash_german, 0x000a5d20, fn(*mut Object, sel: Sel));

    create_soft_target!(
        store_crash_fix,
        0x00007ad0,
        fn(*mut Object, Sel) -> *const Object
    );

    #[cfg(target_pointer_width = "64")]
    create_soft_target!(
        button_hack,
        0x1004ea8c4,
        fn(*const Object, Sel, *mut Object) -> *mut Object
    );
    #[cfg(target_pointer_width = "32")]
    create_soft_target!(
        button_hack,
        0x00017338,
        fn(*const Object, Sel, *mut Object) -> *mut Object
    );

    create_soft_target!(gen_plate, 0x002d3498, fn(*mut u8, i32) -> bool);

    create_soft_target!(idle, 0x001e1164, fn(u64, u64));

    create_soft_target!(cycles_per_millisecond, 0x00208ef4, fn() -> u32);

    create_soft_target!(do_game_state, 0x003c9d10, fn());

    #[cfg(target_pointer_width = "64")]
    create_hard_target!(do_cheats, 0x1001a7f28, fn());
    #[cfg(target_pointer_width = "32")]
    create_hard_target!(do_cheats, 0x000f6bdc, fn());

    create_soft_target!(reset_before_start, 0x00253f6c, fn());

    create_soft_target!(
        find_absolute_path,
        0x003f0ea4,
        fn(i32, *const u8, i32) -> *const u8
    );

    create_soft_target!(init_for_title, 0x002a8114, fn(*mut u8));

    // NOTE: write_fragment_shader and write_vertex_shader are NOT hooked on
    // GTA SA v1.09 armv7 — the shader pipeline is different in this version.
    // They were debug-only on arm64 too (cfg!(feature = "debug")), but the
    // addresses don't exist in this binary.

    create_soft_target!(load_settings, 0x002542ec, fn(u64));

    create_hard_target!(display_fps, 0x001e0c94, fn());

    create_soft_target!(update_pads, 0x001e2b48, fn());

    create_soft_target!(
        load_cd_directory,
        0x00265550,
        fn(*const i8, archive_id: u32)
    );

    create_soft_target!(
        end_dragging,
        0x000ad2e0,
        fn(*const Object, Sel, *mut Object, bool)
    );

    create_hard_target!(
        loading_messages,
        0x002381cc,
        fn(*const c_char, *const c_char)
    );

    #[cfg(target_pointer_width = "64")]
    create_soft_target!(reset_cheats, 0x1001a82f0, fn());
    #[cfg(target_pointer_width = "32")]
    create_soft_target!(reset_cheats, 0x000f61b0, fn());

    create_soft_target!(
        height_above_ceiling,
        0x003b2a74,
        fn(usize, f32, usize) -> f32
    );

    #[cfg(target_pointer_width = "64")]
    create_soft_target!(init_stage_three, 0x1002f9b20, fn(usize));
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
