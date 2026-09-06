//! Provides CLEO's extra features (60 FPS, FPS counter, etc.).

use libc::c_char;

use crate::{
    call_original, hook,
    meta::settings::{FpsVisibility, Options},
    targets,
};
use std::ffi::CStr;

// CTimer::GetCyclesPerMillisecond is called between the FPS limit being set and when it is
// enforced, so if we overwrite the limit here, our new value will be enforced.
fn cycles_per_millisecond() -> u32 {
    let fps_cap = Options::get().fps_lock.fps();

    unsafe {
        // GTA SA v1.09 armv7: CTimer::ms_fTimeStep / FPS cap global
        *hook::slide::<*mut u32>(0x006b8044) = fps_cap;
    }

    call_original!(targets::cycles_per_millisecond)
}

fn idle(p1: u64, p2: u64) {
    let show_fps = matches!(Options::get().fps_visibility, FpsVisibility::Visible);

    unsafe {
        // GTA SA v1.09 armv7: FPS display flag
        *hook::slide::<*mut bool>(0x00644e01) = show_fps;
    }

    call_original!(targets::idle, p1, p2);
}

#[repr(C)]
struct Rgba {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

fn display_fps() {
    // GTA SA v1.09 armv7 addresses for FPS counter internals.
    let delta_time = crate::hook::slide::<fn() -> u32>(0x003f1fa4)();
    let current_delta = crate::hook::slide::<*mut isize>(0x005f80e0);
    let new_delta_index = unsafe { *current_delta } % 40;

    unsafe {
        *current_delta += 1;
    }

    let delta_times: *mut u32 = crate::hook::slide(0x005f8040);

    unsafe {
        delta_times.offset(new_delta_index).write(delta_time);
    }

    // eq: CFont::SetBackground(...)
    crate::hook::slide::<fn(u8, u8)>(0x002e53b0)(1, 0);

    // eq: CFont::SetBackgroundColor(...)
    crate::hook::slide::<fn(*const Rgba)>(0x002e53c4)(&Rgba {
        red: 0,
        green: 0,
        blue: 0,
        alpha: 180,
    });

    // eq: CFont::SetScale(...)
    crate::hook::slide::<fn(f32)>(0x002e5200)(1.12);

    // eq: CFont::SetOrientation(...)
    crate::hook::slide::<fn(u32)>(0x002e5400)(0);

    // eq: CFont::SetJustify(...)
    crate::hook::slide::<fn(u8)>(0x002e53f0)(0);

    // eq: CFont::SetCentreSize(...)
    crate::hook::slide::<fn(f32)>(0x002e52f0)(200.0);

    // eq: CFont::SetProportional(...)
    crate::hook::slide::<fn(u8)>(0x002e53a0)(0);

    // eq: CFont::SetFontStyle(...)
    crate::hook::slide::<fn(u8)>(0x002e5140)(1);

    // eq: CFont::SetEdge(...)
    crate::hook::slide::<fn(u8)>(0x002e5374)(0);

    // eq: CFont::SetColor(...)
    crate::hook::slide::<fn(*const Rgba)>(0x002e5044)(&Rgba {
        red: 9,
        green: 243,
        blue: 11,
        alpha: 255,
    });

    let fps = unsafe {
        let delta_last_frame = *delta_times.offset((*current_delta - 1) % 40);
        let delta_this_frame = *delta_times.offset(*current_delta % 40);
        let delta = delta_last_frame - delta_this_frame;

        39000.0 / delta as f32
    };

    // CFont::PrintString expects UTF16, so encode our FPS string as such.
    let mut bytes: Vec<u16> = format!("FPS: {fps:.2}").encode_utf16().collect();
    bytes.push(0);

    let (x, y) = unsafe {
        // GTA SA v1.09 armv7: screen dimensions globals
        let screen_wide = *crate::hook::slide::<*const i32>(0x006b803c);
        let screen_high = *crate::hook::slide::<*const i32>(0x006b8040);

        (screen_wide as f32 * 0.5, screen_high as f32 * 0.05)
    };

    // eq: CFont::PrintString(...)
    crate::hook::slide::<fn(f32, f32, *const u16)>(0x002e41e8)(x, y, bytes.as_ptr());
}

/// A game shader.
#[derive(Clone, Copy)]
enum Shader {
    /// Fragment shader with bitmask.
    Fragment(u32),

