//! Native Vehicle Spawner tab for GTA Vice City.

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
    // --- Sports & Super ---
    VehicleDef { model_id: 141, name: "Infernus", category: VehicleCategory::Sports, description: "Mobil sport super tercepat" },
    VehicleDef { model_id: 145, name: "Cheetah", category: VehicleCategory::Sports, description: "Mobil sport eksotis legendaris" },
    VehicleDef { model_id: 159, name: "Banshee", category: VehicleCategory::Sports, description: "Mobil sport bertenaga tinggi" },
    VehicleDef { model_id: 210, name: "Comet", category: VehicleCategory::Sports, description: "Mobil sport mewah convertible" },
    VehicleDef { model_id: 207, name: "Phoenix", category: VehicleCategory::Sports, description: "Mobil sport langka dengan spoiler besar" },
    VehicleDef { model_id: 132, name: "Stinger", category: VehicleCategory::Sports, description: "Mobil convertible sporty klasik" },
    VehicleDef { model_id: 206, name: "Sabre Turbo", category: VehicleCategory::Sports, description: "Muscle car super kencang" },
    VehicleDef { model_id: 211, name: "Deluxo", category: VehicleCategory::Sports, description: "Mobil sport modern futuristik" },
    VehicleDef { model_id: 224, name: "Hotring Racer", category: VehicleCategory::Sports, description: "Mobil balap NASCAR" },
    VehicleDef { model_id: 232, name: "Hotring Racer A", category: VehicleCategory::Sports, description: "Mobil balap Hotring varian A" },
    VehicleDef { model_id: 233, name: "Hotring Racer B", category: VehicleCategory::Sports, description: "Mobil balap Hotring varian B" },

    // --- Motor & Skuter ---
    VehicleDef { model_id: 191, name: "PCJ-600", category: VehicleCategory::Bikes, description: "Motor sport paling lincah dan cepat" },
    VehicleDef { model_id: 198, name: "Sanchez", category: VehicleCategory::Bikes, description: "Motor trail segala medan" },
    VehicleDef { model_id: 193, name: "Freeway", category: VehicleCategory::Bikes, description: "Motor chopper penjelajah jalanan" },
    VehicleDef { model_id: 166, name: "Angel", category: VehicleCategory::Bikes, description: "Motor geng biker bergaya Amerika" },
    VehicleDef { model_id: 192, name: "Faggio", category: VehicleCategory::Bikes, description: "Skuter klasik perkotaan" },
    VehicleDef { model_id: 178, name: "Pizza Boy", category: VehicleCategory::Bikes, description: "Skuter pengantar pizza Well Stacked" },

    // --- Pesawat & Helikopter ---
    VehicleDef { model_id: 155, name: "Hunter", category: VehicleCategory::HelisPlanes, description: "Helikopter tempur militer bersenjata roket & minigun" },
    VehicleDef { model_id: 177, name: "Sea Sparrow", category: VehicleCategory::HelisPlanes, description: "Helikopter amfibi bersenjata senapan mesin" },
    VehicleDef { model_id: 199, name: "Sparrow", category: VehicleCategory::HelisPlanes, description: "Helikopter ringan gesit" },
    VehicleDef { model_id: 217, name: "Maverick", category: VehicleCategory::HelisPlanes, description: "Helikopter sipil penumpang" },
    VehicleDef { model_id: 227, name: "Police Maverick", category: VehicleCategory::HelisPlanes, description: "Helikopter patroli kepolisian Vice City" },
    VehicleDef { model_id: 218, name: "VCN Maverick", category: VehicleCategory::HelisPlanes, description: "Helikopter berita Vice City News" },
    VehicleDef { model_id: 190, name: "Skimmer", category: VehicleCategory::HelisPlanes, description: "Pesawat amfibi yang bisa lepas landas di air" },
    VehicleDef { model_id: 194, name: "RC Baron", category: VehicleCategory::HelisPlanes, description: "Pesawat mini remote control" },
    VehicleDef { model_id: 195, name: "RC Raider", category: VehicleCategory::HelisPlanes, description: "Helikopter mini remote control Raider" },
    VehicleDef { model_id: 231, name: "RC Goblin", category: VehicleCategory::HelisPlanes, description: "Helikopter mini remote control Goblin" },

    // --- Militer & Darurat ---
    VehicleDef { model_id: 162, name: "Rhino (Tank)", category: VehicleCategory::MilitaryEmergency, description: "Tank tempur lapis baja berat dengan meriam meriam" },
    VehicleDef { model_id: 200, name: "Patriot", category: VehicleCategory::MilitaryEmergency, description: "Kendaraan tempur off-road lapis baja militer" },
    VehicleDef { model_id: 163, name: "Barracks OL", category: VehicleCategory::MilitaryEmergency, description: "Truk angkut tentara militer besar" },
    VehicleDef { model_id: 156, name: "Police Car", category: VehicleCategory::MilitaryEmergency, description: "Mobil patroli polisi VCPD" },
    VehicleDef { model_id: 157, name: "Enforcer", category: VehicleCategory::MilitaryEmergency, description: "Truk taktis pasukan SWAT antirusuh" },
    VehicleDef { model_id: 147, name: "FBI Washington", category: VehicleCategory::MilitaryEmergency, description: "Mobil sedan hitam agen khusus FBI" },
    VehicleDef { model_id: 220, name: "FBI Rancher", category: VehicleCategory::MilitaryEmergency, description: "SUV hitam tangguh operasional FBI" },
    VehicleDef { model_id: 146, name: "Ambulance", category: VehicleCategory::MilitaryEmergency, description: "Mobil medis darurat rumah sakit" },
    VehicleDef { model_id: 137, name: "Firetruck", category: VehicleCategory::MilitaryEmergency, description: "Truk pemadam kebakaran dengan meriam air" },
    VehicleDef { model_id: 150, name: "Taxi", category: VehicleCategory::MilitaryEmergency, description: "Taksi sedan kuning standar" },
    VehicleDef { model_id: 168, name: "Cabbie", category: VehicleCategory::MilitaryEmergency, description: "Taksi bergaya klasik retro" },
    VehicleDef { model_id: 216, name: "Kaufman Cab", category: VehicleCategory::MilitaryEmergency, description: "Taksi armada perusahaan Kaufman" },
    VehicleDef { model_id: 188, name: "Zebra Cab", category: VehicleCategory::MilitaryEmergency, description: "Taksi bermotif zebra spesial" },

    // --- Sedan & Muscle ---
    VehicleDef { model_id: 139, name: "Stretch", category: VehicleCategory::SedansMuscle, description: "Limusin mewah eksekutif" },
    VehicleDef { model_id: 201, name: "Love Fist Limo", category: VehicleCategory::SedansMuscle, description: "Limusin eksklusif band rock Love Fist" },
    VehicleDef { model_id: 175, name: "Admiral", category: VehicleCategory::SedansMuscle, description: "Sedan mewah favorit Tommy Vercetti" },
    VehicleDef { model_id: 151, name: "Washington", category: VehicleCategory::SedansMuscle, description: "Sedan elegan kenyamanan tinggi" },
    VehicleDef { model_id: 174, name: "Sentinel XS", category: VehicleCategory::SedansMuscle, description: "Sedan sport berpeforma tinggi" },
    VehicleDef { model_id: 135, name: "Sentinel", category: VehicleCategory::SedansMuscle, description: "Sedan keluarga eksekutif" },
    VehicleDef { model_id: 164, name: "Cuban Hermes", category: VehicleCategory::SedansMuscle, description: "Mobil kustom geng Kuba knalpot api" },
    VehicleDef { model_id: 204, name: "Hermes", category: VehicleCategory::SedansMuscle, description: "Mobil vintage klasik berkarakter" },
    VehicleDef { model_id: 142, name: "Voodoo", category: VehicleCategory::SedansMuscle, description: "Lowrider legendaris geng Haiti dengan hidrolik" },
    VehicleDef { model_id: 205, name: "Sabre", category: VehicleCategory::SedansMuscle, description: "Muscle car bertenaga besar" },
    VehicleDef { model_id: 169, name: "Stallion", category: VehicleCategory::SedansMuscle, description: "Muscle car ikonik convertible" },
    VehicleDef { model_id: 131, name: "Idaho", category: VehicleCategory::SedansMuscle, description: "Coupe klasik elegan" },
    VehicleDef { model_id: 149, name: "Esperanto", category: VehicleCategory::SedansMuscle, description: "Coupe panjang retro Amerika" },
    VehicleDef { model_id: 221, name: "Virgo", category: VehicleCategory::SedansMuscle, description: "Mobil sedan coupe santai" },
    VehicleDef { model_id: 196, name: "Glendale", category: VehicleCategory::SedansMuscle, description: "Sedan Amerika tahun 50-an" },
    VehicleDef { model_id: 197, name: "Oceanic", category: VehicleCategory::SedansMuscle, description: "Sedan vintage warna-warni tepi pantai" },
    VehicleDef { model_id: 222, name: "Greenwood", category: VehicleCategory::SedansMuscle, description: "Sedan keluarga tahan banting" },
    VehicleDef { model_id: 226, name: "Blista Compact", category: VehicleCategory::SedansMuscle, description: "Hatchback kompak lincah bertenaga" },
    VehicleDef { model_id: 140, name: "Manana", category: VehicleCategory::SedansMuscle, description: "Mobil kompak mungil" },
    VehicleDef { model_id: 134, name: "Perennial", category: VehicleCategory::SedansMuscle, description: "Station wagon keluarga" },
    VehicleDef { model_id: 209, name: "Regina", category: VehicleCategory::SedansMuscle, description: "Station wagon klasik berbagasi luas" },
    VehicleDef { model_id: 172, name: "Romero's Hearse", category: VehicleCategory::SedansMuscle, description: "Mobil jenazah pemakaman Romero" },
    VehicleDef { model_id: 234, name: "Bloodring Banger A", category: VehicleCategory::SedansMuscle, description: "Mobil derby penghancur varian Glendale" },
    VehicleDef { model_id: 235, name: "Bloodring Banger B", category: VehicleCategory::SedansMuscle, description: "Mobil derby penghancur varian Oceanic" },

    // --- Offroad, Vans & Komersil ---
    VehicleDef { model_id: 130, name: "Landstalker", category: VehicleCategory::OffroadVans, description: "SUV mewah berpenggerak 4 roda" },
    VehicleDef { model_id: 219, name: "Rancher", category: VehicleCategory::OffroadVans, description: "SUV tangguh segala rintangan" },
    VehicleDef { model_id: 225, name: "Sandking", category: VehicleCategory::OffroadVans, description: "Truk reli gurun pasir bersuspensi tinggi" },
    VehicleDef { model_id: 230, name: "Mesa Grande", category: VehicleCategory::OffroadVans, description: "Jeep atap terbuka penjelajah" },
    VehicleDef { model_id: 154, name: "BF Injection", category: VehicleCategory::OffroadVans, description: "Buggy pantai berkecepatan tinggi" },
    VehicleDef { model_id: 152, name: "Bobcat", category: VehicleCategory::OffroadVans, description: "Mobil bak pickup serbaguna" },
    VehicleDef { model_id: 208, name: "Walton", category: VehicleCategory::OffroadVans, description: "Truk pickup pedesaan klasik" },
    VehicleDef { model_id: 187, name: "Caddy", category: VehicleCategory::OffroadVans, description: "Mobil golf Leaf Links yang gesit" },
    VehicleDef { model_id: 212, name: "Burrito", category: VehicleCategory::OffroadVans, description: "Van kargo serbaguna" },
    VehicleDef { model_id: 179, name: "Gang Burrito", category: VehicleCategory::OffroadVans, description: "Van geng bersuara gahar" },
    VehicleDef { model_id: 170, name: "Rumpo", category: VehicleCategory::OffroadVans, description: "Van komersial lincah" },
    VehicleDef { model_id: 143, name: "Pony", category: VehicleCategory::OffroadVans, description: "Mobil van ekspedisi" },
    VehicleDef { model_id: 148, name: "Moonbeam", category: VehicleCategory::OffroadVans, description: "Minivan keluarga 4 pintu" },
    VehicleDef { model_id: 228, name: "Boxville", category: VehicleCategory::OffroadVans, description: "Truk boks ekspedisi paket" },
    VehicleDef { model_id: 229, name: "Benson", category: VehicleCategory::OffroadVans, description: "Truk niaga ukuran sedang" },
    VehicleDef { model_id: 144, name: "Mule", category: VehicleCategory::OffroadVans, description: "Truk kargo pengangkut barang" },
    VehicleDef { model_id: 186, name: "Yankee", category: VehicleCategory::OffroadVans, description: "Truk kontainer berukuran besar" },
    VehicleDef { model_id: 189, name: "Topfun", category: VehicleCategory::OffroadVans, description: "Van pengangkut mainan RC Topfun" },
    VehicleDef { model_id: 213, name: "Spand Express", category: VehicleCategory::OffroadVans, description: "Truk logistik Spand Express" },
    VehicleDef { model_id: 153, name: "Mr. Whoopee", category: VehicleCategory::OffroadVans, description: "Mobil penjual es krim bertopi ceria" },
    VehicleDef { model_id: 158, name: "Securicar", category: VehicleCategory::OffroadVans, description: "Mobil lapis baja pengangkut brankas uang" },
    VehicleDef { model_id: 138, name: "Trashmaster", category: VehicleCategory::OffroadVans, description: "Truk pengangkut sampah kota" },
    VehicleDef { model_id: 161, name: "Bus", category: VehicleCategory::OffroadVans, description: "Bus transit penumpang kota" },
    VehicleDef { model_id: 167, name: "Coach", category: VehicleCategory::OffroadVans, description: "Bus pariwisata antar-kota" },
    VehicleDef { model_id: 215, name: "Baggage Handler", category: VehicleCategory::OffroadVans, description: "Kendaraan penarik bagasi bandara Escobar" },

    // --- Kapal & Perahu ---
    VehicleDef { model_id: 182, name: "Speeder", category: VehicleCategory::Boats, description: "Speedboat bertenaga jet berkecepatan tinggi" },
    VehicleDef { model_id: 176, name: "Squalo", category: VehicleCategory::Boats, description: "Speedboat balap perairan mewah" },
    VehicleDef { model_id: 223, name: "Jetmax", category: VehicleCategory::Boats, description: "Perahu balap bertenaga monster" },
    VehicleDef { model_id: 160, name: "Predator", category: VehicleCategory::Boats, description: "Perahu patroli polisi dengan senapan mesin ganda" },
    VehicleDef { model_id: 184, name: "Tropic", category: VehicleCategory::Boats, description: "Kapal pesiar santai berpantai" },
    VehicleDef { model_id: 136, name: "Rio", category: VehicleCategory::Boats, description: "Kapal pesiar mewah kabin tertutup" },
    VehicleDef { model_id: 203, name: "Dinghy", category: VehicleCategory::Boats, description: "Perahu karet motor tempel cepat" },
    VehicleDef { model_id: 202, name: "Coastguard", category: VehicleCategory::Boats, description: "Perahu penjaga pantai penyelamat" },
    VehicleDef { model_id: 214, name: "Marquis", category: VehicleCategory::Boats, description: "Kapal layar pesiar santai" },
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
        RowDetail::Info(Message::custom(format!("ID {} • {}", self.def.model_id, self.def.description)))
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
