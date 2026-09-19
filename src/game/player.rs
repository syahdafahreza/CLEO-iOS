//! Helper module for interacting with the player ped (Claude), stats, money, weapons, and vehicle spawning.
//! GTA III v1.3.2 (armv7 / 32-bit) iOS on iPhone 5, iOS 10.x.

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Mutex;
use crate::hook;
use lazy_static::lazy_static;

pub static INFINITE_HEALTH: AtomicBool = AtomicBool::new(false);
pub static INFINITE_AMMO: AtomicBool = AtomicBool::new(false);
pub static INFINITE_SPRINT: AtomicBool = AtomicBool::new(false);
pub static FAST_RELOAD: AtomicBool = AtomicBool::new(false);
pub static NEVER_WANTED: AtomicBool = AtomicBool::new(false);
pub static GOD_MODE_VEHICLE: AtomicBool = AtomicBool::new(false);
static QUEUED_VEHICLE_RETRIES: AtomicU32 = AtomicU32::new(0);

lazy_static! {
    static ref QUEUED_VEHICLE: Mutex<Option<u32>> = Mutex::new(None);
    static ref QUEUED_WEAPONS: Mutex<Vec<(u32, u32)>> = Mutex::new(vec![]);
    static ref QUEUED_ACTIONS: Mutex<Vec<PlayerAction>> = Mutex::new(vec![]);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlayerAction {
    AddMoney(i32),
    SetMoney(i32),
    FullHealthAndRepair,
    FullArmor,
    ClearWanted,
    RaiseWanted,
    RepairCurrentVehicle,
}

/// Returns the pointer to Claude (`CPed*`).
/// GTA III v1.3.2 ARMv7: FindPlayerPed() is at 0x000F4CC8.
pub fn find_player_ped() -> *mut u8 {
    #[cfg(target_pointer_width = "32")]
    {
        hook::slide_fn::<extern "C" fn() -> *mut u8>(0x000f4cc8)()
    }
    #[cfg(target_pointer_width = "64")]
    {
        std::ptr::null_mut()
    }
}

/// Returns the pointer to the vehicle Claude is currently in (`CVehicle*`), or null.
/// GTA III v1.3.2 ARMv7: FindPlayerVehicle() is at 0x000F4C14.
pub fn find_player_vehicle() -> *mut u8 {
    #[cfg(target_pointer_width = "32")]
    {
        hook::slide_fn::<extern "C" fn() -> *mut u8>(0x000f4c14)()
    }
    #[cfg(target_pointer_width = "64")]
    {
        std::ptr::null_mut()
    }
}

/// Returns a pointer to the active `CPlayerInfo` struct.
/// GTA III v1.3.2 ARMv7:
///   PlayerInFocus is at 0x003796E4
///   CWorld::Players is at 0x003175F0
///   sizeof(CPlayerInfo) = 0x13C (316 bytes)
pub fn get_player_info_ptr() -> *mut u8 {
    #[cfg(target_pointer_width = "32")]
    {
        let focus_ptr = hook::slide::<*const u8>(0x003796e4);
        let focus = if !focus_ptr.is_null() {
            unsafe { *focus_ptr as usize }
        } else {
            0
        };

        let players_base = hook::slide::<*mut u8>(0x003175f0);
        if players_base.is_null() {
            std::ptr::null_mut()
        } else {
            unsafe { players_base.add(focus * 0x13c) }
        }
    }
    #[cfg(target_pointer_width = "64")]
    {
        std::ptr::null_mut()
    }
}

/// Gets Claude's current money.
/// GTA III v1.3.2 ARMv7: m_nMoney is at CPlayerInfo + 0xAC.
pub fn get_money() -> i32 {
    let info = get_player_info_ptr();
    if !info.is_null() {
        unsafe { *(info.add(0xac) as *const i32) }
    } else {
        0
    }
}

/// Modifies Claude's money (both internal bank and display HUD).
/// GTA III v1.3.2 ARMv7:
///   m_nMoney is at +0xAC
///   m_nDisplayMoney is at +0xB0
pub fn apply_money(amount: i32, is_delta: bool) {
    let info = get_player_info_ptr();
    if !info.is_null() {
        unsafe {
            let money_ptr = info.add(0xac) as *mut i32;
            let display_money_ptr = info.add(0xb0) as *mut i32;
            if is_delta {
                *money_ptr = (*money_ptr).saturating_add(amount);
            } else {
                *money_ptr = amount;
            }
            *display_money_ptr = *money_ptr;
        }
    }
}

/// Queues an instant player action to be executed safely on the game thread.
pub fn queue_action(action: PlayerAction) {
    if let Ok(mut q) = QUEUED_ACTIONS.lock() {
        q.push(action);
    }
}

/// Spawns a vehicle directly in front of Claude facing the same direction as Claude.
/// GTA III v1.3.2 ARMv7 implementation:
///   - CStreaming::RequestModel: 0x0011AEF0
///   - CStreaming::LoadAllRequestedModels: 0x0011D954
///   - CWorld::FindGroundZForCoord: 0x000385F0
/// Helper to check if model has finished streaming and is ready for instantiation.
/// In GTA III, `ms_aInfoForModel` is an array of 20-byte CStreamingInfo structs.
/// Pointer to `ms_aInfoForModel` is stored at DATA address 0x001ba6a4.
/// Struct offset 8 is `m_nLoadState` (1 = LOADED / READING_SUCCESS).
pub fn is_model_loaded(model_id: u32) -> bool {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        let ms_a_info_pptr = hook::slide::<*const *const u8>(0x001ba6a4);
        if ms_a_info_pptr.is_null() || (*ms_a_info_pptr).is_null() {
            return false;
        }
        let info = (*ms_a_info_pptr).add(model_id as usize * 20);
        let load_state = *info.add(8);
        load_state == 1
    }
    #[cfg(target_pointer_width = "64")]
    false
}