    /// Vertex shader with bitmask.
    Vertex(u32),
}

impl Shader {
    /// Returns the name of the file used to store the shader.
    fn file_name(self) -> String {
        let (mask, ext) = match self {
            Shader::Fragment(mask) => (mask, "frag"),
            Shader::Vertex(mask) => (mask, "vert"),
        };

        format!("{mask:032b}.{ext}")
    }

    /// Returns the path of the file that this shader can be read from or written to.
    fn file_path(self) -> std::path::PathBuf {
        crate::meta::resources::shaders_path().join(self.file_name())
    }

    /// Returns the contents of the shader's file.
    fn read_custom(self) -> std::io::Result<String> {
        std::fs::read_to_string(self.file_path())
    }

    /// Writes `contents` to the shader's file on disk.
    fn write(self, contents: &str) -> std::io::Result<()> {
        std::fs::write(self.file_path(), contents)
    }

    /// Returns a pointer to the buffer that the game uses for this shader.
    fn buffer(self) -> *const i8 {
        // NOTE: Shader buffer addresses are NOT available in GTA SA v1.09 armv7.
        // This function should never be called since shader hooks are disabled.
        let static_addr = match self {
            Shader::Fragment(_) => 0x0usize,
            Shader::Vertex(_) => 0x0usize,
        };

        crate::hook::slide(static_addr)
    }
}

/// Returns the shader buffer as a string slice.
fn read_shader_buffer(buffer: *const i8) -> &'static str {
    unsafe { CStr::from_ptr(buffer) }
        .to_str()
        .unwrap_or("/* unable to read shader buffer */")
}

/// Overwrites the contents of the shader buffer with `source`.
fn replace_shader_buffer(buffer: *mut i8, source: &str) -> Result<(), std::ffi::NulError> {
    let c_string = std::ffi::CString::new(source)?;

    for (i, byte) in c_string.into_bytes_with_nul().into_iter().enumerate() {
        unsafe { buffer.add(i).write(byte as i8) };
    }

    Ok(())
}

/// Replaces the shader's code if there is a replacement on disk, or dumps the shader to disk if
/// there is no replacement.
fn debug_shader(shader: Shader) {
    let file_name = shader.file_name();
    let buffer = shader.buffer();

    log::info!("handling shader {file_name}");

    if let Ok(custom_shader) = shader.read_custom() {
        if custom_shader.as_str() != read_shader_buffer(buffer) {
            log::info!("shader differs: {file_name}");
        }

        if let Err(err) = replace_shader_buffer(buffer as *mut i8, &custom_shader) {
            log::warn!("not modifying shader buffer because shader was invalid: {err:?}");
        }
    } else {
        // If there was no custom shader to read, write the current shader to disk.
        if let Err(err) = shader.write(read_shader_buffer(buffer)) {
            log::warn!("could not dump shader: {err:?}");
        }
    }
}

// Shader dumping is disabled on GTA SA v1.09 armv7.
// fn write_fragment_shader(mask: u32) {
//     call_original!(crate::targets::write_fragment_shader, mask);
//     let shader = Shader::Fragment(mask);
//     debug_shader(shader);
// }

// fn write_vertex_shader(mask: u32) {
//     call_original!(crate::targets::write_vertex_shader, mask);
//     let shader = Shader::Vertex(mask);
//     debug_shader(shader);
// }

fn set_loading_messages(msg_1: *const c_char, msg_2: *const c_char) {
    unsafe {
        let msg_1 = std::ffi::CStr::from_ptr(msg_1).to_str().unwrap_or("???");
        let msg_2 = std::ffi::CStr::from_ptr(msg_2).to_str().unwrap_or("???");

        log::info!("{}: {}", msg_1, msg_2);
    }
}

// fn height_above_ceiling(veh: usize, f: f32, flight_model: usize) -> f32 {
//     if Settings::shared().no_ceiling.load(Ordering::SeqCst) {
//         -1.0
//     } else {
//         call_original!(targets::height_above_ceiling, veh, f, flight_model)
//     }
// }

pub fn init() {
    log::info!("installing extra hooks...");

    targets::idle::install(idle);
    targets::cycles_per_millisecond::install(cycles_per_millisecond);

    // NOTE: write_fragment_shader and write_vertex_shader hooks are DISABLED
    // on GTA SA v1.09 armv7. The shader system is different in this binary.
    // They were only used in debug builds on arm64 anyway.

    targets::display_fps::install(display_fps);
    #[cfg(target_pointer_width = "64")]
    targets::loading_messages::install(set_loading_messages);

    // create_soft_target!(do_game_state, 0x003c9d10, fn());
    // targets::height_above_ceiling::install(height_above_ceiling);
}
