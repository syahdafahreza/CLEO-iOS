//! Native Weapon Arsenal tab for GTA Vice City.

use std::sync::atomic::Ordering;
use std::sync::Mutex;
use crate::{
    game::player,
    meta::{
        gui,
        language::{Message, MessageKey},
        menu::{self, RowData, RowDetail, TabData},
    },
};
use lazy_static::lazy_static;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WeaponCategory {
    All,
    Bundles,
    Melee,
    Pistols,
    Shotguns,
    Smg,
    Assault,
    Heavy,
    Snipers,
    Thrown,
}

impl WeaponCategory {
    pub fn name(&self) -> &'static str {
        match self {
            WeaponCategory::All => "SEMUA SENJATA",
            WeaponCategory::Bundles => "PAKET LENGKAP",
            WeaponCategory::Melee => "SENJATA JARAK DEKAT",
            WeaponCategory::Pistols => "PISTOL",
            WeaponCategory::Shotguns => "SHOTGUN",
            WeaponCategory::Smg => "SUBMACHINE GUN",
            WeaponCategory::Assault => "SENAPAN SERBU",
            WeaponCategory::Heavy => "SENJATA BERAT",
            WeaponCategory::Snipers => "SENAPAN RUNDUK",
            WeaponCategory::Thrown => "LEMPAR & GRANAT",
        }
    }

    pub fn next(&self) -> WeaponCategory {
        match self {
            WeaponCategory::All => WeaponCategory::Bundles,
            WeaponCategory::Bundles => WeaponCategory::Melee,
            WeaponCategory::Melee => WeaponCategory::Pistols,
            WeaponCategory::Pistols => WeaponCategory::Shotguns,
            WeaponCategory::Shotguns => WeaponCategory::Smg,
            WeaponCategory::Smg => WeaponCategory::Assault,
            WeaponCategory::Assault => WeaponCategory::Heavy,
            WeaponCategory::Heavy => WeaponCategory::Snipers,
            WeaponCategory::Snipers => WeaponCategory::Thrown,
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
    // --- Melee ---
    WeaponDef { weapon_id: 12, name: "Katana", category: WeaponCategory::Melee, description: "Pedang samurai tajam mematikan" },
    WeaponDef { weapon_id: 13, name: "Chainsaw", category: WeaponCategory::Melee, description: "Gergaji mesin pemotong brutal" },
    WeaponDef { weapon_id: 7,  name: "Baseball Bat", category: WeaponCategory::Melee, description: "Tongkat bisbol pemukul andalan" },
    WeaponDef { weapon_id: 6,  name: "Combat Knife", category: WeaponCategory::Melee, description: "Pisau komando taktis berburu" },
    WeaponDef { weapon_id: 1,  name: "Brass Knuckles", category: WeaponCategory::Melee, description: "Keling tinju besi pelindung tangan" },
    WeaponDef { weapon_id: 3,  name: "Golf Club", category: WeaponCategory::Melee, description: "Stik golf Leaf Links panjang" },
    WeaponDef { weapon_id: 11, name: "Machete", category: WeaponCategory::Melee, description: "Golok parang penebas" },
    WeaponDef { weapon_id: 8,  name: "Meat Cleaver", category: WeaponCategory::Melee, description: "Pisau jagal daging dapur" },
    WeaponDef { weapon_id: 4,  name: "Hammer", category: WeaponCategory::Melee, description: "Palu besi bangunan" },
    WeaponDef { weapon_id: 5,  name: "Nightstick", category: WeaponCategory::Melee, description: "Tongkat pentungan polisi" },

    // --- Pistols ---
    WeaponDef { weapon_id: 18, name: ".357 Python", category: WeaponCategory::Pistols, description: "Revolver Magnum daya rusak super tinggi (1 shot kill)" },
    WeaponDef { weapon_id: 17, name: "Colt .45", category: WeaponCategory::Pistols, description: "Pistol semi-otomatis standar" },

    // --- Shotguns ---
    WeaponDef { weapon_id: 21, name: "SPAS-12", category: WeaponCategory::Shotguns, description: "Shotgun tempur otomatis berdaya hancur tinggi" },
    WeaponDef { weapon_id: 19, name: "Chrome Shotgun", category: WeaponCategory::Shotguns, description: "Shotgun pompa klasik sebaran luas" },
    WeaponDef { weapon_id: 20, name: "Stubby Shotgun", category: WeaponCategory::Shotguns, description: "Shotgun laras pendek tembakan cepat" },

    // --- SMG ---
    WeaponDef { weapon_id: 25, name: "MP5", category: WeaponCategory::Smg, description: "Submachine gun akurasi dan stabilitas tinggi" },
    WeaponDef { weapon_id: 23, name: "Mac-10", category: WeaponCategory::Smg, description: "Submachine gun laju tembak sangat cepat" },
    WeaponDef { weapon_id: 22, name: "Tec-9", category: WeaponCategory::Smg, description: "Pistol mitraliur kapasitas magasin besar" },
    WeaponDef { weapon_id: 24, name: "Uzi", category: WeaponCategory::Smg, description: "Senapan mesin mikro lincah" },

    // --- Assault Rifles ---
    WeaponDef { weapon_id: 27, name: "M4", category: WeaponCategory::Assault, description: "Senapan serbu militer otomatis jarak jauh terbaik" },
    WeaponDef { weapon_id: 26, name: "Kruger (Ruger)", category: WeaponCategory::Assault, description: "Senapan tempur presisi semi/otomatis" },

    // --- Heavy ---
    WeaponDef { weapon_id: 30, name: "Minigun", category: WeaponCategory::Heavy, description: "Meriam putar 6 laras perontok semua kendaraan & lawan" },
    WeaponDef { weapon_id: 28, name: "Rocket Launcher (RPG)", category: WeaponCategory::Heavy, description: "Peluncur roket penghancur tank & helikopter" },
    WeaponDef { weapon_id: 29, name: "Flame Thrower", category: WeaponCategory::Heavy, description: "Penyembur api pembakar area" },
    WeaponDef { weapon_id: 31, name: "M60", category: WeaponCategory::Heavy, description: "Senapan mesin berat militer bertenaga besar" },

    // --- Snipers ---
    WeaponDef { weapon_id: 33, name: "PSG-1 (Laser Scope)", category: WeaponCategory::Snipers, description: "Sniper rifle semi-otomatis dengan teropong laser" },
    WeaponDef { weapon_id: 32, name: "Sniper Rifle", category: WeaponCategory::Snipers, description: "Senapan runduk jarak jauh akurat" },

    // --- Thrown ---
    WeaponDef { weapon_id: 14, name: "Grenade", category: WeaponCategory::Thrown, description: "Granat lempar ledakan dahsyat" },
    WeaponDef { weapon_id: 16, name: "Molotov Cocktail", category: WeaponCategory::Thrown, description: "Bom botol pembakar api" },
    WeaponDef { weapon_id: 15, name: "Remote Grenade", category: WeaponCategory::Thrown, description: "Granat bom peledak detonator jarak jauh" },
    WeaponDef { weapon_id: 10, name: "Tear Gas", category: WeaponCategory::Thrown, description: "Granat gas air mata pelumpuh" },
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

        let weapons: &[u32] = match self.bundle_id {
            // Set 1 (Thugs)
            1 => &[1, 6, 16, 17, 19, 22, 26, 32, 29],
            // Set 2 (Professionals)
            2 => &[12, 15, 18, 20, 23, 27, 33, 28],
            // Set 3 (Nutters)
            _ => &[13, 14, 18, 21, 25, 27, 33, 30],
        };

        for &wid in weapons {
            player::queue_give_weapon(wid, 9999);
        }

        self.activated = true;
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
        player::queue_give_weapon(self.def.weapon_id, 9999);
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
            name: "PAKET 1: THUG TOOLS",
            detail: "Katana/Pisau, Molotov, Pistol .45, Shotgun, Tec-9, Kruger, Sniper, Flamethrower",
            activated: false,
        }));
        rows.push(Box::new(WeaponBundleRow {
            bundle_id: 2,
            name: "PAKET 2: PROFESSIONAL TOOLS",
            detail: "Katana, Granat Remote, Python .357, Stubby, Mac-10, M4, Laser Sniper, RPG",
            activated: false,
        }));
        rows.push(Box::new(WeaponBundleRow {
            bundle_id: 3,
            name: "PAKET 3: NUTTER TOOLS",
            detail: "Chainsaw, Granat, Python .357, SPAS-12, MP5, M4, Laser Sniper, Minigun",
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
