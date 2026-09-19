//! Helper module for interacting with the player ped (Claude), stats, money, weapons, and vehicle spawning.
//! GTA III (armv7 / 32-bit) iOS — all addresses are PLACEHOLDERS until IPA is analysed.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use crate::hook;
use lazy_static::lazy_static;

pub static INFINITE_HEALTH: AtomicBool = AtomicBool::new(false);
pub static INFINITE_AMMO: AtomicBool = AtomicBool::new(false);
pub static INFINITE_SPRINT: AtomicBool = AtomicBool::new(false);
pub static FAST_RELOAD: AtomicBool = AtomicBool::new(false);
pub static NEVER_WANTED: AtomicBool = AtomicBool::new(false);
pub static GOD_MODE_VEHICLE: AtomicBool = AtomicBool::new(false);

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
/// TODO: fill in GTA 3 address for FindPlayerPed()
pub fn find_player_ped() -> *mut u8 {
    #[cfg(target_pointer_width = "32")]
    {
        // TODO: fill in GTA 3 address — FindPlayerPed() or CWorld::FindPlayerPed
        hook::slide_fn::<extern "C" fn() -> *mut u8>(0x00000000)()
    }
    #[cfg(target_pointer_width = "64")]
    {
        std::ptr::null_mut()
    }
}

/// Returns the pointer to the vehicle Claude is currently in (`CVehicle*`), or null.
/// TODO: fill in GTA 3 address for FindPlayerVehicle()
pub fn find_player_vehicle() -> *mut u8 {
    #[cfg(target_pointer_width = "32")]
    {
        // TODO: fill in GTA 3 address — FindPlayerVehicle() or CWorld::FindPlayerVehicle
        hook::slide_fn::<extern "C" fn() -> *mut u8>(0x00000000)()
    }
    #[cfg(target_pointer_width = "64")]
    {
        std::ptr::null_mut()
    }
}

/// Returns a pointer to the active `CPlayerInfo` struct.
/// TODO: fill in GTA 3 address and struct offsets (CWorld::Players, etc.)
pub fn get_player_info_ptr() -> *mut u8 {
    #[cfg(target_pointer_width = "32")]
    {
        // TODO: fill in GTA 3 DATA addresses:
        //   - g_nCurrentPlayerFocusIndex (which player index is active)
        //   - CWorld::Players base pointer
        //   - Correct CPlayerInfo struct size (probably different from VC 0x174)
        let focus_ptr = hook::slide::<*const u8>(0x00000000); // TODO: g_nCurrentPlayerFocusIndex
        let focus = if !focus_ptr.is_null() {
            unsafe { *focus_ptr as usize }
        } else {
            0
        };

        let players_base = hook::slide::<*mut u8>(0x00000000); // TODO: CWorld::Players
        if players_base.is_null() {
            std::ptr::null_mut()
        } else {
            // TODO: fill in correct CPlayerInfo struct size for GTA 3
            // GTA VC used 0x174 — GTA 3 may differ. Check after IPA analysis.
            unsafe { players_base.add(focus * 0x000) } // TODO: replace 0x000 with real size
        }
    }
    #[cfg(target_pointer_width = "64")]
    {
        std::ptr::null_mut()
    }
}

/// Gets Claude's current money.
/// TODO: fill in correct offset of money field inside CPlayerInfo for GTA 3
pub fn get_money() -> i32 {
    let info = get_player_info_ptr();
    if !info.is_null() {
        // TODO: fill in correct money offset for GTA 3 CPlayerInfo struct
        unsafe { *(info.add(0x000) as *const i32) } // TODO: replace 0x000 with real offset
    } else {
        0
    }
}

