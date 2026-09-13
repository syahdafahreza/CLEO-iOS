//! Helper module for interacting with the player ped, stats, money, weapons, and vehicle spawning.

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

/// Returns the pointer to Tommy Vercetti (`CPed*`).
pub fn find_player_ped() -> *mut u8 {
    #[cfg(target_pointer_width = "32")]
    {
        hook::slide_fn::<extern "C" fn() -> *mut u8>(0x000ea448)()
    }
    #[cfg(target_pointer_width = "64")]
    {
        std::ptr::null_mut()
    }
}

/// Returns the pointer to the vehicle Tommy is currently in (`CVehicle*`), or null.
pub fn find_player_vehicle() -> *mut u8 {
    #[cfg(target_pointer_width = "32")]
    {
        hook::slide_fn::<extern "C" fn() -> *mut u8>(0x000ea394)()
    }
    #[cfg(target_pointer_width = "64")]
    {
        std::ptr::null_mut()
    }
}

/// Returns a pointer to the active `CPlayerInfo` struct.
pub fn get_player_info_ptr() -> *mut u8 {
    #[cfg(target_pointer_width = "32")]
    {
        let focus_ptr = hook::slide::<*const u8>(0x003870b8);
        let focus = if !focus_ptr.is_null() {
            unsafe { *focus_ptr as usize }
        } else {
            0
        };

        let players_base = hook::slide::<*mut u8>(0x00347800);
        if players_base.is_null() {
            std::ptr::null_mut()
        } else {
            // Each CPlayerInfo entry is 0x174 (372) bytes
            unsafe { players_base.add(focus * 0x174) }
        }
    }
    #[cfg(target_pointer_width = "64")]
    {
        std::ptr::null_mut()
    }
}

/// Gets Tommy's current money.
pub fn get_money() -> i32 {
    let info = get_player_info_ptr();
    if !info.is_null() {
        unsafe { *(info.add(0xa0) as *const i32) }
    } else {
        0
    }
}

