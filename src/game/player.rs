//! Helper module for interacting with the player ped (Claude), stats, money, weapons, and vehicle spawning.
//! GTA III v1.3.2 (armv7 / 32-bit) iOS on iPhone 5, iOS 10.x.

use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};
use std::sync::Mutex;
use crate::hook;
use lazy_static::lazy_static;

pub static INFINITE_HEALTH: AtomicBool = AtomicBool::new(false);
pub static INFINITE_AMMO: AtomicBool = AtomicBool::new(false);
pub static INFINITE_SPRINT: AtomicBool = AtomicBool::new(false);
pub static FAST_RELOAD: AtomicBool = AtomicBool::new(false);
pub static NEVER_WANTED: AtomicBool = AtomicBool::new(false);
pub static GOD_MODE_VEHICLE: AtomicBool = AtomicBool::new(false);
static LAST_GOD_MODE_VEH: AtomicUsize = AtomicUsize::new(0);
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
    ClearWorldGarbage,
    AdvanceTimeHours(u8),
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
/// Sequence mirrors GTA III's native CREATE_CAR opcode 00A5 and CCheat::VehicleCheat:
///   1. CStreaming::RequestModel (0x0011AEF0, flag 1 = GAME_REQUIRED)
///   2. CStreaming::LoadAllRequestedModels (0x0011D954)
///   3. operator new (0x001388DC) - 0x5AC bytes for CAutomobile, 0x488 bytes for CBoat
///   4. CAutomobile::CAutomobile (0x00068E1C) or CBoat::CBoat (0x0009C23C)
///   5. CMatrix::RotateZ on matrix at veh + 0x04 (0x0005AAB0)
///   6. Write translation vector directly into embedded CMatrix (veh + 0x34..0x3C)
///   7. CMatrix::UpdateRW (0x0005ABE0) & CEntity::UpdateRwFrame (0x0002CDD4) to sync RenderWare hierarchy
///   8. CCarCtrl::ClearAreaAroundVehicle (0x000C5064)
///   9. Set door lock status to CARLOCK_UNLOCKED (1) at veh + 0x228 (opcode 020A)
///  10. Set m_nCreatedBy to RANDOM_VEHICLE (1) at veh + 0x1F8 (opcode 01C8 / Car.RemoveReferences)
///  11. CWorld::Add(veh) (0x0003B090)
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
            hook::slide_fn::<extern "C" fn(u32)>(0x0011c064)(0);
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

        // Determine vehicle class: Boats in GTA III (verified from default.ide in IPA):
        // 120 (predator), 142 (speeder), 143 (reefer), 150 (ghost).
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

        // Query ground Z using CWorld::FindGroundZForCoord (0x00038B48)
        let gz_bits = hook::slide_fn::<extern "C" fn(u32, u32) -> u32>(0x00038b48)(
            spawn_x.to_bits(),
            spawn_y.to_bits(),
        );
        let ground_z = f32::from_bits(gz_bits);
        // If ground_z is valid and near player, use it; otherwise fall back to player Z
        let base_z = if (ground_z - pz).abs() < 5.0 && !ground_z.is_nan() {
            ground_z
        } else {
            pz
        };

        // Heading angle in radians: in GTA coordinate system (forward.x = -sin(heading), forward.y = cos(heading))
        let heading = (-dir_x).atan2(dir_y);
        // Rotate vehicle 90 degrees sideways so Claude stands directly in front of the driver seat (left door)
        // Matches Vice City implementation
        let veh_heading = heading + std::f32::consts::FRAC_PI_2;

        if is_boat {
            // Allocate CBoat (0x488 bytes)
            // Verified from native CREATE_CAR opcode 00A5 (0x000454C0)
            let veh = hook::slide_fn::<extern "C" fn(usize) -> *mut u8>(0x001388dc)(0x488);
            if veh.is_null() {
                return std::ptr::null_mut();
            }

            // Construct CBoat(veh, model_id, 2) - MISSION_VEHICLE
            // Native CBoat constructor at 0x0009C23C (matches 0x000454D6)
            hook::slide_fn::<extern "C" fn(*mut u8, u32, u8) -> *mut u8>(0x0009c23c)(veh, model_id, 2);

            // Set boat orientation using native CMatrix::RotateZ (0x0005AAB0)
            hook::slide_fn::<extern "C" fn(*mut u8, u32)>(0x0005aab0)(
                veh.add(0x04),
                veh_heading.to_bits(),
            );

            let spawn_z = pz + 0.5f32;

            // Set spawn coordinates into embedded CMatrix translation vector (veh + 0x34..0x3C)
            *(veh.add(0x34) as *mut f32) = spawn_x;
            *(veh.add(0x38) as *mut f32) = spawn_y;
            *(veh.add(0x3c) as *mut f32) = spawn_z;

            // Synchronize RenderWare matrix & clump objects (matches native SET_CAR_Z_ANGLE 0x000487EE..0x000487FA)
            hook::slide_fn::<extern "C" fn(*mut u8)>(0x0005abe0)(veh.add(0x04)); // CMatrix::UpdateRW
            hook::slide_fn::<extern "C" fn(*mut u8)>(0x0002cdd4)(veh);           // CEntity::UpdateRwFrame

            // Clear area around vehicle using native CCarCtrl::ClearAreaAroundVehicle (0x000C5064)
            let target_pos = [spawn_x, spawn_y, spawn_z];
            hook::slide_fn::<extern "C" fn(*const f32, *mut u8)>(0x000c5064)(target_pos.as_ptr(), veh);

            // Zero linear & angular momentum (0x000B31F0)
            hook::slide_fn::<extern "C" fn(*mut u8)>(0x000b31f0)(veh);

            // Set status to STATUS_ABANDONED (4): (entity_type & 0x07) | (4 << 3)
            let status_byte = veh.add(0x53) as *mut u8;
            *status_byte = (*status_byte & 0x07) | (4 << 3);

            // Door lock status: CARLOCK_UNLOCKED (1) (matches opcode 020A at 0x00094496)
            *(veh.add(0x228) as *mut u32) = 1;

            // Mark as regular/traffic vehicle so CLEO/game can recycle it when player leaves
            // Matches opcode 01C8 (MARK_CAR_AS_NO_LONGER_NEEDED) / Car.RemoveReferences:
            *(veh.add(0x1f8) as *mut u8) = 1;

            // Set bIsCheatedCar = 1 (veh + 0x1F9, bit 3)
            *(veh.add(0x1f9) as *mut u8) |= 8;

            // Boat physics limits (matches native CREATE_BOAT in opcode 00A5 at 0x00045696..0x000456DE)
            *(veh.add(0x15e) as *mut u8) = 0;
            *(veh.add(0x15f) as *mut u8) = 0;
            *(veh.add(0x15d) as *mut u8) = 0;
            *(veh.add(0x164) as *mut f32) = 9.0f32;
            *(veh.add(0x168) as *mut u8) = 9;
            *(veh.add(0x15b) as *mut u8) = 0;
            *(veh.add(0x15c) as *mut u8) = 0;
            *(veh.add(0x1f9) as *mut u8) &= 0xef;
            *(veh.add(0x1fb) as *mut u8) |= 4; // bIsBoat flag

            // Add vehicle entity to CWorld
            hook::slide_fn::<extern "C" fn(*mut u8)>(0x0003b090)(veh);
            log::info!("spawn_vehicle_direct: Boat {} spawned at ({:.2}, {:.2}, {:.2})", model_id, spawn_x, spawn_y, spawn_z);
            veh
        } else {
            // Allocate CAutomobile (0x5AC bytes)
            // Verified from native CREATE_CAR opcode 00A5 (0x000455E4) and CCheat::VehicleCheat (0x000C0AC8)
            let veh = hook::slide_fn::<extern "C" fn(usize) -> *mut u8>(0x001388dc)(0x5ac);
            if veh.is_null() {
                return std::ptr::null_mut();
            }

            // Construct CAutomobile(veh, model_id, 2) - MISSION_VEHICLE
            // Native CAutomobile constructor at 0x00068E1C (matches 0x000455FA and 0x000C0AE0)
            hook::slide_fn::<extern "C" fn(*mut u8, u32, u8) -> *mut u8>(0x00068e1c)(veh, model_id, 2);

            // Set vehicle orientation sideways so Claude stands directly in front of the driver seat
            hook::slide_fn::<extern "C" fn(*mut u8, u32)>(0x0005aab0)(
                veh.add(0x04),
                veh_heading.to_bits(),
            );

            // Calculate height above ground using model collision bounds (0x0002C970) safely
            let p_model_info_ptr = hook::slide::<*const *const u8>(0x003d9c10);
            let mut valid_height = 0.85f32;
            if !p_model_info_ptr.is_null() && model_id < 2000 {
                let model_info = *p_model_info_ptr.add(model_id as usize);
                if !model_info.is_null() {
                    let col_model = *(model_info.add(0x1c) as *const *const u8);
                    if !col_model.is_null() {
                        let h_bits = hook::slide_fn::<extern "C" fn(*mut u8) -> u32>(0x0002c970)(veh);
                        let h = f32::from_bits(h_bits);
                        if !h.is_nan() && h > 0.0 && h < 5.0 {
                            valid_height = h;
                        }
                    }
                }
            }
            // Drop car gently (+0.6m above ground contact) so tires hit the road cleanly
            // and suspension compresses naturally without polygon intersection glitches.
            let spawn_z = base_z + valid_height + 0.6f32;

            // Set spawn coordinates into the embedded CMatrix translation vector (veh + 0x34..0x3C)
            *(veh.add(0x34) as *mut f32) = spawn_x;
            *(veh.add(0x38) as *mut f32) = spawn_y;
            *(veh.add(0x3c) as *mut f32) = spawn_z;

            // Synchronize RenderWare matrix & clump objects (matches native SET_CAR_Z_ANGLE 0x000487EE..0x000487FA)
            // This is CRITICAL: without these, RenderWare keeps the old/identity matrix,
            // resulting in stale frustum clip bounds (invisible rectangle plane bug)
            // and dummy frames at (0,0,0) which prevented normal car door entry animation.
            hook::slide_fn::<extern "C" fn(*mut u8)>(0x0005abe0)(veh.add(0x04)); // CMatrix::UpdateRW
            hook::slide_fn::<extern "C" fn(*mut u8)>(0x0002cdd4)(veh);           // CEntity::UpdateRwFrame

            // Clear area around vehicle using native CCarCtrl::ClearAreaAroundVehicle (0x000C5064)
            // Matches native 0x0004554C and VehicleCheat 0x000C0A60.
            let target_pos = [spawn_x, spawn_y, spawn_z];
            hook::slide_fn::<extern "C" fn(*const f32, *mut u8)>(0x000c5064)(target_pos.as_ptr(), veh);

            // Zero linear & angular momentum (0x000B31F0 - matches native CREATE_CAR)
            hook::slide_fn::<extern "C" fn(*mut u8)>(0x000b31f0)(veh);

            // Set status to STATUS_ABANDONED (4): (entity_type & 0x07) | (4 << 3).
            // Matches native 0x00045558 & 0x000C0A6E: bfi r1, r2, #3, #0x1d (r2 = 4).
            let status_byte = veh.add(0x53) as *mut u8;
            *status_byte = (*status_byte & 0x07) | (4 << 3);

            // Door lock status: CARLOCK_UNLOCKED (1)
            // Matches opcode 020A (SET_CAR_DOOR_STATUS to 1, see 0x00094496) and CarSpawner.txt line 423
            *(veh.add(0x228) as *mut u32) = 1;

            // Mark as regular/traffic vehicle so CLEO/game can recycle it when player leaves
            // Matches opcode 01C8 (MARK_CAR_AS_NO_LONGER_NEEDED) / Car.RemoveReferences:
            *(veh.add(0x1f8) as *mut u8) = 1;

            // Mark as cheated car: veh + 0x1F9 |= 8 (matches native 0x00045566 & 0x000C0A7E).
            *(veh.add(0x1f9) as *mut u8) |= 8;

            // Native CREATE_CAR suspension & physics limits (matches native 0x00045570..0x0004558C
            // and VehicleCheat 0x000C0A8A..0x000C0AAC):
            *(veh.add(0x15e) as *mut u8) = 0;
            *(veh.add(0x15f) as *mut u8) = 0;
            *(veh.add(0x164) as *mut f32) = 20.0f32; // Native max suspension spring length: 0x41A00000
            *(veh.add(0x168) as *mut u8) = 20;       // Native spring iterations: 0x14 = 20

            // Add vehicle entity to CWorld (0x0003B090)
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

/// Fully repairs a vehicle in GTA III ARMv7 matching native Pay N Spray behavior:
/// - Restores health to 1000.0f (veh + 0x204)
/// - Clears damage / fire timer (veh + 0x534)
/// - Extinguishes any active fire on vehicle (veh + 0x1E8 points to CFire)
/// - For automobiles (veh + 0x288 == 0):
///   Calls native CAutomobile::Fix (0x0005CE78) which:
///     * Resets CDamageManager at veh + 0x28C (0x000CC700) (engine status, panels, doors, lights, tyre damage)
///     * Clears bIsDamaged smoke flag (veh + 0x1FB bit 1)
///     * Swaps all damaged atomics (bonnet/cap, bumpers, boot, doors) to undamaged intact atomics via SetComponentAtomicFlagsCB
///     * Resets transformation matrices of all components (nodes 7..20) back to straight/closed
///   Flips overturned vehicle upright if up.z < 0 (matches Pay N Spray 0x000F2482..0x000F2528)
/// - For boats or other vehicles: resets CDamageManager and clears bIsDamaged
/// - Restores tyre flags (veh + 0x1FD &= !1)
/// Does NOT change vehicle color (preserving original primary & secondary colors).
pub unsafe fn repair_vehicle(veh: *mut u8) {
    if veh.is_null() {
        return;
    }

    // 1. Restore vehicle health to 1000.0f
    *(veh.add(0x204) as *mut f32) = 1000.0;

    // 2. Clear damage / fire timer (matches Pay N Spray at 0x000F245E)
    *(veh.add(0x534) as *mut u32) = 0;

    // 3. Extinguish any active fire on vehicle (veh + 0x1E8 points to CFire)
    let p_fire = *(veh.add(0x1e8) as *const *mut u8);
    if !p_fire.is_null() {
        hook::slide_fn::<extern "C" fn(*mut u8)>(0x00068ef0)(p_fire); // CFire::Extinguish
        *(veh.add(0x1e8) as *mut usize) = 0;
    }

    // 4. Vehicle type at offset 0x288 (0 = Automobile, 1 = Boat, etc.)
    // Note: veh + 0x1F8 is m_nVehicleCreatedBy (1 for random, 2 for mission), NOT vehicle type!
    let veh_type = *(veh.add(0x288) as *const i32);
    if veh_type == 0 {
        // Native CAutomobile::Fix (0x0005CE78) restores all damaged geometries and resets damage
        hook::slide_fn::<extern "C" fn(*mut u8)>(0x0005ce78)(veh);

        // If the car is upside-down / overturned (up.z < 0.0), flip upright
        // exactly like native Pay N Spray does (0x000F2482..0x000F2528)
        let up_z = *(veh.add(0x2c) as *const f32);
        if up_z < 0.0 {
            *(veh.add(0x24) as *mut f32) = -*(veh.add(0x24) as *const f32);
            *(veh.add(0x28) as *mut f32) = -*(veh.add(0x28) as *const f32);
            *(veh.add(0x2c) as *mut f32) = -*(veh.add(0x2c) as *const f32);
            *(veh.add(0x04) as *mut f32) = -*(veh.add(0x04) as *const f32);
            *(veh.add(0x08) as *mut f32) = -*(veh.add(0x08) as *const f32);
            *(veh.add(0x0c) as *mut f32) = -*(veh.add(0x0c) as *const f32);
            hook::slide_fn::<extern "C" fn(*mut u8)>(0x0005abe0)(veh.add(0x04)); // CMatrix::UpdateRW
            hook::slide_fn::<extern "C" fn(*mut u8)>(0x0002cdd4)(veh);           // CEntity::UpdateRwFrame
        }
    } else {
        // Reset CDamageManager directly
        hook::slide_fn::<extern "C" fn(*mut u8)>(0x000cc700)(veh.add(0x28c));
        // Clear bIsDamaged (smoke flag)
        *(veh.add(0x1fb) as *mut u8) &= !2;
    }

    // 5. Restore tyres
    *(veh.add(0x1fd) as *mut u8) &= !1;
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
                                repair_vehicle(veh);
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
                                repair_vehicle(veh);
                            }
                        }
                    }
                    PlayerAction::ClearWorldGarbage => {
                        unsafe {
                            let current_veh = find_player_vehicle();
                            let ped = find_player_ped();
                            let mut cleared_vehicles = 0;
                            let mut cleared_objects = 0;

                            // 1. Iterate ms_pVehiclePool (0x001BA5D0 -> 0x0049F4DC)
                            //    Safely removes and destroys all stray/spawned vehicles on the entire map,
                            //    EXCEPT the vehicle Claude is currently driving.
                            let veh_pool_pptr = hook::slide::<*const *const u8>(0x001ba5d0);
                            if !veh_pool_pptr.is_null() && !(*veh_pool_pptr).is_null() {
                                let pool = *veh_pool_pptr;
                                let m_objects = *(pool as *const *mut u8);
                                let m_byte_map = *(pool.add(4) as *const *const i8);
                                let m_size = *(pool.add(8) as *const i32);

                                if !m_objects.is_null() && !m_byte_map.is_null() && m_size > 0 {
                                    for i in 0..m_size {
                                        if *m_byte_map.add(i as usize) >= 0 {
                                            let veh = m_objects.add((i as usize) * 0x5ac);
                                            if !veh.is_null() && veh != current_veh && veh != ped {
                                                hook::slide_fn::<extern "C" fn(*mut u8)>(0x0003a8d4)(veh);
                                                cleared_vehicles += 1;
                                            }
                                        }
                                    }
                                }
                            }

                            // 2. Iterate ms_pObjectPool (0x001BA85C -> 0x0049F4E0)
                            //    Safely removes temporary / spawned objects (type == 2 or 0).
                            let obj_pool_pptr = hook::slide::<*const *const u8>(0x001ba85c);
                            if !obj_pool_pptr.is_null() && !(*obj_pool_pptr).is_null() {
                                let pool = *obj_pool_pptr;
                                let m_objects = *(pool as *const *mut u8);
                                let m_byte_map = *(pool.add(4) as *const *const i8);
                                let m_size = *(pool.add(8) as *const i32);

                                if !m_objects.is_null() && !m_byte_map.is_null() && m_size > 0 {
                                    for i in 0..m_size {
                                        if *m_byte_map.add(i as usize) >= 0 {
                                            let obj = m_objects.add((i as usize) * 0x1b4);
                                            if !obj.is_null() && obj != ped && obj != current_veh {
                                                let obj_type = *obj.add(0x178);
                                                if obj_type == 2 || obj_type == 0 {
                                                    hook::slide_fn::<extern "C" fn(*mut u8)>(0x0003a8d4)(obj);
                                                    cleared_objects += 1;
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // 3. Clear dead peds, burning wrecks, and lingering debris around Claude
                            if !ped.is_null() {
                                let px = *(ped.add(0x34) as *const f32);
                                let py = *(ped.add(0x38) as *const f32);
                                let pz = *(ped.add(0x3c) as *const f32);
                                let pos = [px, py, pz];
                                hook::slide_fn::<extern "C" fn(*const f32, f32, u8)>(0x0003a700)(pos.as_ptr(), 500.0f32, 1);
                            }

                            log::info!(
                                "ClearWorldGarbage: Cleared {} vehicles and {} objects from map",
                                cleared_vehicles, cleared_objects
                            );
                        }
                    }
                    PlayerAction::AdvanceTimeHours(hours_to_add) => {
                        unsafe {
                            // GTA III ARMv7 CClock addresses:
                            // ms_nGameClockHours (u8): 0x002483B8
                            // ms_nGameClockMinutes (u8): 0x002483BC
                            // CClock::SetGameClock(hours, minutes): 0x000103CC
                            let hours_ptr = hook::slide::<*const u8>(0x002483b8);
                            let mins_ptr = hook::slide::<*const u8>(0x002483bc);
                            if !hours_ptr.is_null() && !mins_ptr.is_null() {
                                let cur_h = *hours_ptr;
                                let cur_m = *mins_ptr;
                                let new_h = (cur_h + hours_to_add) % 24;
                                hook::slide_fn::<extern "C" fn(u8, u8)>(0x000103cc)(new_h, cur_m);
                                log::info!(
                                    "AdvanceTime: Clock moved forward +{} hours (from {:02}:{:02} to {:02}:{:02})",
                                    hours_to_add, cur_h, cur_m, new_h, cur_m
                                );
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

            // Vehicle God Mode / Mobil Kebal: makes Claude's vehicle 100% invincible
            let current_veh = find_player_vehicle();
            if !current_veh.is_null() {
                if GOD_MODE_VEHICLE.load(Ordering::Relaxed) {
                    unsafe {
                        // If this is a newly entered vehicle or cheat was just turned on,
                        // fix all clump geometries once to restore any previously damaged parts cleanly.
                        let veh_addr = current_veh as usize;
                        if LAST_GOD_MODE_VEH.swap(veh_addr, Ordering::Relaxed) != veh_addr {
                            repair_vehicle(current_veh);
                        }

                        // Extinguish any fire if vehicle caught on fire
                        let p_fire = *(current_veh.add(0x1e8) as *const *mut u8);
                        if !p_fire.is_null() {
                            hook::slide_fn::<extern "C" fn(*mut u8)>(0x00068ef0)(p_fire);
                            *(current_veh.add(0x1e8) as *mut usize) = 0;
                        }

                        // 1. Lock vehicle health to 1000.0f
                        *(current_veh.add(0x204) as *mut f32) = 1000.0;

                        // 2. CRITICAL: Clear byte 0x52!
                        //    In GTA III iOS, non-zero at 0x52 causes CEntity::UpdateRwFrame (0x0002CD20)
                        //    to abort and skip rendering the clump, making the vehicle body completely
                        //    invisible ("wheels only" / transparent bug).
                        *(current_veh.add(0x52) as *mut u8) = 0;

                        // 3. Flags byte 2 (+0x55):
                        //    bit 1 (0x02): bExplosionProof (immune to explosions)
                        //    bit 2 (0x04): bIsVisible (guarantees vehicle is visible)
                        //    Clear bit 0 (bWasPostponed) and bit 4 (bRenderScorched)
                        let flags_55 = current_veh.add(0x55) as *mut u8;
                        *flags_55 = (*flags_55 | 0x02 | 0x04) & !0x01 & !0x10;

                        // 4. Flags byte 3 (+0x56):
                        //    bit 0 (0x01): bBulletProof
                        //    bit 1 (0x02): bFireProof
                        //    bit 2 (0x04): bCollisionProof (prevents visual deformation & crash damage)
                        //    bit 3 (0x08): bMeleeProof
                        let flags_56 = current_veh.add(0x56) as *mut u8;
                        *flags_56 |= 0x0f;

                        // 5. Flags byte 4 (+0x57):
                        //    Clear bit 7 (bDoNotRender)
                        let flags_57 = current_veh.add(0x57) as *mut u8;
                        *flags_57 &= !0x80;

                        // 6. Puncture-proof tyres (bTyresDontBurst is bit 1 of byte +0x1fd)
                        *(current_veh.add(0x1fd) as *mut u8) |= 0x02;

                        // 7. Reset CDamageManager status (0x000CC700) each frame
                        hook::slide_fn::<extern "C" fn(*mut u8)>(0x000cc700)(current_veh.add(0x28c));

                        // 8. Clear bIsDamaged (bit 1 of byte +0x1fb) to prevent smoke/fire
                        *(current_veh.add(0x1fb) as *mut u8) &= !2;
                    }
                } else {
                    let prev = LAST_GOD_MODE_VEH.swap(0, Ordering::Relaxed);
                    if prev != 0 && prev == current_veh as usize {
                        unsafe {
                            // Restore normal vehicle destructibility when cheat is disabled
                            let flags_55 = current_veh.add(0x55) as *mut u8;
                            *flags_55 &= !0x02; // clear bExplosionProof
                            let flags_56 = current_veh.add(0x56) as *mut u8;
                            *flags_56 &= !0x0f; // clear bullet, fire, collision, melee proofs
                            *(current_veh.add(0x1fd) as *mut u8) &= !0x02; // allow tyre bursting
                        }
                    }
                }
            } else {
                LAST_GOD_MODE_VEH.store(0, Ordering::Relaxed);
            }

            // Infinite Ammo: lock reserve ammo to 9999 and keep magazine clip fully loaded
            // for firearms & throwables (slots 2..=11).
            // Skip slot 0 (unarmed) and slot 1 (baseball bat has no ammo).
            if INFINITE_AMMO.load(Ordering::Relaxed) {
                unsafe {
                    for slot in 2..=11 {
                        let wep_ptr = ped.add(0x360 + slot * 0x18);
                        let wep_type = *(wep_ptr as *const u32);
                        if wep_type >= 2 && wep_type <= 11 {
                            let clip_ptr = wep_ptr.add(0x08) as *mut u32;
                            let total_ptr = wep_ptr.add(0x0c) as *mut u32;
                            let state_ptr = wep_ptr.add(0x04) as *mut u32;

                            // 1. Determine clip capacity from CWeaponInfo (0x0002c5bc)
                            let winfo = hook::slide_fn::<extern "C" fn(u32) -> *mut u8>(0x0002c5bc)(wep_type);
                            let clip_cap = if !winfo.is_null() {
                                *(winfo.add(0x10) as *const u32)
                            } else {
                                match wep_type {
                                    2 => 12,   // Colt45
                                    3 => 25,   // Uzi
                                    4 => 1000, // Shotgun
                                    5 => 30,   // AK47
                                    6 => 60,   // M16
                                    9 => 500,  // FlameThrower
                                    _ => 1,    // Sniper, RPG, Molotov, Grenade
                                }
                            };

                            // 2. Lock clip ammo to maximum capacity so magazine never depletes:
                            //    Keeping *clip_ptr locked to clip_cap means the player never runs out of
                            //    bullets in the magazine, never has to reload, and the HUD counter never drops.
                            if clip_cap > 0 && *clip_ptr < clip_cap {
                                *clip_ptr = clip_cap;
                            }

                            // 3. Always keep total reserve ammo locked at 9999
                            if *total_ptr < 9999 {
                                *total_ptr = 9999;
                            }

                            // 4. If weapon was marked out of ammo (state 3), restore to READY (0)
                            if *state_ptr == 3 {
                                *state_ptr = 0;
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