/// Modifies Claude's money (both internal bank and display HUD).
/// TODO: fill in correct money offsets inside CPlayerInfo for GTA 3
pub fn apply_money(amount: i32, is_delta: bool) {
    let info = get_player_info_ptr();
    if !info.is_null() {
        unsafe {
            // TODO: fill in correct money offset and display money offset for GTA 3
            let money_ptr = info.add(0x000) as *mut i32;         // TODO: real offset
            let display_money_ptr = info.add(0x000) as *mut i32; // TODO: real offset (may be same as money)
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
/// TODO: fill in all GTA 3 addresses before using.
pub fn spawn_vehicle_direct(model_id: u32) -> *mut u8 {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        let ped = find_player_ped();
        if ped.is_null() {
            return std::ptr::null_mut();
        }

        // 1. Request and load model synchronously
        // TODO: fill in GTA 3 address for CStreaming::RequestModel
        hook::slide_fn::<extern "C" fn(u32, u32)>(0x00000000)(model_id, 1);
        // TODO: fill in GTA 3 address for CStreaming::LoadAllRequestedModels
        hook::slide_fn::<extern "C" fn(u8)>(0x00000000)(0);

        // 2. Read player position from CMatrix at ped + 0x04
        // TODO: verify these struct offsets in GTA 3 — may differ from VC
        let px = *(ped.add(0x34) as *const f32); // TODO: verify position offset
        let py = *(ped.add(0x38) as *const f32); // TODO: verify position offset
        let pz = *(ped.add(0x3c) as *const f32); // TODO: verify position offset

        let fx = *(ped.add(0x14) as *const f32); // TODO: verify forward vector offset
        let fy = *(ped.add(0x18) as *const f32); // TODO: verify forward vector offset

        // Normalize 2D forward vector
        let len = (fx * fx + fy * fy).sqrt();
        let (dir_x, dir_y) = if len > 0.001 {
            (fx / len, fy / len)
        } else {
            (0.0, 1.0)
        };

        let heading = (-dir_x).atan2(dir_y);
        let veh_heading = heading + std::f32::consts::FRAC_PI_2;

        // 3. Determine vehicle class
        // TODO: verify these model IDs and classification functions for GTA 3
        let is_bike = hook::slide_fn::<extern "C" fn(u32) -> u32>(0x00000000)(model_id) != 0; // TODO: IsModelBike
        let is_boat = hook::slide_fn::<extern "C" fn(u32) -> u32>(0x00000000)(model_id) != 0; // TODO: IsModelBoat

        let in_car = !find_player_vehicle().is_null();
        let dist = if in_car {
            8.5f32
        } else if is_boat {
            10.0f32
        } else if is_bike {
            3.5f32
        } else {
            5.0f32
        };

        let spawn_x = px + dir_x * dist;
        let spawn_y = py + dir_y * dist;

        // 4. Query ground elevation via CWorld::FindGroundZForCoord
        // TODO: fill in GTA 3 address for CWorld::FindGroundZForCoord
        let ground_z_raw = hook::slide_fn::<extern "C" fn(u32, u32) -> u32>(0x00000000)(
            spawn_x.to_bits(),
            spawn_y.to_bits(),
        );
        let ground_z = f32::from_bits(ground_z_raw);
        let spawn_z = if ground_z > -50.0 && (ground_z - pz).abs() < 12.0 {
            ground_z
        } else {
            pz
        };

        // 5. Allocate vehicle memory
        // TODO: fill in GTA 3 operator new address, and verify struct sizes
        let (size, veh_type) = if is_boat {
            (0x000usize, 0u8) // TODO: CBoat size in GTA 3
        } else if is_bike {
            (0x000usize, 1u8) // TODO: CBike size in GTA 3
        } else {
            (0x000usize, 2u8) // TODO: CAutomobile size in GTA 3
        };

        // TODO: fill in GTA 3 operator new address
        let veh = hook::slide_fn::<extern "C" fn(usize) -> *mut u8>(0x00000000)(size);
        if veh.is_null() {
            return std::ptr::null_mut();
        }

        // 6. Invoke vehicle C++ constructor
        // TODO: fill in GTA 3 constructor addresses for CBoat, CBike, CAutomobile
        match veh_type {
            0 => {
                hook::slide_fn::<extern "C" fn(*mut u8, u32, u8) -> *mut u8>(0x00000000)(veh, model_id, 2); // TODO: CBoat::CBoat
            }
            1 => {
                hook::slide_fn::<extern "C" fn(*mut u8, u32, u8) -> *mut u8>(0x00000000)(veh, model_id, 1); // TODO: CBike::CBike
            }
            _ => {
                hook::slide_fn::<extern "C" fn(*mut u8, u32, u8) -> *mut u8>(0x00000000)(veh, model_id, 1); // TODO: CAutomobile::CAutomobile
            }
        }

        // 7. Get suspension height and set position & orientation
        // TODO: fill in GTA 3 address for suspension height function
        let height_raw = hook::slide_fn::<extern "C" fn(*mut u8) -> u32>(0x00000000)(veh);
        let height = f32::from_bits(height_raw);
        let final_z = spawn_z + if height > 0.05 && height < 5.0 { height } else if is_bike { 0.25 } else { 0.4 };

        // Set vehicle rotation
        // TODO: fill in GTA 3 address for CMatrix::SetRotate
        hook::slide_fn::<extern "C" fn(*mut u8, u32, u32, u32)>(0x00000000)(
            veh.add(4),
            0.0f32.to_bits(),
            0.0f32.to_bits(),
            veh_heading.to_bits(),
        );

        // Set coordinates — TODO: verify position offsets for GTA 3
        *(veh.add(0x34) as *mut f32) = spawn_x; // TODO: verify offset
        *(veh.add(0x38) as *mut f32) = spawn_y; // TODO: verify offset
        *(veh.add(0x3c) as *mut f32) = final_z; // TODO: verify offset

        // 8. Configure status
        // TODO: verify these flag offsets in GTA 3 CVehicle struct
        let status_flags = veh.add(0x52) as *mut u8; // TODO: verify offset
        *status_flags = (*status_flags & !0x38) | (4 << 3);

        if veh_type == 2 {
            *(veh.add(0x000) as *mut u32) = 1; // TODO: unlocked doors offset
        } else if veh_type == 0 {
            *(veh.add(0x000) as *mut f32) = 20.0; // TODO: boat fuel offset
            *(veh.add(0x000) as *mut u8) = 20;    // TODO: boat fuel offset
        }

        // 9. Add vehicle entity to CWorld
        // TODO: fill in GTA 3 address for CWorld::Add
        hook::slide_fn::<extern "C" fn(*mut u8)>(0x00000000)(veh);

        // 10. Mark model as deletable
        // TODO: fill in GTA 3 address for CStreaming::SetModelIsDeletable
        hook::slide_fn::<extern "C" fn(u32)>(0x00000000)(model_id);

        veh
    }
    #[cfg(target_pointer_width = "64")]
    {
        std::ptr::null_mut()
    }
}

/// Queues a vehicle to be spawned right in front of Tommy.
pub fn queue_spawn_vehicle(model_id: u32) {
    if let Ok(mut q) = QUEUED_VEHICLE.lock() {
        *q = Some(model_id);
    }
}

/// Returns the streaming model IDs required for a given weapon ID in GTA III.
/// Weapon model IDs are from GTA III iOS.
/// TODO: Verify all model IDs after binary IPA analysis.
pub fn get_weapon_models(weapon_id: u32) -> &'static [u32] {
    match weapon_id {
        // GTA III Weapon IDs & their model IDs
        // Slot 0 — Unarmed
        0  => &[],
        // Slot 1 — Melee (Brass Knuckles)
        1  => &[259],
        // Slot 2 — Melee (Screwdriver)
        2  => &[260],
        // Slot 3 — Melee (Golf Club)
        3  => &[261],
        // Slot 4 — Melee (Nightstick)
        4  => &[262],
        // Slot 5 — Melee (Knife)
        5  => &[263],
        // Slot 6 — Melee (Baseball Bat)
        6  => &[264],
        // Slot 7 — Grenade
        7  => &[342],  // TODO: verify model ID
        // Slot 8 — Rocket Launcher
        8  => &[343],  // TODO: verify model ID
        // Slot 9 — Flamethrower
        9  => &[344],  // TODO: verify model ID
        // Slot 10 — Molotov Cocktail
        10 => &[345],  // TODO: verify model ID
        // Slot 11 — Pistol
        11 => &[346],  // TODO: verify model ID
        // Slot 12 — Shotgun
        12 => &[347],  // TODO: verify model ID
        // Slot 13 — Uzi / Micro Uzi
        13 => &[348],  // TODO: verify model ID
        // Slot 14 — AK-47
        14 => &[349],  // TODO: verify model ID
        // Slot 15 — M16
        15 => &[350],  // TODO: verify model ID
        // Slot 16 — Sniper Rifle
        16 => &[351],  // TODO: verify model ID
        // Slot 17 — Rocket Launcher (RPG)
        17 => &[352],  // TODO: verify model ID
        _  => &[],
    }
}

/// Queues a weapon to be given to Claude with ammo.
pub fn queue_give_weapon(weapon_id: u32, ammo: u32) {
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
                        // TODO: fill in GTA 3 address for CPlayerPed::MakePlayerSafe or SetHealth
                        hook::slide_fn::<extern "C" fn(u32)>(0x00000000)(1);
                    }
                    PlayerAction::FullArmor => {
                        // TODO: fill in GTA 3 address for giving armor
                        hook::slide_fn::<extern "C" fn()>(0x00000000)();
                    }
                    PlayerAction::ClearWanted => {
                        let ped = find_player_ped();
                        if !ped.is_null() {
                            unsafe {
                                // TODO: fill in CWanted* offset in Claude's CPed struct for GTA 3
                                let wanted = *(ped.add(0x000) as *const *mut u8); // TODO: CWanted* offset
                                if !wanted.is_null() {
                                    // TODO: fill in GTA 3 address for CWanted::SetWantedLevel
                                    hook::slide_fn::<extern "C" fn(*mut u8, u32)>(0x00000000)(wanted, 0);
                                }
                            }
                        }
                        // TODO: fill in GTA 3 address for ClearWantedLevel helper
                        hook::slide_fn::<extern "C" fn()>(0x00000000)();
                    }
                    PlayerAction::RaiseWanted => {
                        let ped = find_player_ped();
                        if !ped.is_null() {
                            unsafe {
                                // TODO: fill in CWanted* offset for GTA 3
                                let wanted = *(ped.add(0x000) as *const *mut u8); // TODO: CWanted* offset
                                if !wanted.is_null() {
                                    // TODO: fill in CWanted wanted level offset for GTA 3
                                    let current = *(wanted.add(0x000) as *const u32); // TODO: wanted level offset
                                    let new_lvl = (current + 2).min(6);
                                    // TODO: fill in GTA 3 address for CWanted::SetWantedLevel
                                    hook::slide_fn::<extern "C" fn(*mut u8, u32)>(0x00000000)(wanted, new_lvl);
                                }
                            }
                        } else {
                            // TODO: fill in GTA 3 address for RaiseWantedLevel helper
                            hook::slide_fn::<extern "C" fn()>(0x00000000)();
                        }
                    }
                    PlayerAction::RepairCurrentVehicle => {
                        let veh = find_player_vehicle();
                        if !veh.is_null() {
                            unsafe {
                                // TODO: verify CVehicle health offset for GTA 3
                                *(veh.add(0x000) as *mut f32) = 1000.0; // TODO: vehicle health offset
                                // TODO: fill in GTA 3 address for CVehicle::Fix
                                hook::slide_fn::<extern "C" fn(*mut u8)>(0x00000000)(veh);

                                // TODO: verify model ID offset and bike/boat classification for GTA 3
                                let model_id = *(veh.add(0x000) as *const u16) as u32; // TODO: model ID offset
                                let is_bike = hook::slide_fn::<extern "C" fn(u32) -> u32>(0x00000000)(model_id) != 0; // TODO
                                let is_boat = hook::slide_fn::<extern "C" fn(u32) -> u32>(0x00000000)(model_id) != 0; // TODO

                                if !is_bike && !is_boat {
                                    // TODO: fill in GTA 3 address for CAutomobile::Fix or similar
                                    hook::slide_fn::<extern "C" fn(*mut u8)>(0x00000000)(veh);
                                } else {
                                    // TODO: verify these flag offsets in GTA 3 CBike/CBoat struct
                                    *(veh.add(0x000) as *mut u8) &= !2;    // TODO: offset
                                    *(veh.add(0x000) as *mut u8) &= !0x3f; // TODO: offset
                                }
                            }
                        }
                    }
                }
            }
        }

        // 2. Process queued vehicle spawn (spawns directly in front of Claude)
        if let Ok(mut q) = QUEUED_VEHICLE.lock() {
            if let Some(model_id) = q.take() {
                spawn_vehicle_direct(model_id);
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
                        // TODO: fill in GTA 3 address for CStreaming::RequestModel
                        hook::slide_fn::<extern "C" fn(u32, u32)>(0x00000000)(mid, 1);
                    }
                }

                // 3b. Synchronously load all requested models
                // TODO: fill in GTA 3 address for CStreaming::LoadAllRequestedModels
                hook::slide_fn::<extern "C" fn(u8)>(0x00000000)(0);

                // 3c. Give weapons to Claude
                for &(wid, ammo) in &weapons {
                    // TODO: fill in GTA 3 address for CPed::GiveWeapon or CWeaponInfo::GiveWeapon
                    hook::slide_fn::<extern "C" fn(*mut u8, u32, u32, u32) -> u32>(0x00000000)(
                        ped, wid, ammo, 1,
                    );
                }

                // 3d. Mark models as deletable so CStreaming cache is managed normally
                for &(wid, _) in &weapons {
                    for &mid in get_weapon_models(wid) {
                        // TODO: fill in GTA 3 address for CStreaming::SetModelIsDeletable
                        hook::slide_fn::<extern "C" fn(u32)>(0x00000000)(mid);
                    }
                }
            }
        }

        // 4. Handle persistent toggle states
        let ped = find_player_ped();
        if !ped.is_null() {
            // TODO: verify bleeding flag offsets in GTA 3 CPed struct (may not exist)
            // unsafe {
            //     *(ped.add(0x000) as *mut u8) &= !4; // TODO: bIsBleeding bit
            //     *(ped.add(0x000) as *mut u8) = 0;   // TODO: m_nBleeding timer
            // }

            // Infinite Health / God Mode (locks Claude to 250 HP and 250 Armor)
            if INFINITE_HEALTH.load(Ordering::Relaxed) {
                unsafe {
                    // TODO: verify health and armor offsets in GTA 3 CPed struct
                    *(ped.add(0x000) as *mut f32) = 250.0; // TODO: health offset
                    *(ped.add(0x000) as *mut f32) = 250.0; // TODO: armor offset
                }

                // If inside a vehicle, keep vehicle fully repaired
                let veh = find_player_vehicle();
                if !veh.is_null() {
                    unsafe {
                        // TODO: verify CVehicle health offset for GTA 3
                        *(veh.add(0x000) as *mut f32) = 1000.0; // TODO: vehicle health offset
                    }
                }
            }

            // Vehicle God Mode / Mobil Kebal
            let current_veh = find_player_vehicle();
            if !current_veh.is_null() {
                if GOD_MODE_VEHICLE.load(Ordering::Relaxed) {
                    unsafe {
                        // TODO: verify all these CVehicle offsets for GTA 3
                        *(current_veh.add(0x000) as *mut f32) = 1000.0;      // TODO: vehicle health
                        *(current_veh.add(0x000) as *mut u32) |= 0x002f0000; // TODO: immunity flags
                        *(current_veh.add(0x000) as *mut u8) |= 2;           // TODO: tyre proof flag
                        *(current_veh.add(0x000) as *mut u8) = 0;            // TODO: engine status
                    }
                }
            }

            // Infinite Ammo: keep ammo and clip full across all weapon slots
            // TODO: verify weapon slot count and struct layout for GTA 3 CPed
            // GTA 3 has different number of weapon slots compared to VC (12 vs 10)
            if INFINITE_AMMO.load(Ordering::Relaxed) {
                unsafe {
                    // TODO: fill in correct weapon slots base offset and slot stride for GTA 3
                    for slot in 0..12 { // GTA 3 has 12 weapon slots
                        let wep_ptr = ped.add(0x000 + slot * 0x000); // TODO: base and stride offsets
                        let wep_type = *(wep_ptr as *const u32);
                        if wep_type > 0 {
                            let clip_ptr = wep_ptr.add(0x000) as *mut u32;  // TODO: clip offset
                            let total_ptr = wep_ptr.add(0x000) as *mut u32; // TODO: total ammo offset
                            if *total_ptr < 9000 {
                                *total_ptr = 9999;
                            }
                            if *clip_ptr < 50 {
                                *clip_ptr = 99;
                            }
                        }
                    }
                }
            }
        }

        // Never Wanted / Anti Polisi:
        // Claude's CWanted — offsets need to be found via IPA analysis for GTA 3.
        // TODO: fill in GTA 3 DATA addresses for MaximumWantedLevel and MaximumChaosLevel
        let max_wanted_ptr = hook::slide::<*mut i32>(0x00000000); // TODO: CWanted::MaximumWantedLevel
        let max_chaos_ptr = hook::slide::<*mut i32>(0x00000000);  // TODO: CWanted::MaximumChaosLevel
        if NEVER_WANTED.load(Ordering::Relaxed) {
            unsafe {
                if !max_wanted_ptr.is_null() {
                    *max_wanted_ptr = 0;
                }
                if !max_chaos_ptr.is_null() {
                    *max_chaos_ptr = 0;
                }

                if !ped.is_null() {
                    // TODO: fill in CWanted* offset in GTA 3 CPed struct
                    let wanted = *(ped.add(0x000) as *const *mut u8); // TODO: CWanted* offset
                    if !wanted.is_null() {
                        *(wanted.add(0x00) as *mut u32) = 0;
                        // TODO: fill in CWanted wanted level offset for GTA 3
                        let stars = *(wanted.add(0x000) as *const u32); // TODO: wanted level offset
                        if stars > 0 {
                            // TODO: fill in GTA 3 address for CWanted::SetWantedLevel
                            hook::slide_fn::<extern "C" fn(*mut u8, u32)>(0x00000000)(wanted, 0);
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
                    *max_chaos_ptr = 7200; // 0x1c20
                }
            }
        }

        let info = get_player_info_ptr();
        if !info.is_null() {
            // TODO: verify infinite sprint flag offset in GTA 3 CPlayerInfo
            if INFINITE_SPRINT.load(Ordering::Relaxed) {
                unsafe {
                    *(info.add(0x000) as *mut u8) = 1; // TODO: sprint flag offset
                }
            }
            // TODO: verify fast reload flag offset in GTA 3 CPlayerInfo
            if FAST_RELOAD.load(Ordering::Relaxed) {
                unsafe {
                    *(info.add(0x000) as *mut u8) = 1; // TODO: fast reload flag offset
                }
            }
        }
    }
}
