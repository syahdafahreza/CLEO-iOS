//! Helper module for interacting with the player ped, stats, money, weapons, and vehicle spawning.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use crate::hook;
use lazy_static::lazy_static;

pub static INFINITE_HEALTH: AtomicBool = AtomicBool::new(false);
pub static INFINITE_AMMO: AtomicBool = AtomicBool::new(false);
pub static INFINITE_SPRINT: AtomicBool = AtomicBool::new(false);
pub static FAST_RELOAD: AtomicBool = AtomicBool::new(false);

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
                }
            }
        }

        // 2. Process queued vehicle spawn
        if let Ok(mut q) = QUEUED_VEHICLE.lock() {
            if let Some(model_id) = q.take() {
                hook::slide_fn::<extern "C" fn(u32)>(0x00078920)(model_id);
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

            // Infinite Ammo (sets bInfiniteAmmo bit on CPed->m_nPedFlags at offset 0x14c)
            if INFINITE_AMMO.load(Ordering::Relaxed) {
                unsafe {
                    *(ped.add(0x14c) as *mut u32) |= 0x04000000;
                }
            }
        }

        let info = get_player_info_ptr();
        if !info.is_null() {
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
