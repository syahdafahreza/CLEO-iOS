//! Native Vehicle Spawner tab for GTA III.
//! Vehicle model IDs are for GTA III iOS (armv7 / 32-bit).
//! TODO: Verify all model IDs after binary IPA analysis.

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
pub enum VehicleCategory {
    All,
    Sports,
    Bikes,
    HelisPlanes,
    MilitaryEmergency,
    SedansMuscle,
    OffroadVans,
    Boats,
}

impl VehicleCategory {
    pub fn name(&self) -> &'static str {
        match self {
            VehicleCategory::All => "SEMUA KENDARAAN",
            VehicleCategory::Sports => "SPORT & SUPER",
            VehicleCategory::Bikes => "MOTOR & SKUTER",
            VehicleCategory::HelisPlanes => "PESAWAT & HELI",
            VehicleCategory::MilitaryEmergency => "MILITER & DARURAT",
            VehicleCategory::SedansMuscle => "SEDAN & MUSCLE",
            VehicleCategory::OffroadVans => "OFFROAD & VAN",
            VehicleCategory::Boats => "KAPAL & PERAHU",
        }
    }

    pub fn next(&self) -> VehicleCategory {
        match self {
            VehicleCategory::All => VehicleCategory::Sports,
            VehicleCategory::Sports => VehicleCategory::Bikes,
            VehicleCategory::Bikes => VehicleCategory::HelisPlanes,
            VehicleCategory::HelisPlanes => VehicleCategory::MilitaryEmergency,
            VehicleCategory::MilitaryEmergency => VehicleCategory::SedansMuscle,
            VehicleCategory::SedansMuscle => VehicleCategory::OffroadVans,
            VehicleCategory::OffroadVans => VehicleCategory::Boats,
            VehicleCategory::Boats => VehicleCategory::All,
        }
    }
}

lazy_static! {
    static ref CURRENT_CATEGORY: Mutex<VehicleCategory> = Mutex::new(VehicleCategory::All);
}

#[derive(Clone, Copy)]
pub struct VehicleDef {
    pub model_id: u32,
    pub name: &'static str,
    pub category: VehicleCategory,
    pub description: &'static str,
}