/// Spawns a vehicle directly in front of Claude using Rockstar's native vehicle creation logic.
///
/// Sequence mirrors GTA III's native CCheat::VehicleCheat (re3 reference):
///   1. CStreaming::RequestModel (0x0011AEF0, flag 1 = GAME_REQUIRED)
///   2. CStreaming::LoadAllRequestedModels (0x0011D954)
///   3. operator new (0x001388DC)
///   4. CAutomobile::CAutomobile(veh, model_id, RANDOM_VEHICLE=1) (0x0009C23C, size 0x488)
///   5. CMatrix::SetRotateZ on the vehicle matrix (0x0005A904)
///   6. Write position directly into the embedded CMatrix
///   7. CWorld::Add(veh) (0x0003B090)
///
/// IMPORTANT: CWorld::AlignToGroundAndRoof (0x000C5064) is intentionally NOT called here.
/// When spawning near bridges/overpasses it would compress the vehicle between the
/// ground and the bridge underside, causing "invisible rectangle" visual artifact,
/// extreme down-force physics, and wheel instability. The native VehicleCheat does
/// not call AlignToGroundAndRoof either.
pub fn spawn_vehicle_direct(model_id: u32) -> *mut u8 {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        let ped = find_player_ped();
        if ped.is_null() {
            return std::ptr::null_mut();
        }

        // 1. Ensure model is loaded via CStreaming (use flag 1 = GAME_REQUIRED to force immediate load)
        if !is_model_loaded(model_id) {
            log::info!("spawn_vehicle_direct: Model {} not loaded, requesting with GAME_REQUIRED (1)", model_id);
            hook::slide_fn::<extern "C" fn(u32, u32)>(0x0011aef0)(model_id, 1);
            hook::slide_fn::<extern "C" fn(u8)>(0x0011d954)(0);
            if !is_model_loaded(model_id) {
                // Streaming is still pending; return null so caller retries on next frame
                log::info!("spawn_vehicle_direct: Model {} still loading...", model_id);
                return std::ptr::null_mut();
            }
        }

        // 2. Read player position from CPlaceable at ped + 0x34..0x3C
        let px = *(ped.add(0x34) as *const f32);
        let py = *(ped.add(0x38) as *const f32);
        let pz = *(ped.add(0x3c) as *const f32);

        let fx = *(ped.add(0x14) as *const f32);
        let fy = *(ped.add(0x18) as *const f32);

        // Normalize 2D forward vector
        let len = (fx * fx + fy * fy).sqrt();
        let (dir_x, dir_y) = if len > 0.001 {
            (fx / len, fy / len)
        } else {
            (0.0, 1.0)
        };

        // Determine vehicle class: Boats in GTA III iOS: 120 (predator), 142 (speeder), 143 (reefer), 150 (ghost)
        let is_boat = matches!(model_id, 120 | 142 | 143 | 150);
        let in_car = !find_player_vehicle().is_null();
        let dist = if in_car {
            8.5f32
        } else if is_boat {
            10.0f32
        } else {
            6.5f32
        };

        let spawn_x = px + dir_x * dist;
        let spawn_y = py + dir_y * dist;

        // Query ground Z using CWorld::FindGroundZForCoord (0x00038B48)
        let gz_bits = hook::slide_fn::<extern "C" fn(u32, u32) -> u32>(0x00038b48)(
            spawn_x.to_bits(),
            spawn_y.to_bits(),
        );
        let ground_z = f32::from_bits(gz_bits);
        // If ground_z is within 3 meters of player, use it; otherwise reject bogus raycast results (like 20.0f default)
        let base_z = if (ground_z - pz).abs() < 3.0 && !ground_z.is_nan() {
            ground_z
        } else {
            pz - 0.5f32
        };

        if is_boat {
            // Allocate CBoat (0x5AC bytes)
            let veh = hook::slide_fn::<extern "C" fn(usize) -> *mut u8>(0x001388dc)(0x5ac);
            if veh.is_null() {
                return std::ptr::null_mut();
            }
            // Construct CBoat(veh, model_id, 2)
            hook::slide_fn::<extern "C" fn(*mut u8, u32, u8) -> *mut u8>(0x00068e1c)(veh, model_id, 2);

            let spawn_z = pz + 0.2f32;

            // Set boat orientation using native CMatrix::SetRotate (0x0005A9D0).
            // Args: (matrix, angle_x = 0, angle_y = 0, angle_z = heading)
            // Softfp ABI: passed in r0, r1, r2, r3. Keeps roll & pitch strictly 0 (100% upright).
            let heading = (-dir_x).atan2(dir_y);
            hook::slide_fn::<extern "C" fn(*mut u8, u32, u32, u32)>(0x0005a9d0)(
                veh.add(0x04),
                0,
                0,
                heading.to_bits(),
            );

            // Set spawn coordinates into the embedded CMatrix position (veh+0x34..0x3C)
            *(veh.add(0x34) as *mut f32) = spawn_x;
            *(veh.add(0x38) as *mut f32) = spawn_y;
            *(veh.add(0x3c) as *mut f32) = spawn_z;

            // Zero linear & angular momentum (0x000B31F0)
            hook::slide_fn::<extern "C" fn(*mut u8)>(0x000b31f0)(veh);

            // Set status to STATUS_ABANDONED (4)
            let status_flags = veh.add(0x53) as *mut u8;
            *status_flags = (*status_flags & !0x38) | (4 << 3);

            // Add vehicle entity to CWorld
            hook::slide_fn::<extern "C" fn(*mut u8)>(0x0003b090)(veh);
            log::info!("spawn_vehicle_direct: Boat {} spawned at ({:.2}, {:.2}, {:.2})", model_id, spawn_x, spawn_y, spawn_z);
            veh
        } else {
            // Allocate CAutomobile (0x488 bytes)
            let veh = hook::slide_fn::<extern "C" fn(usize) -> *mut u8>(0x001388dc)(0x488);
            if veh.is_null() {
                return std::ptr::null_mut();
            }

            // Construct CAutomobile(veh, model_id, 2)
            // Matches native GTA III CREATE_CAR opcode 00A5 and CCheat::VehicleCheat (0x000C0A04).
            hook::slide_fn::<extern "C" fn(*mut u8, u32, u8) -> *mut u8>(0x0009c23c)(veh, model_id, 2);

            // Calculate height above ground using model collision bounds.
            // GetDistanceFromCentreOfMassToBaseOfModel(veh) → half-height of vehicle model.
            let h_bits = hook::slide_fn::<extern "C" fn(*mut u8) -> u32>(0x0002c970)(veh);
            let height_from_base = f32::from_bits(h_bits);
            let valid_height = if height_from_base.is_nan() || height_from_base <= 0.0 || height_from_base > 5.0 {
                0.75f32   // safe default for most GTA III cars (~sedan half-height)
            } else {
                height_from_base
            };
            // Spawn the car with tyres just touching the ground (+0.15m clearance)
            let spawn_z = base_z + valid_height + 0.15f32;

            // Set vehicle orientation using native CMatrix::SetRotate (0x0005A9D0).
            // Args: (matrix, angle_x = 0, angle_y = 0, angle_z = heading).
            // Passing 0 for X (pitch) and 0 for Y (roll) guarantees the vehicle is 100% upright,
            // fixing the bug where the car was pitched/rolled 90 degrees onto its side.
            // Preserves RenderWare matrix flags (0x10, 0x20, 0x30).
            let heading = (-dir_x).atan2(dir_y);
            hook::slide_fn::<extern "C" fn(*mut u8, u32, u32, u32)>(0x0005a9d0)(
                veh.add(0x04),
                0,
                0,
                heading.to_bits(),
            );

            // Write position into the embedded CMatrix translation vector (veh + 0x34..0x3C).
            *(veh.add(0x34) as *mut f32) = spawn_x;
            *(veh.add(0x38) as *mut f32) = spawn_y;
            *(veh.add(0x3c) as *mut f32) = spawn_z;

            // Zero linear & angular momentum (0x000B31F0 - matches native CREATE_CAR)
            hook::slide_fn::<extern "C" fn(*mut u8)>(0x000b31f0)(veh);

            // Set status to STATUS_ABANDONED (4): bits 3-5 in the entity flags byte at +0x53.
            let status_flags = veh.add(0x53) as *mut u8;
            *status_flags = (*status_flags & !0x38) | (4 << 3);

            // Mark as cheated car (bIsCheatedCar flag at veh + 0x1F9, bit 3).
            *(veh.add(0x1f9) as *mut u8) |= 8;

            // Native CREATE_CAR opcode 00A5 suspension & physics setup (disassembled from 0x00045696):
            //   +0x15E: bStuckOnRound flag → 0
            //   +0x15F: wheel-stuck bitfield → 0
            //   +0x15D: wheel-stuck timer → 0
            //   +0x164: max spring length (f32) → 9.0f (0x41100000)
            //   +0x168: spring-iterations byte → 9
            //   +0x15B, +0x15C: 0
            //   +0x1F9: &= !0x10
            //   +0x1FB: |= 4
            *(veh.add(0x15e) as *mut u8) = 0;
            *(veh.add(0x15f) as *mut u8) = 0;
            *(veh.add(0x15d) as *mut u8) = 0;
            *(veh.add(0x164) as *mut f32) = 9.0f32;
            *(veh.add(0x168) as *mut u8) = 9;
            *(veh.add(0x15b) as *mut u8) = 0;
            *(veh.add(0x15c) as *mut u8) = 0;
            *(veh.add(0x1f9) as *mut u8) &= !0x10;
            *(veh.add(0x1fb) as *mut u8) |= 4;

            // Add vehicle entity to CWorld
            hook::slide_fn::<extern "C" fn(*mut u8)>(0x0003b090)(veh);
            log::info!(
                "spawn_vehicle_direct: Automobile {} spawned at ({:.2}, {:.2}, {:.2}) heading={:.2}rad",
                model_id, spawn_x, spawn_y, spawn_z, heading
            );
            veh
        }
    }
    #[cfg(target_pointer_width = "64")]
    {
        std::ptr::null_mut()
    }
}

