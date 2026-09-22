//! Native Weapon Arsenal tab for GTA III iOS.

use std::sync::atomic::Ordering;
use std::sync::Mutex;
use crate::{
    game::player,
    meta::{
        gui,
        language::{Message, MessageKey},
        menu::{RowData, RowDetail, TabData},
    },
};
use lazy_static::lazy_static;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WeaponCategory {
    All,
    Bundles,
    Melee,
    Pistols,
    Smg,
    Shotguns,
    Assault,
    Snipers,
    Heavy,
    Thrown,
}

impl WeaponCategory {
    pub fn name(&self) -> &'static str {
        match self {
            WeaponCategory::All => "SEMUA SENJATA",
            WeaponCategory::Bundles => "PAKET LENGKAP",
            WeaponCategory::Melee => "SENJATA JARAK DEKAT",
            WeaponCategory::Pistols => "PISTOL",
            WeaponCategory::Smg => "SUBMACHINE GUN",
            WeaponCategory::Shotguns => "SHOTGUN",
            WeaponCategory::Assault => "SENAPAN SERBU",
            WeaponCategory::Snipers => "SENAPAN RUNDUK",
            WeaponCategory::Heavy => "SENJATA BERAT",
            WeaponCategory::Thrown => "LEMPAR & GRANAT",
        }
    }

    pub fn next(&self) -> WeaponCategory {
        match self {
            WeaponCategory::All => WeaponCategory::Bundles,
            WeaponCategory::Bundles => WeaponCategory::Melee,
            WeaponCategory::Melee => WeaponCategory::Pistols,
            WeaponCategory::Pistols => WeaponCategory::Smg,
            WeaponCategory::Smg => WeaponCategory::Shotguns,
            WeaponCategory::Shotguns => WeaponCategory::Assault,
            WeaponCategory::Assault => WeaponCategory::Snipers,
            WeaponCategory::Snipers => WeaponCategory::Heavy,
            WeaponCategory::Heavy => WeaponCategory::Thrown,
            WeaponCategory::Thrown => WeaponCategory::All,
        }
    }
}

lazy_static! {
    static ref CURRENT_CATEGORY: Mutex<WeaponCategory> = Mutex::new(WeaponCategory::All);
}

#[derive(Clone, Copy)]
pub struct WeaponDef {
    pub weapon_id: u32,
    pub name: &'static str,
    pub category: WeaponCategory,
    pub description: &'static str,
}

pub static WEAPONS: &[WeaponDef] = &[
    // --- Melee (Slot 1) ---
    WeaponDef {
        weapon_id: 1,
        name: "Baseball Bat",
        category: WeaponCategory::Melee,
        description: "Tongkat bisbol kayu pemukul andalan Claude",
    },

    // --- Pistols (Slot 2) ---
    WeaponDef {
        weapon_id: 2,
        name: "Colt .45",
        category: WeaponCategory::Pistols,
        description: "Pistol semi-otomatis standar LCPD dan mafia",
    },

    // --- SMG (Slot 3) ---
    WeaponDef {
        weapon_id: 3,
        name: "Micro Uzi (Ingram)",
        category: WeaponCategory::Smg,
        description: "Submachine gun kompak laju tembak sangat cepat",
    },

    // --- Shotguns (Slot 4) ---
    WeaponDef {
        weapon_id: 4,
        name: "Shotgun",
        category: WeaponCategory::Shotguns,
        description: "Shotgun pompa 12-gauge klasik daya rusak tinggi jarak dekat",
    },

    // --- Assault Rifles (Slot 5) ---
    WeaponDef {
        weapon_id: 5,
        name: "AK-47",
        category: WeaponCategory::Assault,
        description: "Senapan serbu militer tangguh buatan Soviet",
    },
    WeaponDef {
        weapon_id: 6,
        name: "M16",
        category: WeaponCategory::Assault,
        description: "Senapan serbu militer presisi tinggi laju tembak tinggi",
    },

    // --- Snipers (Slot 6) ---
    WeaponDef {
        weapon_id: 7,
        name: "Sniper Rifle",
        category: WeaponCategory::Snipers,
        description: "Senapan runduk jarak jauh dengan teropong bidik presisi",
    },

    // --- Heavy (Slot 7) ---
    WeaponDef {
        weapon_id: 8,
        name: "Rocket Launcher (RPG)",
        category: WeaponCategory::Heavy,
        description: "Peluncur roket penghancur helikopter polisi dan tank",
    },
    WeaponDef {
        weapon_id: 9,
        name: "FlameThrower",
        category: WeaponCategory::Heavy,
        description: "Penyembur api pembakar massa pejalan kaki dan kendaraan",
    },

    // --- Thrown (Slot 8) ---
    WeaponDef {
        weapon_id: 10,
        name: "Molotov Cocktail",
        category: WeaponCategory::Thrown,
        description: "Bom botol bensin pembakar area dan pengendara mobil",
    },
    WeaponDef {
        weapon_id: 11,
        name: "Grenade",
        category: WeaponCategory::Thrown,
        description: "Granat tangan berdaya ledak dahsyat pemusnah kerumunan",
    },
];

struct CategoryFilterRow;

