//! Helper module for interacting with the player ped (Claude), stats, money, weapons, and vehicle spawning.
//! GTA III v1.3.2 (armv7 / 32-bit) iOS on iPhone 5, iOS 10.x.

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
///   - operator new: 0x001388DC
///   - CAutomobile::CAutomobile: 0x0009C23C (size 0x488)
///   - CBoat::CBoat: 0x00068E1C (size 0x5AC)
///   - CVehicle::GetHeightAboveRoad: 0x0009C274
///   - CMatrix::SetRotate: 0x000C5064
///   - CWorld::Add: 0x0003B090
///   - CStreaming::SetModelIsDeletable: 0x0011C210
pub fn spawn_vehicle_direct(model_id: u32) -> *mut u8 {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        let ped = find_player_ped();
        if ped.is_null() {
            return std::ptr::null_mut();
        }

        // 1. Request and load model synchronously
        hook::slide_fn::<extern "C" fn(u32, u32)>(0x0011aef0)(model_id, 1);
        hook::slide_fn::<extern "C" fn(u8)>(0x0011d954)(0);

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

        let heading = (-dir_x).atan2(dir_y);
        let veh_heading = heading + std::f32::consts::FRAC_PI_2;

        // 3. Determine vehicle class (GTA III has no bikes; only boats and automobiles)
        // Boats in GTA III iOS: 120 (predator), 142 (speeder), 143 (reefer), 150 (ghost)
        let is_boat = matches!(model_id, 120 | 142 | 143 | 150);

        let in_car = !find_player_vehicle().is_null();
        let dist = if in_car {
            8.5f32
        } else if is_boat {
            10.0f32
        } else {
            5.0f32
        };

        let spawn_x = px + dir_x * dist;
        let spawn_y = py + dir_y * dist;

        // 4. Query ground elevation via CWorld::FindGroundZForCoord
        let ground_z_raw = hook::slide_fn::<extern "C" fn(u32, u32) -> u32>(0x000385f0)(
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
        // CBoat: 0x5AC bytes, CAutomobile: 0x488 bytes
        let (size, veh_type) = if is_boat {
            (0x5acusize, 0u8)
        } else {
            (0x488usize, 2u8)
        };

        let veh = hook::slide_fn::<extern "C" fn(usize) -> *mut u8>(0x001388dc)(size);
        if veh.is_null() {
            return std::ptr::null_mut();
        }

        // 6. Invoke vehicle C++ constructor
        match veh_type {
            0 => {
                hook::slide_fn::<extern "C" fn(*mut u8, u32, u8) -> *mut u8>(0x00068e1c)(veh, model_id, 2);
            }
            _ => {
                hook::slide_fn::<extern "C" fn(*mut u8, u32, u8) -> *mut u8>(0x0009c23c)(veh, model_id, 1);
            }
        }

        // 7. Get suspension height and set position & orientation
        let height_raw = hook::slide_fn::<extern "C" fn(*mut u8) -> u32>(0x0009c274)(veh);
        let height = f32::from_bits(height_raw);
        let final_z = spawn_z + if height > 0.05 && height < 5.0 { height } else { 0.4 };

        // Set vehicle orientation & position directly in CMatrix (embedded at veh + 0x04)
        // CMatrix layout (RenderWare RwMatrix):
        //   +0x04: Right vector   (cos(h), sin(h), 0.0)
        //   +0x14: Forward vector (-sin(h), cos(h), 0.0)
        //   +0x24: Up vector      (0.0, 0.0, 1.0)
        //   +0x34: Position       (spawn_x, spawn_y, final_z)
        let (sin_h, cos_h) = veh_heading.sin_cos();

        // Right vector
        *(veh.add(0x04) as *mut f32) = cos_h;
        *(veh.add(0x08) as *mut f32) = sin_h;
        *(veh.add(0x0c) as *mut f32) = 0.0;

        // Forward vector
        *(veh.add(0x14) as *mut f32) = -sin_h;
        *(veh.add(0x18) as *mut f32) = cos_h;
        *(veh.add(0x1c) as *mut f32) = 0.0;

        // Up vector
        *(veh.add(0x24) as *mut f32) = 0.0;
        *(veh.add(0x28) as *mut f32) = 0.0;
        *(veh.add(0x2c) as *mut f32) = 1.0;

        // Coordinates at placeable position (+0x34, +0x38, +0x3C)
        *(veh.add(0x34) as *mut f32) = spawn_x;
        *(veh.add(0x38) as *mut f32) = spawn_y;
        *(veh.add(0x3c) as *mut f32) = final_z;

        // 8. Configure status (active driver/vehicle status at CEntity + 0x53)
        let status_flags = veh.add(0x53) as *mut u8;
        *status_flags = (*status_flags & !0x38) | (4 << 3);

        // 9. Add vehicle entity to CWorld
        hook::slide_fn::<extern "C" fn(*mut u8)>(0x0003b090)(veh);

        // 10. Mark model as deletable so CStreaming cache can unload when appropriate
        hook::slide_fn::<extern "C" fn(u32)>(0x0011c210)(model_id);

        veh
    }
    #[cfg(target_pointer_width = "64")]
    {
        std::ptr::null_mut()
    }
}

/// Queues a vehicle to be spawned right in front of Claude.
pub fn queue_spawn_vehicle(model_id: u32) {
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
                        hook::slide_fn::<extern "C" fn(u32, u32)>(0x0011aef0)(mid, 1);
                    }
                }

                // 3b. Synchronously load all requested models
                hook::slide_fn::<extern "C" fn(u8)>(0x0011d954)(0);

                // 3c. Give weapons to Claude via CPed::GiveWeapon
                for &(wid, ammo) in &weapons {
                    hook::slide_fn::<extern "C" fn(*mut u8, u32, u32, u32) -> u32>(0x000de170)(
                        ped, wid, ammo, 1,
                    );
                }

                // 3d. Mark models as deletable so CStreaming cache is managed normally
                for &(wid, _) in &weapons {
                    for &mid in get_weapon_models(wid) {
                        hook::slide_fn::<extern "C" fn(u32)>(0x0011c210)(mid);
                    }
                }
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

            // Infinite Ammo: keep ammo and clip full across all 12 weapon slots (ped + 0x360 + slot * 0x18)
            if INFINITE_AMMO.load(Ordering::Relaxed) {
                unsafe {
                    for slot in 0..12 {
                        let wep_ptr = ped.add(0x360 + slot * 0x18);
                        let wep_type = *(wep_ptr as *const u32);
                        if wep_type > 0 {
                            let clip_ptr = wep_ptr.add(0x08) as *mut u32;
                            let total_ptr = wep_ptr.add(0x0c) as *mut u32;
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