/// Queues a vehicle to be spawned right in front of Claude.
pub fn queue_spawn_vehicle(model_id: u32) {
    log::info!("queue_spawn_vehicle: Queuing model ID {}", model_id);
    if let Ok(mut q) = QUEUED_VEHICLE.lock() {
        *q = Some(model_id);
    }
}

/// Returns the streaming model IDs required for a given weapon ID in GTA III.
/// Weapon model IDs are verified from GTA III iOS default.ide & weapon.dat:
///   1: Baseball Bat  -> Model 172
///   2: Colt45        -> Model 173
///   3: Uzi           -> Model 178
///   4: Shotgun       -> Model 176
///   5: AK47          -> Model 171
///   6: M16           -> Model 180
///   7: Sniper Rifle  -> Model 177
///   8: Rocket Lnchr  -> Model 175
///   9: FlameThrower  -> Model 181
///  10: Molotov       -> Model 174
///  11: Grenade       -> Model 170
pub fn get_weapon_models(weapon_id: u32) -> &'static [u32] {
    match weapon_id {
        0  => &[],       // Unarmed
        1  => &[172],    // Baseball Bat
        2  => &[173],    // Colt45 (Pistol)
        3  => &[178],    // Uzi
        4  => &[176],    // Shotgun
        5  => &[171],    // AK47
        6  => &[180],    // M16
        7  => &[177],    // Sniper Rifle
        8  => &[175],    // Rocket Launcher
        9  => &[181],    // FlameThrower
        10 => &[174],    // Molotov Cocktail
        11 => &[170],    // Grenade
        _  => &[],
    }
}