/// Modifies Tommy's money (both internal bank and display HUD).
pub fn apply_money(amount: i32, is_delta: bool) {
    let info = get_player_info_ptr();
    if !info.is_null() {
        unsafe {
            let money_ptr = info.add(0xa0) as *mut i32;
            let display_money_ptr = info.add(0xa4) as *mut i32;
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

/// Spawns a vehicle directly in front of Tommy facing the same direction as Tommy,
/// matching the reliable GTA SA CLEO spawner behavior without relying on vanilla road path nodes.
pub fn spawn_vehicle_direct(model_id: u32) -> *mut u8 {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        let ped = find_player_ped();
        if ped.is_null() {
            return std::ptr::null_mut();
        }

        // 1. Request and load model synchronously
        hook::slide_fn::<extern "C" fn(u32, u32)>(0x00099414)(model_id, 1);
        hook::slide_fn::<extern "C" fn(u8)>(0x0009c55c)(0);

        // 2. Read player position and forward facing vector from CMatrix at ped + 0x04
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

        // Heading angle in radians: in GTA coordinate system (forward.x = -sin(heading), forward.y = cos(heading))
        let heading = (-dir_x).atan2(dir_y);
        // Rotate vehicle 90 degrees sideways so Tommy stands directly in front of the driver seat (left door)
        let veh_heading = heading + std::f32::consts::FRAC_PI_2;

        // 3. Determine vehicle class and allocation size
        let is_bike = matches!(model_id, 166 | 178 | 191 | 192 | 193 | 198)
            || hook::slide_fn::<extern "C" fn(u32) -> u32>(0x0019132c)(model_id) != 0;
        let is_boat = matches!(model_id, 136 | 153 | 176 | 182 | 183 | 184 | 200 | 201 | 212)
            || hook::slide_fn::<extern "C" fn(u32) -> u32>(0x001912d4)(model_id) != 0;

        let in_car = !find_player_vehicle().is_null();
        let dist = if in_car {
            8.5f32
        } else if model_id == 155 || model_id == 162 || model_id == 138 || model_id == 186 || model_id == 165 {
            // Large vehicles: Hunter heli (155), Rhino tank (162), Trashmaster (138), Coach (186), Bus (165)
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

        // 4. Query exact ground elevation at spawn coordinates via CWorld::FindGroundZForCoord
        let ground_z_raw = hook::slide_fn::<extern "C" fn(u32, u32) -> u32>(0x0004909c)(
            spawn_x.to_bits(),
            spawn_y.to_bits(),
        );
        let ground_z = f32::from_bits(ground_z_raw);
        let spawn_z = if ground_z > -50.0 && (ground_z - pz).abs() < 12.0 {
            ground_z
        } else {
            pz
        };

        // 5. Allocate vehicle memory using game operator new (0x0013f024)
        let (size, veh_type) = if is_boat {
            (0x4c0usize, 0u8) // CBoat
        } else if is_bike {
            (0x4ecusize, 1u8) // CBike (actual size is 0x4ec)
        } else {
            (0x5dcusize, 2u8) // CAutomobile
        };

        let veh = hook::slide_fn::<extern "C" fn(usize) -> *mut u8>(0x0013f024)(size);
        if veh.is_null() {
            return std::ptr::null_mut();
        }

        // 6. Invoke vehicle C++ constructor
        match veh_type {
            0 => {
                // CBoat::CBoat(ptr, model_id, 2)
                hook::slide_fn::<extern "C" fn(*mut u8, u32, u8) -> *mut u8>(0x0005b5f0)(veh, model_id, 2);
            }
            1 => {
                // CBike::CBike(ptr, model_id, 1) at 0x00152710
                hook::slide_fn::<extern "C" fn(*mut u8, u32, u8) -> *mut u8>(0x00152710)(veh, model_id, 1);
            }
            _ => {
                // CAutomobile::CAutomobile(ptr, model_id, 1) at 0x001d7620
                hook::slide_fn::<extern "C" fn(*mut u8, u32, u8) -> *mut u8>(0x001d7620)(veh, model_id, 1);
            }
        }

        // 7. Calculate suspension height above road and set position & orientation
        let height_raw = hook::slide_fn::<extern "C" fn(*mut u8) -> u32>(0x0015d0d4)(veh);
        let height = f32::from_bits(height_raw);
        let final_z = spawn_z + if height > 0.05 && height < 5.0 { height } else if is_bike { 0.25 } else { 0.4 };

        // Set vehicle rotation sideways so Tommy stands directly in front of the driver seat
        hook::slide_fn::<extern "C" fn(*mut u8, u32, u32, u32)>(0x00075798)(
            veh.add(4),
            0.0f32.to_bits(),
            0.0f32.to_bits(),
            veh_heading.to_bits(),
        );

        // Set vehicle coordinates in CPlaceable matrix at veh + 0x34
        *(veh.add(0x34) as *mut f32) = spawn_x;
        *(veh.add(0x38) as *mut f32) = spawn_y;
        *(veh.add(0x3c) as *mut f32) = final_z;

        // 8. Configure status and permissions so player can immediately enter
        let status_flags = veh.add(0x52) as *mut u8;
        *status_flags = (*status_flags & !0x38) | (4 << 3); // STATUS_PLAYER

        if veh_type == 2 {
            *(veh.add(0x230) as *mut u32) = 1; // Unlocked doors
        } else if veh_type == 0 {
            *(veh.add(0x15c) as *mut f32) = 20.0;
            *(veh.add(0x160) as *mut u8) = 20;
        }

        // 9. Add vehicle entity to CWorld (0x0004bc24)
        hook::slide_fn::<extern "C" fn(*mut u8)>(0x0004bc24)(veh);

        // 10. Mark model as deletable so it can be streamed out later
        hook::slide_fn::<extern "C" fn(u32)>(0x000998ec)(model_id);

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

/// Queues a weapon to be given to Tommy with ammo.
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
                        hook::slide_fn::<extern "C" fn(u32)>(0x00079254)(1);
                    }
                    PlayerAction::FullArmor => {
                        hook::slide_fn::<extern "C" fn()>(0x00077e34)();
                    }
                    PlayerAction::ClearWanted => {
                        hook::slide_fn::<extern "C" fn()>(0x00077e84)();
                    }
                    PlayerAction::RaiseWanted => {
                        hook::slide_fn::<extern "C" fn()>(0x00077edc)();
                    }
                    PlayerAction::RepairCurrentVehicle => {
                        let veh = find_player_vehicle();
                        if !veh.is_null() {
                            unsafe {
                                *(veh.add(0x204) as *mut f32) = 1000.0;
                                hook::slide_fn::<extern "C" fn(*mut u8)>(0x000c1cc0)(veh.add(0x2a0));

                                let model_id = *(veh.add(0x5c) as *const u16) as u32;
                                let is_bike = matches!(model_id, 166 | 178 | 191 | 192 | 193 | 198)
                                    || hook::slide_fn::<extern "C" fn(u32) -> u32>(0x0019132c)(model_id) != 0;
                                let is_boat = matches!(model_id, 136 | 153 | 176 | 182 | 183 | 184 | 200 | 201 | 212)
                                    || hook::slide_fn::<extern "C" fn(u32) -> u32>(0x001912d4)(model_id) != 0;

                                if !is_bike && !is_boat {
                                    hook::slide_fn::<extern "C" fn(*mut u8)>(0x001c6100)(veh);
                                } else {
                                    *(veh.add(0x1fb) as *mut u8) &= !2;
                                    *(veh.add(0x1fd) as *mut u8) &= !0x3f;
                                }
                            }
                        }
                    }
                }
            }
        }

        // 2. Process queued vehicle spawn (spawns directly in front of Tommy)
        if let Ok(mut q) = QUEUED_VEHICLE.lock() {
            if let Some(model_id) = q.take() {
                spawn_vehicle_direct(model_id);
            }
        }

        // 3. Process queued weapons
        if let Ok(mut q) = QUEUED_WEAPONS.lock() {
            let ped = find_player_ped();
            if !ped.is_null() {
                for (wid, ammo) in q.drain(..) {
                    hook::slide_fn::<extern "C" fn(*mut u8, u32, u32, u32) -> u32>(0x00126cd4)(
                        ped, wid, ammo, 1,
                    );
                }
            }
        }

        // 4. Handle persistent toggle states
        let ped = find_player_ped();
        if !ped.is_null() {
            // Fix blood bug: clear bIsBleeding (bit 2 of ped + 0x14f) and m_nBleeding timer (ped + 0x51f)
            unsafe {
                *(ped.add(0x14f) as *mut u8) &= !4;
                *(ped.add(0x51f) as *mut u8) = 0;
            }

            // Infinite Health / God Mode (locks Tommy to 250 HP and 250 Armor)
            if INFINITE_HEALTH.load(Ordering::Relaxed) {
                unsafe {
                    *(ped.add(0x34c) as *mut f32) = 250.0;
                    *(ped.add(0x350) as *mut f32) = 250.0;
                }

                // If inside a vehicle, keep vehicle fully repaired
                let veh = find_player_vehicle();
                if !veh.is_null() {
                    unsafe {
                        *(veh.add(0x204) as *mut f32) = 1000.0;
                    }
                }
            }

            // Vehicle God Mode / Mobil Kebal: when Tommy is in a vehicle, make it 100% indestructible
            let current_veh = find_player_vehicle();
            if !current_veh.is_null() {
                if GOD_MODE_VEHICLE.load(Ordering::Relaxed) {
                    unsafe {
                        // 1. Lock vehicle health to 1000.0f
                        *(current_veh.add(0x204) as *mut f32) = 1000.0;
                        // 2. Full immunities: bullet, fire, collision, melee, explosion (bits 16..19, 21 on entity + 0x52)
                        *(current_veh.add(0x52) as *mut u32) |= 0x002f0000;
                        // 3. Ban anti bocor / tyres don't burst (bit 1 on 0x1fd)
                        *(current_veh.add(0x1fd) as *mut u8) |= 2;
                        // 4. Engine status intact
                        *(current_veh.add(0x2a4) as *mut u8) = 0;
                    }
                }
            }

            // Infinite Ammo: safely keep ammo and clip full across all 10 weapon slots (ped + 0x400 + slot * 0x18)
            // without corrupting ped flags
            if INFINITE_AMMO.load(Ordering::Relaxed) {
                unsafe {
                    for slot in 0..10 {
                        let wep_ptr = ped.add(0x400 + slot * 0x18);
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

        // Never Wanted / Anti Polisi: lock MaximumWantedLevel (0x0026a6d8) to 0
        let max_wanted_ptr = hook::slide::<*mut i32>(0x0026a6d8);
        if NEVER_WANTED.load(Ordering::Relaxed) {
            unsafe {
                if !max_wanted_ptr.is_null() {
                    *max_wanted_ptr = 0;
                }
            }
        } else {
            unsafe {
                if !max_wanted_ptr.is_null() && *max_wanted_ptr == 0 {
                    *max_wanted_ptr = 6;
                }
            }
        }

        let info = get_player_info_ptr();
        if !info.is_null() {
            // If Never Wanted is active, clear any active wanted stars immediately
            if NEVER_WANTED.load(Ordering::Relaxed) {
                unsafe {
                    let wanted_lvl = *(info.add(0x20) as *const i32);
                    if wanted_lvl > 0 {
                        *(info.add(0x20) as *mut i32) = 0;
                        hook::slide_fn::<extern "C" fn(*mut u8, u32)>(0x001f51d8)(info, 0);
                    }
                }
            }

            if INFINITE_SPRINT.load(Ordering::Relaxed) {
                unsafe {
                    *(info.add(0x140) as *mut u8) = 1;
                }
            }
            if FAST_RELOAD.load(Ordering::Relaxed) {
                unsafe {
                    *(info.add(0x141) as *mut u8) = 1;
                }
            }
        }
    }
}