pub static VEHICLES: &[VehicleDef] = &[
    // =========================================================================
    // GTA III Vehicle List (model IDs from GTA III iOS)
    // TODO: Verify model IDs after binary IPA analysis
    // =========================================================================

    // --- Sports & Super ---
    VehicleDef { model_id: 130, name: "Infernus", category: VehicleCategory::Sports, description: "Mobil sport tercepat di Liberty City" },
    VehicleDef { model_id: 131, name: "Cheetah", category: VehicleCategory::Sports, description: "Mobil sport eksotis mewah" },
    VehicleDef { model_id: 132, name: "Stinger", category: VehicleCategory::Sports, description: "Mobil convertible sporty" },
    VehicleDef { model_id: 133, name: "Banshee", category: VehicleCategory::Sports, description: "Mobil sport bertenaga tinggi" },
    VehicleDef { model_id: 134, name: "Idaho", category: VehicleCategory::Sports, description: "Coupe klasik elegan" },

    // --- Motor ---
    VehicleDef { model_id: 135, name: "Freeway", category: VehicleCategory::Bikes, description: "Motor chopper penjelajah jalanan" },
    VehicleDef { model_id: 136, name: "Angel", category: VehicleCategory::Bikes, description: "Motor geng biker bergaya Amerika" },
    VehicleDef { model_id: 137, name: "PCJ-600", category: VehicleCategory::Bikes, description: "Motor sport paling lincah dan cepat" },

    // --- Helikopter & Pesawat ---
    VehicleDef { model_id: 138, name: "Dodo", category: VehicleCategory::HelisPlanes, description: "Pesawat kecil sayap terpotong ikonik GTA 3" },
    VehicleDef { model_id: 139, name: "Maverick", category: VehicleCategory::HelisPlanes, description: "Helikopter sipil penumpang" },
    VehicleDef { model_id: 140, name: "Police Maverick", category: VehicleCategory::HelisPlanes, description: "Helikopter patroli kepolisian LCPD" },
    VehicleDef { model_id: 141, name: "Heli", category: VehicleCategory::HelisPlanes, description: "Helikopter ringan serbaguna" },

    // --- Militer & Darurat ---
    VehicleDef { model_id: 142, name: "Rhino (Tank)", category: VehicleCategory::MilitaryEmergency, description: "Tank tempur lapis baja Rhino" },
    VehicleDef { model_id: 143, name: "Barracks OL", category: VehicleCategory::MilitaryEmergency, description: "Truk angkut tentara militer besar" },
    VehicleDef { model_id: 144, name: "Police Car", category: VehicleCategory::MilitaryEmergency, description: "Mobil patroli polisi LCPD" },
    VehicleDef { model_id: 145, name: "Enforcer", category: VehicleCategory::MilitaryEmergency, description: "Truk taktis pasukan SWAT" },
    VehicleDef { model_id: 146, name: "FBI Car", category: VehicleCategory::MilitaryEmergency, description: "Mobil agen khusus FBI" },
    VehicleDef { model_id: 147, name: "Ambulance", category: VehicleCategory::MilitaryEmergency, description: "Mobil medis darurat" },
    VehicleDef { model_id: 148, name: "Firetruck", category: VehicleCategory::MilitaryEmergency, description: "Truk pemadam kebakaran" },
    VehicleDef { model_id: 149, name: "Taxi", category: VehicleCategory::MilitaryEmergency, description: "Taksi Liberty City" },
    VehicleDef { model_id: 150, name: "Cabbie", category: VehicleCategory::MilitaryEmergency, description: "Taksi bergaya klasik retro" },

    // --- Sedan & Muscle ---
    VehicleDef { model_id: 151, name: "Stretch", category: VehicleCategory::SedansMuscle, description: "Limusin mewah eksekutif" },
    VehicleDef { model_id: 152, name: "Manana", category: VehicleCategory::SedansMuscle, description: "Sedan kompak mungil" },
    VehicleDef { model_id: 153, name: "Peren", category: VehicleCategory::SedansMuscle, description: "Station wagon keluarga" },
    VehicleDef { model_id: 154, name: "Sentinel", category: VehicleCategory::SedansMuscle, description: "Sedan keluarga eksekutif" },
    VehicleDef { model_id: 155, name: "Sentinel XS", category: VehicleCategory::SedansMuscle, description: "Sedan sport berperforma tinggi" },
    VehicleDef { model_id: 156, name: "Flatbed", category: VehicleCategory::SedansMuscle, description: "Truk flatbed bak terbuka" },
    VehicleDef { model_id: 157, name: "Yankee", category: VehicleCategory::SedansMuscle, description: "Truk kontainer berukuran besar" },
    VehicleDef { model_id: 158, name: "Bobcat", category: VehicleCategory::SedansMuscle, description: "Pickup truck serbaguna" },
    VehicleDef { model_id: 159, name: "Stallion", category: VehicleCategory::SedansMuscle, description: "Muscle car ikonik convertible" },
    VehicleDef { model_id: 160, name: "Esperanto", category: VehicleCategory::SedansMuscle, description: "Coupe panjang retro Amerika" },
    VehicleDef { model_id: 161, name: "Patriot", category: VehicleCategory::SedansMuscle, description: "SUV lapis baja militer" },
    VehicleDef { model_id: 162, name: "Mr. Whoopee", category: VehicleCategory::SedansMuscle, description: "Mobil penjual es krim" },
    VehicleDef { model_id: 163, name: "BF Injection", category: VehicleCategory::SedansMuscle, description: "Buggy pantai berkecepatan tinggi" },
    VehicleDef { model_id: 164, name: "Hoods Rumpo XL", category: VehicleCategory::SedansMuscle, description: "Van geng Southside Hoods" },
    VehicleDef { model_id: 165, name: "Pony", category: VehicleCategory::SedansMuscle, description: "Van ekspedisi serbaguna" },
    VehicleDef { model_id: 166, name: "Moonbeam", category: VehicleCategory::SedansMuscle, description: "Minivan keluarga" },
    VehicleDef { model_id: 167, name: "Speeder", category: VehicleCategory::Boats, description: "Speedboat bertenaga jet" },
    VehicleDef { model_id: 168, name: "Reefer", category: VehicleCategory::Boats, description: "Kapal nelayan kayu kecil" },
    VehicleDef { model_id: 169, name: "Predator", category: VehicleCategory::Boats, description: "Perahu patroli polisi" },
    VehicleDef { model_id: 170, name: "Dinghy", category: VehicleCategory::Boats, description: "Perahu karet motor tempel" },
    VehicleDef { model_id: 171, name: "Ghost", category: VehicleCategory::Boats, description: "Perahu kayu misterius" },

    // --- Offroad & Vans ---
    VehicleDef { model_id: 172, name: "Bus", category: VehicleCategory::OffroadVans, description: "Bus transit penumpang kota" },
    VehicleDef { model_id: 173, name: "Coach", category: VehicleCategory::OffroadVans, description: "Bus pariwisata antar-kota" },
    VehicleDef { model_id: 174, name: "Rumpo", category: VehicleCategory::OffroadVans, description: "Van komersial lincah" },
    VehicleDef { model_id: 175, name: "RC Bandit", category: VehicleCategory::OffroadVans, description: "Buggy mini remote control" },
    VehicleDef { model_id: 176, name: "Trashmaster", category: VehicleCategory::OffroadVans, description: "Truk pengangkut sampah" },
    VehicleDef { model_id: 177, name: "Stretch", category: VehicleCategory::OffroadVans, description: "Limusin mafia Panlantic" },
    VehicleDef { model_id: 178, name: "Mule", category: VehicleCategory::OffroadVans, description: "Truk kargo pengangkut barang" },
    VehicleDef { model_id: 179, name: "Linerunner", category: VehicleCategory::OffroadVans, description: "Truk semi-trailer penarik" },
    VehicleDef { model_id: 180, name: "Securicar", category: VehicleCategory::OffroadVans, description: "Mobil lapis baja pengangkut uang" },
    VehicleDef { model_id: 181, name: "Landstalker", category: VehicleCategory::OffroadVans, description: "SUV mewah berpenggerak 4 roda" },
];