/// Queues a weapon to be given to Claude with ammo.
pub fn queue_give_weapon(weapon_id: u32, ammo: u32) {
    if weapon_id == 0 || weapon_id > 11 {
        log::warn!("Ignored invalid weapon ID {} for GTA III (must be 1..=11)", weapon_id);
        return;
    }
    if let Ok(mut q) = QUEUED_WEAPONS.lock() {
        q.push((weapon_id, ammo));
    }
}

/// Called per frame inside `script_tick` / `script_update` on the game thread.
pub fn tick() {
    #[cfg(target_pointer_width = "32")]
    {
        // 1. Process queued instant actions
        if let Ok(mut q) = QUEUED_ACTIONS.lock() {
            for action in q.drain(..) {
                match action {
                    PlayerAction::AddMoney(amt) => apply_money(amt, true),
                    PlayerAction::SetMoney(amt) => apply_money(amt, false),
                    PlayerAction::FullHealthAndRepair => {
                        let ped = find_player_ped();
                        if !ped.is_null() {
                            unsafe {
                                // GTA III: Health is at ped + 0x2C4
                                *(ped.add(0x2c4) as *mut f32) = 100.0;
                            }
                        }
                        let veh = find_player_vehicle();
                        if !veh.is_null() {
                            unsafe {
                                *(veh.add(0x204) as *mut f32) = 1000.0;
                                hook::slide_fn::<extern "C" fn(*mut u8, u32)>(0x000cc560)(veh.add(0x28c), 0);
                            }
                        }
                    }
                    PlayerAction::FullArmor => {
                        let ped = find_player_ped();
                        if !ped.is_null() {
                            unsafe {
                                // GTA III: Armor is at ped + 0x2C8
                                *(ped.add(0x2c8) as *mut f32) = 100.0;
                            }
                        }
                    }
                    PlayerAction::ClearWanted => {
                        let ped = find_player_ped();
                        if !ped.is_null() {
                            hook::slide_fn::<extern "C" fn(*mut u8, u32)>(0x0001de64)(ped, 0);
                        }
                    }
                    PlayerAction::RaiseWanted => {
                        let ped = find_player_ped();
                        if !ped.is_null() {
                            unsafe {
                                let wanted = *(ped.add(0x544) as *const *mut u8);
                                if !wanted.is_null() {
                                    let current = *(wanted.add(0x18) as *const u32);
                                    let new_lvl = (current + 2).min(6);
                                    hook::slide_fn::<extern "C" fn(*mut u8, u32)>(0x0002f2d0)(wanted, new_lvl);
                                }
                            }
                        }
                    }
                    PlayerAction::RepairCurrentVehicle => {
                        let veh = find_player_vehicle();
                        if !veh.is_null() {
                            unsafe {
                                *(veh.add(0x204) as *mut f32) = 1000.0;
                                hook::slide_fn::<extern "C" fn(*mut u8, u32)>(0x000cc560)(veh.add(0x28c), 0);
                            }
                        }
                    }
                }
            }
        }

        // 2. Process queued vehicle spawn (spawns directly in front of Claude)
        if let Ok(mut q) = QUEUED_VEHICLE.lock() {
            if let Some(model_id) = *q {
                let veh = spawn_vehicle_direct(model_id);
                if !veh.is_null() {
                    *q = None;
                    QUEUED_VEHICLE_RETRIES.store(0, Ordering::Relaxed);
                } else {
                    let retries = QUEUED_VEHICLE_RETRIES.fetch_add(1, Ordering::Relaxed);
                    if retries >= 300 {
                        // Timeout after ~5 seconds of waiting for streaming
                        *q = None;
                        QUEUED_VEHICLE_RETRIES.store(0, Ordering::Relaxed);
                        log::warn!("Vehicle spawn timed out for model ID {}", model_id);
                    }
                }
            }
        }

        // 3. Process queued weapons (with streaming model loading)
        if let Ok(mut q) = QUEUED_WEAPONS.lock() {
            let ped = find_player_ped();
            if !ped.is_null() && !q.is_empty() {
                let weapons: Vec<(u32, u32)> = q.drain(..).collect();

                // 3a. Request all required models from CStreaming
                for &(wid, _) in &weapons {
                    for &mid in get_weapon_models(wid) {
                        hook::slide_fn::<extern "C" fn(u32, u32)>(0x0011aef0)(mid, 1);
                    }
                }

                // 3b. Synchronously load all requested models
                hook::slide_fn::<extern "C" fn(u8)>(0x0011d954)(0);

                // 3c. Give weapons to Claude via CPed::GiveWeapon(ped, weapon_type, ammo)
                for &(wid, ammo) in &weapons {
                    if wid >= 1 && wid <= 11 {
                        hook::slide_fn::<extern "C" fn(*mut u8, u32, u32)>(0x000de170)(
                            ped, wid, ammo,
                        );
                    }
                }

                // NOTE: CStreaming::SetModelIsDeletable (0x0011c210) is intentionally NOT
                // called here. Weapon models in GTA III must remain resident in memory
                // while the player has them; marking them deletable causes CStreaming to
                // purge them and crash with EXC_BAD_ACCESS when rendered or used.
            }
        }

        // 4. Handle persistent toggle states
        let ped = find_player_ped();
        if !ped.is_null() {
            // Infinite Health / God Mode (locks Claude to 250 HP and 250 Armor)
            if INFINITE_HEALTH.load(Ordering::Relaxed) {
                unsafe {
                    *(ped.add(0x2c4) as *mut f32) = 250.0; // Health
                    *(ped.add(0x2c8) as *mut f32) = 250.0; // Armor
                }

                // If inside a vehicle, keep vehicle fully repaired
                let veh = find_player_vehicle();
                if !veh.is_null() {
                    unsafe {
                        *(veh.add(0x204) as *mut f32) = 1000.0;
                    }
                }
            }

            // Vehicle God Mode / Mobil Kebal
            let current_veh = find_player_vehicle();
            if !current_veh.is_null() {
                if GOD_MODE_VEHICLE.load(Ordering::Relaxed) {
                    unsafe {
                        *(current_veh.add(0x204) as *mut f32) = 1000.0;
                        *(current_veh.add(0x53) as *mut u32) |= 0x002f0000;
                        hook::slide_fn::<extern "C" fn(*mut u8, u32)>(0x000cc560)(current_veh.add(0x28c), 0);
                    }
                }
            }

            // Infinite Ammo: keep total ammo refilled for firearms & throwables (slots 2..=11).
            // Skip slot 0 (unarmed) and slot 1 (baseball bat has no ammo).
            // Clip ammo is managed natively by CWeapon::Update / Reload; do not overwrite clip
            // directly as that breaks non-clip weapons (RPG, Bat, Grenades, Molotovs).
            if INFINITE_AMMO.load(Ordering::Relaxed) {
                unsafe {
                    for slot in 2..=11 {
                        let wep_ptr = ped.add(0x360 + slot * 0x18);
                        let wep_type = *(wep_ptr as *const u32);
                        if wep_type > 0 {
                            let total_ptr = wep_ptr.add(0x0c) as *mut u32;
                            if *total_ptr < 9000 {
                                *total_ptr = 9999;
                            }
                        }
                    }
                }
            }
        }

        // Never Wanted / Anti Polisi:
        // Claude's CWanted is at ped + 0x544.
        // CWanted::m_nChaosLevel is at +0x00, CWanted::m_nWantedLevel is at +0x18.
        // CWanted::MaximumWantedLevel is at 0x001BEBC8, CWanted::MaximumChaosLevel is at 0x001BEBCC.
        let max_wanted_ptr = hook::slide::<*mut i32>(0x001bebc8);
        let max_chaos_ptr = hook::slide::<*mut i32>(0x001bebcc);
        if NEVER_WANTED.load(Ordering::Relaxed) {
            unsafe {
                if !max_wanted_ptr.is_null() {
                    *max_wanted_ptr = 0;
                }
                if !max_chaos_ptr.is_null() {
                    *max_chaos_ptr = 0;
                }

                if !ped.is_null() {
                    let wanted = *(ped.add(0x544) as *const *mut u8);
                    if !wanted.is_null() {
                        *(wanted.add(0x00) as *mut u32) = 0;
                        let stars = *(wanted.add(0x18) as *const u32);
                        if stars > 0 {
                            hook::slide_fn::<extern "C" fn(*mut u8, u32)>(0x0002f2d0)(wanted, 0);
                        }
                    }
                }
            }
        } else {
            unsafe {
                if !max_wanted_ptr.is_null() && *max_wanted_ptr == 0 {
                    *max_wanted_ptr = 6;
                }
                if !max_chaos_ptr.is_null() && *max_chaos_ptr == 0 {
                    *max_chaos_ptr = 6400; // 0x1900
                }
            }
        }

        let info = get_player_info_ptr();
        if !info.is_null() {
            // Infinite sprint flag at CPlayerInfo + 0x114
            if INFINITE_SPRINT.load(Ordering::Relaxed) {
                unsafe {
                    *(info.add(0x114) as *mut u8) = 1;
                }
            }
            // Fast reload flag at CPlayerInfo + 0x115
            if FAST_RELOAD.load(Ordering::Relaxed) {
                unsafe {
                    *(info.add(0x115) as *mut u8) = 1;
                }
            }
        }
    }
}