impl RowData for CategoryFilterRow {
    fn title(&self) -> Message {
        let cat = *CURRENT_CATEGORY.lock().unwrap();
        Message::custom(format!("KATEGORI: {}", cat.name()))
    }

    fn detail(&self) -> RowDetail {
        RowDetail::Info(Message::custom("Sentuh untuk ganti kategori senjata"))
    }

    fn value(&self) -> Message {
        Message::custom("PILIH")
    }

    fn tint(&self) -> Option<(u8, u8, u8)> {
        Some(gui::colours::BLUE)
    }

    fn handle_tap(&mut self) -> bool {
        let mut cat = CURRENT_CATEGORY.lock().unwrap();
        *cat = cat.next();
        true
    }
}

struct WeaponBundleRow {
    bundle_id: u8,
    name: &'static str,
    detail: &'static str,
    activated: bool,
}

impl RowData for WeaponBundleRow {
    fn title(&self) -> Message {
        Message::custom(self.name)
    }

    fn detail(&self) -> RowDetail {
        RowDetail::Info(Message::custom(self.detail))
    }

    fn value(&self) -> Message {
        if self.activated {
            MessageKey::CheatActionOk.to_message()
        } else {
            Message::custom("AMBIL")
        }
    }

    fn tint(&self) -> Option<(u8, u8, u8)> {
        if self.activated {
            Some(gui::colours::GREEN)
        } else {
            None
        }
    }

    fn handle_tap(&mut self) -> bool {
        // Automatically enable infinite ammo when taking weapons
        player::INFINITE_AMMO.store(true, Ordering::Relaxed);

        if self.bundle_id == 3 {
            // Bundle 3: Native GTA III Weapons Cheat (gives all 11 weapons natively & safely)
            #[cfg(target_pointer_width = "32")]
            crate::hook::slide_fn::<extern "C" fn()>(0x000c0c70)();
            // Top up ammo to 9999 for firearm & throwable weapons (skip Bat: wid=1)
            for wid in 2..=11 {
                player::queue_give_weapon(wid, 9999);
            }
        } else {
            let weapons: &[u32] = match self.bundle_id {
                // Set 1 (Street / Standard)
                1 => &[1, 2, 3, 4, 11],
                // Set 2 (Tactical / SWAT)
                _ => &[1, 2, 3, 4, 5, 6, 7, 10],
            };

            for &wid in weapons {
                let ammo = if wid == 1 { 0 } else { 9999 };
                player::queue_give_weapon(wid, ammo);
            }
        }

        self.activated = true;
        player::show_cheat_toast(true);
        true
    }
}

struct WeaponRow {
    def: WeaponDef,
    activated: bool,
}

impl RowData for WeaponRow {
    fn title(&self) -> Message {
        Message::custom(self.def.name)
    }

    fn detail(&self) -> RowDetail {
        RowDetail::Info(Message::custom(format!("9.999 Peluru • {}", self.def.description)))
    }

    fn value(&self) -> Message {
        if self.activated {
            MessageKey::CheatActionOk.to_message()
        } else {
            Message::custom("AMBIL")
        }
    }

    fn tint(&self) -> Option<(u8, u8, u8)> {
        if self.activated {
            Some(gui::colours::GREEN)
        } else {
            None
        }
    }

    fn handle_tap(&mut self) -> bool {
        // Automatically ensure infinite ammo is also active
        player::INFINITE_AMMO.store(true, Ordering::Relaxed);
        let ammo = if self.def.weapon_id == 1 { 0 } else { 9999 };
        player::queue_give_weapon(self.def.weapon_id, ammo);
        player::show_cheat_toast(true);
        self.activated = true;
        true
    }
}

pub fn tab_data() -> TabData {
    let current_cat = *CURRENT_CATEGORY.lock().unwrap();
    let mut rows: Vec<Box<dyn RowData>> = vec![];

    // Filter selector row at the top
    rows.push(Box::new(CategoryFilterRow));

    // Weapon Bundles (displayed in 'All' and 'Bundles')
    if current_cat == WeaponCategory::All || current_cat == WeaponCategory::Bundles {
        rows.push(Box::new(WeaponBundleRow {
            bundle_id: 1,
            name: "PAKET 1: STANDARD / STREET",
            detail: "Baseball Bat, Colt .45, Micro Uzi, Shotgun, Granat",
            activated: false,
        }));
        rows.push(Box::new(WeaponBundleRow {
            bundle_id: 2,
            name: "PAKET 2: TACTICAL / SWAT",
            detail: "Bat, Colt .45, Uzi, Shotgun, AK-47, M16, Sniper, Molotov",
            activated: false,
        }));
        rows.push(Box::new(WeaponBundleRow {
            bundle_id: 3,
            name: "PAKET 3: ALL WEAPONS / MAYHEM",
            detail: "Semua 11 Senjata (Termasuk RPG & Flamethrower)",
            activated: false,
        }));
    }

    // Individual weapons
    for def in WEAPONS {
        if current_cat == WeaponCategory::All || def.category == current_cat {
            rows.push(Box::new(WeaponRow {
                def: *def,
                activated: false,
            }));
        }
    }

    TabData {
        name: MessageKey::WeaponsTabTitle.to_message(),
        warning: None,
        row_data: rows,
    }
}