struct CategoryFilterRow;

impl RowData for CategoryFilterRow {
    fn title(&self) -> Message {
        let cat = *CURRENT_CATEGORY.lock().unwrap();
        Message::custom(format!("KATEGORI: {}", cat.name()))
    }

    fn detail(&self) -> RowDetail {
        RowDetail::Info(Message::custom("Sentuh untuk ganti kategori kendaraan"))
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
        true // Rebuild rows!
    }
}

struct VehicleRow {
    def: VehicleDef,
    spawn_count: u32,
}

impl RowData for VehicleRow {
    fn title(&self) -> Message {
        Message::custom(self.def.name)
    }

    fn detail(&self) -> RowDetail {
        RowDetail::Info(Message::custom(format!("Model ID {} • {}", self.def.model_id, self.def.description)))
    }

    fn value(&self) -> Message {
        if self.spawn_count > 0 {
            MessageKey::CheatActionOk.to_message()
        } else {
            Message::custom("SPAWN")
        }
    }

    fn tint(&self) -> Option<(u8, u8, u8)> {
        if self.spawn_count > 0 {
            Some(gui::colours::GREEN)
        } else {
            None
        }
    }

    fn handle_tap(&mut self) -> bool {
        player::queue_spawn_vehicle(self.def.model_id);
        self.spawn_count = self.spawn_count.saturating_add(1);
        true
    }
}

pub fn tab_data() -> TabData {
    let current_cat = *CURRENT_CATEGORY.lock().unwrap();
    let mut rows: Vec<Box<dyn RowData>> = vec![];

    // Filter selector row at the top
    rows.push(Box::new(CategoryFilterRow));

    for def in VEHICLES {
        if current_cat == VehicleCategory::All || def.category == current_cat {
            rows.push(Box::new(VehicleRow {
                def: *def,
                spawn_count: 0,
            }));
        }
    }

    TabData {
        name: MessageKey::VehiclesTabTitle.to_message(),
        warning: None,
        row_data: rows,
    }
}
