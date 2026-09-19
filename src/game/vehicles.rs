//! Native Vehicle Spawner tab for GTA III.
//! Vehicle model IDs are from GTA III iOS (armv7 / 32-bit) default.ide.

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
    Gang,
    EmergencyMilitary,
    SedansMuscle,
    VansTrucks,
    BoatsAir,
}

impl VehicleCategory {
    pub fn name(&self) -> &'static str {
        match self {
            VehicleCategory::All => "SEMUA KENDARAAN",
            VehicleCategory::Sports => "SPORT & SUPER",
            VehicleCategory::Gang => "MOBIL GENG",
            VehicleCategory::EmergencyMilitary => "DARURAT & MILITER",
            VehicleCategory::SedansMuscle => "SEDAN & MUSCLE",
            VehicleCategory::VansTrucks => "VAN & TRUK",
            VehicleCategory::BoatsAir => "KAPAL & PESAWAT",
        }
    }

    pub fn next(&self) -> VehicleCategory {
        match self {
            VehicleCategory::All => VehicleCategory::Sports,
            VehicleCategory::Sports => VehicleCategory::Gang,
            VehicleCategory::Gang => VehicleCategory::EmergencyMilitary,
            VehicleCategory::EmergencyMilitary => VehicleCategory::SedansMuscle,
            VehicleCategory::SedansMuscle => VehicleCategory::VansTrucks,
            VehicleCategory::VansTrucks => VehicleCategory::BoatsAir,
            VehicleCategory::BoatsAir => VehicleCategory::All,
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
    // GTA III iOS Vehicle List (Verified from binary model table at 0x1CCB10)
    // Model IDs 90 to 146
    // =========================================================================

    // --- Sport & Super ---
    VehicleDef { model_id: 101, name: "Infernus", category: VehicleCategory::Sports, description: "Mobil super tercepat di Liberty City" },
    VehicleDef { model_id: 105, name: "Cheetah", category: VehicleCategory::Sports, description: "Mobil sport eksotis mewah bertenaga besar" },
    VehicleDef { model_id: 118, name: "Banshee", category: VehicleCategory::Sports, description: "Mobil sport legendaris convertible" },
    VehicleDef { model_id: 92,  name: "Stinger", category: VehicleCategory::Sports, description: "Mobil sport elegan atap terbuka" },
    VehicleDef { model_id: 114, name: "BF Injection", category: VehicleCategory::Sports, description: "Buggy pantai lincah dan gesit" },

    // --- Mobil Geng Liberty City ---
    VehicleDef { model_id: 133, name: "Mafia Sentinel", category: VehicleCategory::Gang, description: "Sedan mewah berpelindung keluarga Leone" },
    VehicleDef { model_id: 135, name: "Yakuza Stinger", category: VehicleCategory::Gang, description: "Sport convertible geng Yakuza" },
    VehicleDef { model_id: 134, name: "Yardie Lobo", category: VehicleCategory::Gang, description: "Lowrider bermesin hidrolik geng Yardie" },
    VehicleDef { model_id: 136, name: "Diablo Stallion", category: VehicleCategory::Gang, description: "Muscle car garang geng Diablos" },
    VehicleDef { model_id: 137, name: "Cartel Cruiser", category: VehicleCategory::Gang, description: "SUV mewah kartel Kolombia berbadan tinggi" },
    VehicleDef { model_id: 138, name: "Hoods Rumpo XL", category: VehicleCategory::Gang, description: "Van bertenaga besar geng Southside Hoods" },

    // --- Darurat & Militer ---
    VehicleDef { model_id: 121, name: "Rhino (Tank)", category: VehicleCategory::EmergencyMilitary, description: "Tank lapis baja senjata meriam mematikan" },
    VehicleDef { model_id: 122, name: "Barracks OL", category: VehicleCategory::EmergencyMilitary, description: "Truk angkut militer berukuran besar" },
    VehicleDef { model_id: 115, name: "Police Car", category: VehicleCategory::EmergencyMilitary, description: "Mobil patroli kepolisian LCPD" },
    VehicleDef { model_id: 116, name: "Enforcer", category: VehicleCategory::EmergencyMilitary, description: "Truk taktis pasukan khusus SWAT" },
    VehicleDef { model_id: 107, name: "FBI Kuruma", category: VehicleCategory::EmergencyMilitary, description: "Sedan hitam taktis agen khusus FBI" },
    VehicleDef { model_id: 106, name: "Ambulance", category: VehicleCategory::EmergencyMilitary, description: "Mobil medis darurat LCPD" },
    VehicleDef { model_id: 97,  name: "Firetruck", category: VehicleCategory::EmergencyMilitary, description: "Truk pemadam kebakaran dengan meriam air" },
    VehicleDef { model_id: 117, name: "Securicar", category: VehicleCategory::EmergencyMilitary, description: "Mobil lapis baja pengangkut uang bank" },
    VehicleDef { model_id: 110, name: "Taxi", category: VehicleCategory::EmergencyMilitary, description: "Taksi kuning khas Liberty City" },
    VehicleDef { model_id: 127, name: "Cabbie", category: VehicleCategory::EmergencyMilitary, description: "Taksi klasik retro Liberty City" },
    VehicleDef { model_id: 146, name: "Borgnine Cabbie", category: VehicleCategory::EmergencyMilitary, description: "Taksi spesial bertanduk bertenaga turbo" },

    // --- Sedan & Muscle ---
    VehicleDef { model_id: 99,  name: "Stretch", category: VehicleCategory::SedansMuscle, description: "Limusin mewah kelas atas" },
    VehicleDef { model_id: 95,  name: "Sentinel", category: VehicleCategory::SedansMuscle, description: "Sedan bisnis eksekutif 4 pintu" },
    VehicleDef { model_id: 111, name: "Kuruma", category: VehicleCategory::SedansMuscle, description: "Sedan 4 pintu andalan Liberty City" },
    VehicleDef { model_id: 128, name: "Stallion", category: VehicleCategory::SedansMuscle, description: "Muscle car bertenaga besar" },
    VehicleDef { model_id: 109, name: "Esperanto", category: VehicleCategory::SedansMuscle, description: "Coupe Amerika klasik berbodi panjang" },
    VehicleDef { model_id: 91,  name: "Idaho", category: VehicleCategory::SedansMuscle, description: "Coupe 2 pintu bergaya retro" },
    VehicleDef { model_id: 100, name: "Manana", category: VehicleCategory::SedansMuscle, description: "Sedan kompak mungil 2 pintu" },
    VehicleDef { model_id: 94,  name: "Perennial", category: VehicleCategory::SedansMuscle, description: "Station wagon keluarga" },
    VehicleDef { model_id: 102, name: "Blista", category: VehicleCategory::SedansMuscle, description: "Minivan kompak serbaguna" },

    // --- Van, Pickup & Truk ---
    VehicleDef { model_id: 90,  name: "Landstalker", category: VehicleCategory::VansTrucks, description: "SUV mewah 4x4 penjelajah segala medan" },
    VehicleDef { model_id: 96,  name: "Patriot", category: VehicleCategory::VansTrucks, description: "SUV militer tangguh dan kuat" },
    VehicleDef { model_id: 112, name: "Bobcat", category: VehicleCategory::VansTrucks, description: "Truk pickup bak terbuka serbaguna" },
    VehicleDef { model_id: 108, name: "Moonbeam", category: VehicleCategory::VansTrucks, description: "Minivan keluarga berbodi luas" },
    VehicleDef { model_id: 129, name: "Rumpo", category: VehicleCategory::VansTrucks, description: "Van kargo komersial" },
    VehicleDef { model_id: 103, name: "Pony", category: VehicleCategory::VansTrucks, description: "Van ekspedisi antar barang" },
    VehicleDef { model_id: 104, name: "Mule", category: VehicleCategory::VansTrucks, description: "Truk box ekspedisi sedang" },
    VehicleDef { model_id: 145, name: "Yankee", category: VehicleCategory::VansTrucks, description: "Truk kontainer besar" },
    VehicleDef { model_id: 144, name: "Flatbed", category: VehicleCategory::VansTrucks, description: "Truk flatbed pengangkut muatan berat" },
    VehicleDef { model_id: 93,  name: "Linerunner", category: VehicleCategory::VansTrucks, description: "Truk kepala semi-trailer bertenaga tinggi" },
    VehicleDef { model_id: 98,  name: "Trashmaster", category: VehicleCategory::VansTrucks, description: "Truk pengangkut sampah lapis tebal" },
    VehicleDef { model_id: 120, name: "Bus", category: VehicleCategory::VansTrucks, description: "Bus transit penumpang kota LCPD" },
    VehicleDef { model_id: 126, name: "Coach", category: VehicleCategory::VansTrucks, description: "Bus pariwisata antar kota" },
    VehicleDef { model_id: 113, name: "Mr. Whoopee", category: VehicleCategory::VansTrucks, description: "Mobil penjual es krim berlagu khas" },
    VehicleDef { model_id: 130, name: "RC Bandit", category: VehicleCategory::VansTrucks, description: "Mobil mini remote control peledak" },
    VehicleDef { model_id: 131, name: "Belly Up", category: VehicleCategory::VansTrucks, description: "Truk box pedagang ikan Triad" },
    VehicleDef { model_id: 132, name: "Mr. Wong's", category: VehicleCategory::VansTrucks, description: "Van pengantar binatu Mr. Wong" },
    VehicleDef { model_id: 143, name: "Panlantic", category: VehicleCategory::VansTrucks, description: "Van perusahaan konstruksi Panlantic" },

    // --- Kapal & Pesawat ---
    VehicleDef { model_id: 125, name: "Dodo", category: VehicleCategory::BoatsAir, description: "Pesawat kecil ikonik GTA 3 bersayap potong" },
    VehicleDef { model_id: 119, name: "Predator", category: VehicleCategory::BoatsAir, description: "Perahu patroli bersenjata polisi perairan" },
    VehicleDef { model_id: 141, name: "Speeder", category: VehicleCategory::BoatsAir, description: "Speedboat cepat bertenaga tinggi" },
    VehicleDef { model_id: 142, name: "Reefer", category: VehicleCategory::BoatsAir, description: "Kapal nelayan perairan Liberty City" },
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
