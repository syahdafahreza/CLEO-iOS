//! Replaces the game's broken cheats system with our own system that integrates with the menu.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

use crate::{
    call_original, hook,
    meta::{
        gui,
        language::{self, Message, MessageKey},
        menu::{self, RowData, TabData},
        settings::{CheatTransience, Options},
    },
};
use lazy_static::lazy_static;
#[allow(unused_imports)]
use log::error;
use once_cell::sync::Lazy;

pub struct Cheat {
    index: usize,
    code: &'static str,
    description: MessageKey,
}

lazy_static! {
    static ref WAITING_CHEATS: std::sync::Mutex<Vec<usize>> = std::sync::Mutex::new(vec![]);
}

impl Cheat {
    const fn new(index: usize, code: &'static str, description: MessageKey) -> Cheat {
        Cheat {
            index,
            code,
            description,
        }
    }

    fn get_function(&self) -> Option<fn()> {
        #[cfg(target_pointer_width = "64")]
        {
            let entry_address = 0x10065c358 + (self.index * 8);
            let ptr = hook::slide::<*const usize>(entry_address);
            if ptr.is_null() {
                None
            } else {
                let raw_fn: usize = unsafe { *ptr };
                if raw_fn == 0 {
                    None
                } else {
                    let func: fn() = unsafe { std::mem::transmute(raw_fn) };
                    Some(func)
                }
            }
        }

        #[cfg(target_pointer_width = "32")]
        {
            None
        }
    }

    fn get_active_mut(&self) -> &'static mut bool {
        #[cfg(target_pointer_width = "64")]
        unsafe {
            let base = 0x10072dda8;
            hook::slide::<*mut bool>(base + self.index)
                .as_mut()
                .unwrap()
        }

        #[cfg(target_pointer_width = "32")]
        unsafe {
            static mut VC_CHEAT_ACTIVE: [bool; 37] = [false; 37];
            &mut VC_CHEAT_ACTIVE[self.index]
        }
    }

    fn is_active(&self) -> bool {
        *self.get_active_mut()
    }

    fn queue(&self) {
        let mut waiting = WAITING_CHEATS.lock().unwrap();
        waiting.push(self.index);
    }

    fn cancel(&self) {
        let mut waiting = WAITING_CHEATS.lock().unwrap();
        waiting.retain(|cheat_index| *cheat_index != self.index);
    }

    fn is_in_queue(&self) -> bool {
        let waiting = WAITING_CHEATS.lock().unwrap();
        waiting.contains(&self.index)
    }

    fn run(&self) {
        #[cfg(target_pointer_width = "32")]
        {
            log::info!("Activating VC cheat index {} ({})", self.index, self.code);
            call_vc_cheat(self.index);
            let active = self.get_active_mut();
            *active = !*active;
            return;
        }

        #[cfg(target_pointer_width = "64")]
        {
            if let Some(function) = self.get_function() {
                log::info!("Calling cheat function {:?}", function);
                function();
                return;
            }

            // If the cheat has no function pointer, then we need to toggle its active status.
            let active = self.get_active_mut();
            *active = !*active;
        }
    }
}

// (a, b) where a = "check b" and b = "save the cheat states". a is false when a save is in progress.
// This is a weak system for avoiding two saves happening at the same time, but it works well enough.
// Besides, it's practically impossible for a save to be triggered when one is already in progress because
//  saving is very fast.
static SAVE_FLAGS: Lazy<(AtomicBool, AtomicBool)> =
    Lazy::new(|| (AtomicBool::new(true), AtomicBool::new(false)));

// CCheat::DoCheats is where cheat codes are checked and then cheats activated (indirectly),
//  so we need to do our cheat stuff here to ensure that the cheats don't fuck up the game by
//  doing stuff at weird times. The point in CGame::Process where DoCheats is called is where
//  every other system in the game expects cheats to be activated.
// Cheats that need textures to be loaded - such as weapon or vehicle cheats - can crash the
//  game if they are executed on the wrong thread or at the wrong time, so it is very important
pub fn run_waiting_cheats() {
    if let Ok(mut waiting) = WAITING_CHEATS.lock() {
        for cheat_index in waiting.iter() {
            CHEATS[*cheat_index].run();
        }
        waiting.clear();
    }
}

#[cfg(target_pointer_width = "64")]
fn do_cheats() {
    if let Ok(waiting) = WAITING_CHEATS.lock().as_mut() {
        // Perform all queued cheat actions.
        for cheat_index in waiting.iter() {
            CHEATS[*cheat_index].run();
        }

        // Clear the queue.
        waiting.clear();
    } else {
        error!("Unable to lock cheat queue for CCheat::DoCheats!");
    }

    if SAVE_FLAGS.0.load(Ordering::SeqCst) && SAVE_FLAGS.1.load(Ordering::SeqCst) {
        // Ignore further requests to save the cheat states.
        SAVE_FLAGS.0.store(false, Ordering::SeqCst);

        // We need to save the cheat states now. We only do this after the cheat functions have been called because
        //  some will clear their "enabled" status when they run.
        // todo: Check that saving cheats after execution is always a good idea.

        // We just save an array of bytes in the order of the cheats, with each being 1 or 0 depending on the status of that cheat.
        // We do this outside of our saving thread because we don't want the statuses to change while we're accessing them.
        let cheat_state_bytes: Vec<u8> = CHEATS
            .iter()
            .map(|cheat| {
                let is_active = cheat.is_active();
                if is_active {
                    log::info!("Saving status ON for cheat '{}'.", cheat.code);
                }
                is_active as u8
            })
            .collect();

        std::thread::spawn(|| {
            if let Err(err) = std::fs::write(
                crate::meta::resources::get_documents_path("cleo_saved_cheats.u8"),
                cheat_state_bytes,
            ) {
                log::error!("Error while saving cheat states: {}", err);
            } else {
                log::info!("Cheat states saved successfully.");
            }

            // Clear save request flag (so we don't keep saving infinitely).
            SAVE_FLAGS.1.store(false, Ordering::SeqCst);

            // Allow new saving requests to be processed.
            SAVE_FLAGS.0.store(true, Ordering::SeqCst);
        });
    }
}

struct CheatData {
    cheat: &'static Cheat,
    queued_state: Option<bool>,
}

impl CheatData {
    fn new(cheat: &'static Cheat) -> CheatData {
        CheatData {
            cheat,
            queued_state: if cheat.is_in_queue() {
                Some(!cheat.is_active())
            } else {
                None
            },
        }
    }

    fn will_be_active(&self) -> bool {
        self.queued_state.unwrap_or_else(|| self.cheat.is_active())
    }
}

impl RowData for CheatData {
    fn title(&self) -> Message {
        if self.cheat.code.is_empty() {
            MessageKey::CheatNoCodeTitle.to_message()
        } else {
            MessageKey::CheatCodeRowTitle
                .format(language::msg_args!["cheat_code" => self.cheat.code])
        }
    }

    fn detail(&self) -> menu::RowDetail {
        menu::RowDetail::Info(self.cheat.description.to_message())
    }

    fn value(&self) -> Message {
        /*
            State                       Tint        Status

            In queue, turning on        Blue        "Queued On"
            In queue, turning off       Red         "Queued Off"
            Not in queue, on            Green       "On"
            Not in queue, off           None        "Off"
        */

        let will_be_active = self.will_be_active();

        let key = if self.cheat.is_in_queue() {
            if will_be_active {
                MessageKey::CheatQueuedOn
            } else {
                MessageKey::CheatQueuedOff
            }
        } else if will_be_active {
            MessageKey::CheatOn
        } else {
            MessageKey::CheatOff
        };

        key.to_message()
    }

    fn tint(&self) -> Option<(u8, u8, u8)> {
        let will_be_active = self.will_be_active();

        if self.cheat.is_in_queue() {
            if will_be_active {
                Some(gui::colours::BLUE)
            } else {
                Some(gui::colours::RED)
            }
        } else if will_be_active {
            Some(gui::colours::GREEN)
        } else {
            None
        }
    }

    fn handle_tap(&mut self) -> bool {
        let is_queued = self.queued_state.is_some();

        if is_queued {
            // Remove the cheat from the queue.
            self.cheat.cancel();
            self.queued_state = None;
        } else {
            self.queued_state = Some(!self.cheat.is_active());

            // Not queued yet.
            self.cheat.queue();
        }

        if let CheatTransience::Persistent = Options::get().cheat_transience {
            // Request that the cheats be saved because it is likely that a status will change.
            SAVE_FLAGS.1.store(true, Ordering::SeqCst);
        }

        true
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CheatCategory {
    All,
    Weapons,
    HealthWanted,
    Vehicles,
    Traffic,
    Player,
    WeatherTime,
    Chaos,
    Misc,
}

impl CheatCategory {
    pub fn name(&self) -> &'static str {
        match self {
            CheatCategory::All => "SEMUA",
            CheatCategory::Weapons => "SENJATA",
            CheatCategory::HealthWanted => "KESEHATAN & POLISI",
            CheatCategory::Vehicles => "SPAWN KENDARAAN",
            CheatCategory::Traffic => "LALU LINTAS",
            CheatCategory::Player => "KARAKTER CJ",
            CheatCategory::WeatherTime => "CUACA & WAKTU",
            CheatCategory::Chaos => "KEKACAUAN (CHAOS)",
            CheatCategory::Misc => "LAIN-LAIN",
        }
    }

    pub fn next(&self) -> CheatCategory {
        match self {
            CheatCategory::All => CheatCategory::Weapons,
            CheatCategory::Weapons => CheatCategory::HealthWanted,
            CheatCategory::HealthWanted => CheatCategory::Vehicles,
            CheatCategory::Vehicles => CheatCategory::Traffic,
            CheatCategory::Traffic => CheatCategory::Player,
            CheatCategory::Player => CheatCategory::WeatherTime,
            CheatCategory::WeatherTime => CheatCategory::Chaos,
            CheatCategory::Chaos => CheatCategory::Misc,
            CheatCategory::Misc => CheatCategory::All,
        }
    }

    pub fn matches(&self, cat: CheatCategory) -> bool {
        match self {
            CheatCategory::All => true,
            _ => *self == cat,
        }
    }
}

lazy_static! {
    static ref CURRENT_CATEGORY: Mutex<CheatCategory> = Mutex::new(CheatCategory::All);
}

pub fn get_cheat_category(key: MessageKey) -> CheatCategory {
    match key {
        // Weapons
        MessageKey::CheatThugsArmoury
        | MessageKey::CheatProfessionalsKit
        | MessageKey::CheatNuttersToys
        | MessageKey::CheatWeapons4
        | MessageKey::CheatFullClip
        | MessageKey::CheatIWannaDriveby
        | MessageKey::CheatSlotMelee
        | MessageKey::CheatSlotHandgun
        | MessageKey::CheatSlotSmg
        | MessageKey::CheatSlotShotgun
        | MessageKey::CheatSlotAssaultRifle
        | MessageKey::CheatSlotLongRifle
        | MessageKey::CheatSlotThrown
        | MessageKey::CheatSlotHeavy
        | MessageKey::CheatSlotEquipment
        | MessageKey::CheatSlotOther => CheatCategory::Weapons,

        // Health & Wanted
        MessageKey::CheatINeedSomeHelp
        | MessageKey::CheatFullInvincibility
        | MessageKey::CheatNooneCanHurtMe
        | MessageKey::CheatTurnUpTheHeat
        | MessageKey::CheatTurnDownTheHeat
        | MessageKey::CheatIDoAsIPlease
        | MessageKey::CheatBringItOn => CheatCategory::HealthWanted,

        // Vehicles Spawner
        MessageKey::Cheat18Holes
        | MessageKey::CheatOldSpeedDemon
        | MessageKey::CheatTintedRancher
        | MessageKey::CheatNotForPublicRoads
        | MessageKey::CheatJustTryAndStopMe
        | MessageKey::CheatWheresTheFuneral
        | MessageKey::CheatCelebrityStatus
        | MessageKey::CheatTrueGrime
        | MessageKey::CheatJumpJet
        | MessageKey::CheatIWantToHover
        | MessageKey::CheatOhDude
        | MessageKey::CheatFourWheelFun
        | MessageKey::CheatHitTheRoadJack
        | MessageKey::CheatItsAllBull
        | MessageKey::CheatFlyingToStunt
        | MessageKey::CheatMonsterMash
        | MessageKey::CheatRocketman
        | MessageKey::CheatLetsGoBaseJumping
        | MessageKey::CheatPredator => CheatCategory::Vehicles,

        // Traffic & Vehicle physics
        MessageKey::CheatAllCarsGoBoom
        | MessageKey::CheatWheelsOnlyPlease
        | MessageKey::CheatStickLikeGlue
        | MessageKey::CheatDontTryAndStopMe
        | MessageKey::CheatAllDriversAreCriminals
        | MessageKey::CheatPinkIsTheNewCool
        | MessageKey::CheatSoLongAsItsBlack
        | MessageKey::CheatSidewaysWheels
        | MessageKey::CheatFlyingFish
        | MessageKey::CheatEveryoneIsPoor
        | MessageKey::CheatEveryoneIsRich
        | MessageKey::CheatChittyChittyBangBang
        | MessageKey::CheatCjPhoneHome
        | MessageKey::CheatTouchMyCarYouDie
        | MessageKey::CheatSpeedFreak
        | MessageKey::CheatBubbleCars
        | MessageKey::CheatGhostTown
        | MessageKey::CheatCoolTaxis => CheatCategory::Traffic,

        // Player & CJ stats
        MessageKey::CheatStingLikeABee
        | MessageKey::CheatIAmNeverHungry
        | MessageKey::CheatKangaroo
        | MessageKey::CheatManFromAtlantis
        | MessageKey::CheatWorshipMe
        | MessageKey::CheatHelloLadies
        | MessageKey::CheatWhoAteAllThePies
        | MessageKey::CheatBuffMeUp
        | MessageKey::CheatMaxGambling
        | MessageKey::CheatLeanAndMean
        | MessageKey::CheatICanGoAllNight
        | MessageKey::CheatProfessionalKiller
        | MessageKey::CheatNaturalTalent
        | MessageKey::CheatGoodbyeCruelWorld
        | MessageKey::CheatTakeAChillPill => CheatCategory::Player,

        // Weather & Time
        MessageKey::CheatPleasantlyWarm
        | MessageKey::CheatTooDamnHot
        | MessageKey::CheatDullDullDay
        | MessageKey::CheatStayInAndWatchTv
        | MessageKey::CheatCantSeeWhereImGoing
        | MessageKey::CheatScottishSummer
        | MessageKey::CheatSandInMyEars
        | MessageKey::CheatClockForward
        | MessageKey::CheatTimeJustFliesBy
        | MessageKey::CheatSpeedItUp
        | MessageKey::CheatSlowItDown
        | MessageKey::CheatNightProwler
        | MessageKey::CheatDontBringOnTheNight => CheatCategory::WeatherTime,

        // Chaos & Riots
        MessageKey::CheatRoughNeighbourhood
        | MessageKey::CheatStopPickingOnMe
        | MessageKey::CheatSurroundedByNutters
        | MessageKey::CheatTimeToKickAss
        | MessageKey::CheatBlueSuedeShoes
        | MessageKey::CheatAttackOfTheVillagePeople
        | MessageKey::CheatLifesABeach
        | MessageKey::CheatOnlyHomiesAllowed
        | MessageKey::CheatBetterStayIndoors
        | MessageKey::CheatNinjaTown
        | MessageKey::CheatLoveConquersAll
        | MessageKey::CheatStateOfEmergency
        | MessageKey::CheatHicksville
        | MessageKey::CheatWannaBeInMyGang
        | MessageKey::CheatNooneCanStopUs
        | MessageKey::CheatRocketMayhem
        | MessageKey::CheatCrazyTown => CheatCategory::Chaos,

        // Misc
        MessageKey::CheatSkipMission
        | MessageKey::CheatProstitutesPay
        | MessageKey::CheatDebugMappings
        | MessageKey::CheatDebugTapToTarget
        | MessageKey::CheatDebugTargeting
        | MessageKey::CheatXboxHelper => CheatCategory::Misc,

        _ => CheatCategory::Misc,
    }
}

struct CategoryFilterRow {
    category: CheatCategory,
    count: usize,
}

impl RowData for CategoryFilterRow {
    fn title(&self) -> Message {
        MessageKey::CheatCodeRowTitle
            .format(language::msg_args!["cheat_code" => "[ KATEGORI CHEAT ]".to_string()])
    }

    fn detail(&self) -> menu::RowDetail {
        menu::RowDetail::Info(
            MessageKey::CheatCodeRowTitle.format(
                language::msg_args!["cheat_code" => "Ketuk untuk ganti filter kategori".to_string()],
            ),
        )
    }

    fn value(&self) -> Message {
        MessageKey::CheatCodeRowTitle.format(language::msg_args![
            "cheat_code" => format!("{} ({}) ▸", self.category.name(), self.count)
        ])
    }

    fn tint(&self) -> Option<(u8, u8, u8)> {
        Some(gui::colours::BLUE)
    }

    fn handle_tap(&mut self) -> bool {
        let mut cat = CURRENT_CATEGORY.lock().unwrap();
        *cat = cat.next();
        drop(cat);

        crate::meta::menu::MenuMessage::RebuildTab(2).send();
        false
    }
}

static ALL_SORTED_CHEATS: Lazy<Vec<&'static Cheat>> = Lazy::new(|| {
    let mut vec: Vec<&'static Cheat> = CHEATS.iter().collect();
    vec.sort_by_key(|cheat| {
        if cheat.code.is_empty() {
            "ZZZZZ"
        } else {
            cheat.code
        }
    });
    vec
});

pub fn tab_data() -> TabData {
    let current_cat = *CURRENT_CATEGORY.lock().unwrap();

    let filtered_cheats: Vec<&'static Cheat> = ALL_SORTED_CHEATS
        .iter()
        .copied()
        .filter(|cheat| current_cat.matches(get_cheat_category(cheat.description)))
        .collect();

    let count = filtered_cheats.len();

    let mut rows: Vec<Box<dyn RowData>> = Vec::with_capacity(count + 1);
    rows.push(Box::new(CategoryFilterRow {
        category: current_cat,
        count,
    }));

    for cheat in filtered_cheats {
        rows.push(Box::new(CheatData::new(cheat)));
    }

    TabData {
        name: MessageKey::CheatTabTitle.to_message(),
        warning: Some(MessageKey::CheatMenuWarning.to_message()),
        row_data: rows,
    }
}

#[cfg(target_pointer_width = "64")]
fn reset_cheats() {
    log::info!("Resetting cheats");
    call_original!(crate::targets::reset_cheats);

    if !matches!(Options::get().cheat_transience, CheatTransience::Persistent) {
        log::info!("Cheat saving/loading is disabled.");
        return;
    }

    log::info!("Loading saved cheats.");

    let path = crate::meta::resources::get_documents_path("cleo_saved_cheats.u8");

    if !path.exists() {
        log::info!("No saved cheats file found.");
        return;
    }

    match std::fs::read(path) {
        Ok(loaded_bytes) => {
            // Check that we have a file that matches our cheat count.
            if loaded_bytes.len() != CHEATS.len() {
                log::error!("Invalid cheat save: byte count must match cheat count.");
                return;
            }

            // Ensure that all the bytes are valid.
            for byte in &loaded_bytes {
                if byte != &0 && byte != &1 {
                    log::error!("Invalid cheat save: found non-Boolean byte.");
                    return;
                }
            }

            // Set all the cheat statuses according to the bytes in the file.
            for (i, b) in loaded_bytes.iter().enumerate() {
                *CHEATS[i].get_active_mut() = b == &1;
            }

            log::info!("Cheats loaded successfully.");
        }
        Err(err) => {
            log::error!("Error loading cheat file: {}", err);
        }
    }
}

pub fn init() {
    log::info!("installing cheat hooks...");
    #[cfg(target_pointer_width = "64")]
    {
        crate::targets::do_cheats::install(do_cheats);
        crate::targets::reset_cheats::install(reset_cheats);
    }
}

// We have to include the codes because the game doesn't have the array.
// Android does, though, so I copied the codes from there. The order has been preserved.
// The spreadsheet at
//   https://docs.google.com/spreadsheets/d/1-rmga12W9reALga7fct22tJ-1thxbbsfGiGltK2qgh0/edit?usp=sharing
//  was very helpful during research, and the page at https://gta.fandom.com/wiki/Cheats_in_GTA_San_Andreas
//  was really useful for writing cheat descriptions.

#[cfg(target_pointer_width = "32")]
fn call_vc_cheat(index: usize) {
    match index {
        0 => hook::slide_fn::<extern "C" fn()>(0x000795b8)(), // THUGSTOOLS
        1 => hook::slide_fn::<extern "C" fn()>(0x0007946c)(), // PROFESSIONALTOOLS
        2 => hook::slide_fn::<extern "C" fn()>(0x00079314)(), // NUTTERTOOLS
        3 => hook::slide_fn::<extern "C" fn()>(0x00077e34)(), // PRECIOUSPROTECTION
        4 => hook::slide_fn::<extern "C" fn(u32)>(0x00079254)(1), // ASPIRINE
        5 => hook::slide_fn::<extern "C" fn()>(0x00077edc)(), // YOUWONTTAKEMEALIVE
        6 => hook::slide_fn::<extern "C" fn()>(0x00077e84)(), // LEAVEMEALONE
        7 => hook::slide_fn::<extern "C" fn()>(0x0007874c)(), // APLEASANTDAY
        8 => hook::slide_fn::<extern "C" fn()>(0x0007871c)(), // ALOVELYDAY
        9 => hook::slide_fn::<extern "C" fn()>(0x000786ec)(), // ABITDRIEG
        10 => hook::slide_fn::<extern "C" fn()>(0x000786bc)(), // CATSANDDOGS
        11 => hook::slide_fn::<extern "C" fn()>(0x0007868c)(), // CANTSEEATHING
        12 => hook::slide_fn::<extern "C" fn()>(0x00079130)(), // PANZER
        13 => hook::slide_fn::<extern "C" fn()>(0x00079b94)(), // LIFEISPASSINGMEBY
        14 => hook::slide_fn::<extern "C" fn()>(0x00077dc4)(), // BIGBANG
        15 => hook::slide_fn::<extern "C" fn()>(0x0007877c)(), // STILLLIKEDRESSINGUP
        16 => hook::slide_fn::<extern "C" fn()>(0x00077d54)(), // FIGHTFIGHTFIGHT
        17 => hook::slide_fn::<extern "C" fn()>(0x00077ce8)(), // NOBODYLIKESME
        18 => hook::slide_fn::<extern "C" fn()>(0x00077c84)(), // OURGODGIVENRIGHTTOBEARARMS
        19 => hook::slide_fn::<extern "C" fn()>(0x00077c2c)(), // ONSPEED
        20 => hook::slide_fn::<extern "C" fn()>(0x00077bd0)(), // BOOOOOORING
        21 => hook::slide_fn::<extern "C" fn()>(0x00077b6c)(), // WHEELSAREALLINEED
        22 => hook::slide_fn::<extern "C" fn()>(0x00077b08)(), // COMEFLYWITHME
        23 => hook::slide_fn::<extern "C" fn()>(0x00077aa4)(), // GRIPISEVERYTHING
        24 => hook::slide_fn::<extern "C" fn()>(0x00079d84)(), // CHASESTAT
        25 => hook::slide_fn::<extern "C" fn()>(0x000785fc)(), // CHICKSWITHGUNS
        26 => hook::slide_fn::<extern "C" fn()>(0x000785b4)(), // ICANTTAKEITANYMORE
        27 => hook::slide_fn::<extern "C" fn()>(0x00079e2c)(), // GREENLIGHT
        28 => hook::slide_fn::<extern "C" fn()>(0x00079e84)(), // MIAMITRAFFIC
        29 => hook::slide_fn::<extern "C" fn()>(0x00079ed8)(), // AHAIRDRESSERSCAR
        30 => hook::slide_fn::<extern "C" fn()>(0x00079f40)(), // IWANTITPAINTEDBLACK
        31 => hook::slide_fn::<extern "C" fn(u32)>(0x00078920)(234), // TRAVELINSTYLE (Bloodring Banger)
        32 => hook::slide_fn::<extern "C" fn(u32)>(0x00078920)(172), // THELASTRIDE (Romero's Hearse)
        33 => hook::slide_fn::<extern "C" fn(u32)>(0x00078920)(201), // ROCKANDROLLCAR (Love Fist Limo)
        34 => hook::slide_fn::<extern "C" fn(u32)>(0x00078920)(138), // RUBBISHCAR (Trashmaster)
        35 => hook::slide_fn::<extern "C" fn(u32)>(0x00078920)(206), // GETTHEREFAST (Sabre Turbo)
        36 => hook::slide_fn::<extern "C" fn(u32)>(0x00078920)(187), // BETTERTHANWALKING (Caddy)
        _ => {}
    }
}

#[cfg(target_pointer_width = "32")]
static CHEATS: [Cheat; 37] = [
    Cheat::new(0, "THUGSTOOLS", MessageKey::CheatThugsArmoury),
    Cheat::new(1, "PROFESSIONALTOOLS", MessageKey::CheatProfessionalsKit),
    Cheat::new(2, "NUTTERTOOLS", MessageKey::CheatNuttersToys),
    Cheat::new(3, "PRECIOUSPROTECTION", MessageKey::CheatFullInvincibility),
    Cheat::new(4, "ASPIRINE", MessageKey::CheatINeedSomeHelp),
    Cheat::new(5, "YOUWONTTAKEMEALIVE", MessageKey::CheatTurnUpTheHeat),
    Cheat::new(6, "LEAVEMEALONE", MessageKey::CheatTurnDownTheHeat),
    Cheat::new(7, "APLEASANTDAY", MessageKey::CheatTooDamnHot),
    Cheat::new(8, "ALOVELYDAY", MessageKey::CheatPleasantlyWarm),
    Cheat::new(9, "ABITDRIEG", MessageKey::CheatDullDullDay),
    Cheat::new(10, "CATSANDDOGS", MessageKey::CheatStayInAndWatchTv),
    Cheat::new(11, "CANTSEEATHING", MessageKey::CheatCantSeeWhereImGoing),
    Cheat::new(12, "PANZER", MessageKey::CheatTimeToKickAss),
    Cheat::new(13, "LIFEISPASSINGMEBY", MessageKey::CheatTimeJustFliesBy),
    Cheat::new(14, "BIGBANG", MessageKey::CheatAllCarsGoBoom),
    Cheat::new(15, "STILLLIKEDRESSINGUP", MessageKey::CheatBlueSuedeShoes),
    Cheat::new(16, "FIGHTFIGHTFIGHT", MessageKey::CheatStateOfEmergency),
    Cheat::new(17, "NOBODYLIKESME", MessageKey::CheatStopPickingOnMe),
    Cheat::new(18, "OURGODGIVENRIGHTTOBEARARMS", MessageKey::CheatSurroundedByNutters),
    Cheat::new(19, "ONSPEED", MessageKey::CheatSpeedItUp),
    Cheat::new(20, "BOOOOOORING", MessageKey::CheatSlowItDown),
    Cheat::new(21, "WHEELSAREALLINEED", MessageKey::CheatWheelsOnlyPlease),
    Cheat::new(22, "COMEFLYWITHME", MessageKey::CheatChittyChittyBangBang),
    Cheat::new(23, "GRIPISEVERYTHING", MessageKey::CheatStickLikeGlue),
    Cheat::new(24, "CHASESTAT", MessageKey::CheatDebugMappings),
    Cheat::new(25, "CHICKSWITHGUNS", MessageKey::CheatCrazyTown),
    Cheat::new(26, "ICANTTAKEITANYMORE", MessageKey::CheatGoodbyeCruelWorld),
    Cheat::new(27, "GREENLIGHT", MessageKey::CheatDontTryAndStopMe),
    Cheat::new(28, "MIAMITRAFFIC", MessageKey::CheatAllDriversAreCriminals),
    Cheat::new(29, "AHAIRDRESSERSCAR", MessageKey::CheatPinkIsTheNewCool),
    Cheat::new(30, "IWANTITPAINTEDBLACK", MessageKey::CheatSoLongAsItsBlack),
    Cheat::new(31, "TRAVELINSTYLE", MessageKey::CheatOldSpeedDemon),
    Cheat::new(32, "THELASTRIDE", MessageKey::CheatWheresTheFuneral),
    Cheat::new(33, "ROCKANDROLLCAR", MessageKey::CheatCelebrityStatus),
    Cheat::new(34, "RUBBISHCAR", MessageKey::CheatTrueGrime),
    Cheat::new(35, "GETTHEREFAST", MessageKey::CheatNotForPublicRoads),
    Cheat::new(36, "BETTERTHANWALKING", MessageKey::Cheat18Holes),
];


#[cfg(target_pointer_width = "64")]
static CHEATS: [Cheat; 111] = [
    Cheat::new(0, "THUGSARMOURY", MessageKey::CheatThugsArmoury),
    Cheat::new(1, "PROFESSIONALSKIT", MessageKey::CheatProfessionalsKit),
    Cheat::new(2, "NUTTERSTOYS", MessageKey::CheatNuttersToys),
    Cheat::new(3, "", MessageKey::CheatWeapons4),
    Cheat::new(4, "", MessageKey::CheatClockForward),
    Cheat::new(5, "", MessageKey::CheatSkipMission),
    Cheat::new(6, "", MessageKey::CheatDebugMappings),
    Cheat::new(7, "", MessageKey::CheatFullInvincibility),
    Cheat::new(8, "", MessageKey::CheatDebugTapToTarget),
    Cheat::new(9, "", MessageKey::CheatDebugTargeting),
    Cheat::new(10, "INEEDSOMEHELP", MessageKey::CheatINeedSomeHelp),
    Cheat::new(11, "TURNUPTHEHEAT", MessageKey::CheatTurnUpTheHeat),
    Cheat::new(12, "TURNDOWNTHEHEAT", MessageKey::CheatTurnDownTheHeat),
    Cheat::new(13, "PLEASANTLYWARM", MessageKey::CheatPleasantlyWarm),
    Cheat::new(14, "TOODAMNHOT", MessageKey::CheatTooDamnHot),
    Cheat::new(15, "DULLDULLDAY", MessageKey::CheatDullDullDay),
    Cheat::new(16, "STAYINANDWATCHTV", MessageKey::CheatStayInAndWatchTv),
    Cheat::new(
        17,
        "CANTSEEWHEREIMGOING",
        MessageKey::CheatCantSeeWhereImGoing,
    ),
    Cheat::new(18, "TIMEJUSTFLIESBY", MessageKey::CheatTimeJustFliesBy),
    Cheat::new(19, "SPEEDITUP", MessageKey::CheatSpeedItUp),
    Cheat::new(20, "SLOWITDOWN", MessageKey::CheatSlowItDown),
    Cheat::new(
        21,
        "ROUGHNEIGHBOURHOOD",
        MessageKey::CheatRoughNeighbourhood,
    ),
    Cheat::new(22, "STOPPICKINGONME", MessageKey::CheatStopPickingOnMe),
    Cheat::new(
        23,
        "SURROUNDEDBYNUTTERS",
        MessageKey::CheatSurroundedByNutters,
    ),
    Cheat::new(24, "TIMETOKICKASS", MessageKey::CheatTimeToKickAss),
    Cheat::new(25, "OLDSPEEDDEMON", MessageKey::CheatOldSpeedDemon),
    Cheat::new(26, "", MessageKey::CheatTintedRancher),
    Cheat::new(27, "NOTFORPUBLICROADS", MessageKey::CheatNotForPublicRoads),
    Cheat::new(28, "JUSTTRYANDSTOPME", MessageKey::CheatJustTryAndStopMe),
    Cheat::new(29, "WHERESTHEFUNERAL", MessageKey::CheatWheresTheFuneral),
    Cheat::new(30, "CELEBRITYSTATUS", MessageKey::CheatCelebrityStatus),
    Cheat::new(31, "TRUEGRIME", MessageKey::CheatTrueGrime),
    Cheat::new(32, "18HOLES", MessageKey::Cheat18Holes),
    Cheat::new(33, "ALLCARSGOBOOM", MessageKey::CheatAllCarsGoBoom),
    Cheat::new(34, "WHEELSONLYPLEASE", MessageKey::CheatWheelsOnlyPlease),
    Cheat::new(35, "STICKLIKEGLUE", MessageKey::CheatStickLikeGlue),
    Cheat::new(36, "GOODBYECRUELWORLD", MessageKey::CheatGoodbyeCruelWorld),
    Cheat::new(37, "DONTTRYANDSTOPME", MessageKey::CheatDontTryAndStopMe),
    Cheat::new(
        38,
        "ALLDRIVERSARECRIMINALS",
        MessageKey::CheatAllDriversAreCriminals,
    ),
    Cheat::new(39, "PINKISTHENEWCOOL", MessageKey::CheatPinkIsTheNewCool),
    Cheat::new(40, "SOLONGASITSBLACK", MessageKey::CheatSoLongAsItsBlack),
    Cheat::new(41, "", MessageKey::CheatSidewaysWheels),
    Cheat::new(42, "FLYINGFISH", MessageKey::CheatFlyingFish),
    Cheat::new(43, "WHOATEALLTHEPIES", MessageKey::CheatWhoAteAllThePies),
    Cheat::new(44, "BUFFMEUP", MessageKey::CheatBuffMeUp),
    Cheat::new(45, "", MessageKey::CheatMaxGambling),
    Cheat::new(46, "LEANANDMEAN", MessageKey::CheatLeanAndMean),
    Cheat::new(47, "BLUESUEDESHOES", MessageKey::CheatBlueSuedeShoes),
    Cheat::new(
        48,
        "ATTACKOFTHEVILLAGEPEOPLE",
        MessageKey::CheatAttackOfTheVillagePeople,
    ),
    Cheat::new(49, "LIFESABEACH", MessageKey::CheatLifesABeach),
    Cheat::new(50, "ONLYHOMIESALLOWED", MessageKey::CheatOnlyHomiesAllowed),
    Cheat::new(51, "BETTERSTAYINDOORS", MessageKey::CheatBetterStayIndoors),
    Cheat::new(52, "NINJATOWN", MessageKey::CheatNinjaTown),
    Cheat::new(53, "LOVECONQUERSALL", MessageKey::CheatLoveConquersAll),
    Cheat::new(54, "EVERYONEISPOOR", MessageKey::CheatEveryoneIsPoor),
    Cheat::new(55, "EVERYONEISRICH", MessageKey::CheatEveryoneIsRich),
    Cheat::new(
        56,
        "CHITTYCHITTYBANGBANG",
        MessageKey::CheatChittyChittyBangBang,
    ),
    Cheat::new(57, "CJPHONEHOME", MessageKey::CheatCjPhoneHome),
    Cheat::new(58, "JUMPJET", MessageKey::CheatJumpJet),
    Cheat::new(59, "IWANTTOHOVER", MessageKey::CheatIWantToHover),
    Cheat::new(60, "TOUCHMYCARYOUDIE", MessageKey::CheatTouchMyCarYouDie),
    Cheat::new(61, "SPEEDFREAK", MessageKey::CheatSpeedFreak),
    Cheat::new(62, "BUBBLECARS", MessageKey::CheatBubbleCars),
    Cheat::new(63, "NIGHTPROWLER", MessageKey::CheatNightProwler),
    Cheat::new(
        64,
        "DONTBRINGONTHENIGHT",
        MessageKey::CheatDontBringOnTheNight,
    ),
    Cheat::new(65, "SCOTTISHSUMMER", MessageKey::CheatScottishSummer),
    Cheat::new(66, "SANDINMYEARS", MessageKey::CheatSandInMyEars),
    Cheat::new(67, "", MessageKey::CheatPredator),
    Cheat::new(68, "KANGAROO", MessageKey::CheatKangaroo),
    Cheat::new(69, "NOONECANHURTME", MessageKey::CheatNooneCanHurtMe),
    Cheat::new(70, "MANFROMATLANTIS", MessageKey::CheatManFromAtlantis),
    Cheat::new(71, "LETSGOBASEJUMPING", MessageKey::CheatLetsGoBaseJumping),
    Cheat::new(72, "ROCKETMAN", MessageKey::CheatRocketman),
    Cheat::new(73, "IDOASIPLEASE", MessageKey::CheatIDoAsIPlease),
    Cheat::new(74, "BRINGITON", MessageKey::CheatBringItOn),
    Cheat::new(75, "STINGLIKEABEE", MessageKey::CheatStingLikeABee),
    Cheat::new(76, "IAMNEVERHUNGRY", MessageKey::CheatIAmNeverHungry),
    Cheat::new(77, "STATEOFEMERGENCY", MessageKey::CheatStateOfEmergency),
    Cheat::new(78, "CRAZYTOWN", MessageKey::CheatCrazyTown),
    Cheat::new(79, "TAKEACHILLPILL", MessageKey::CheatTakeAChillPill),
    Cheat::new(80, "FULLCLIP", MessageKey::CheatFullClip),
    Cheat::new(81, "IWANNADRIVEBY", MessageKey::CheatIWannaDriveby),
    Cheat::new(82, "GHOSTTOWN", MessageKey::CheatGhostTown),
    Cheat::new(83, "HICKSVILLE", MessageKey::CheatHicksville),
    Cheat::new(84, "WANNABEINMYGANG", MessageKey::CheatWannaBeInMyGang),
    Cheat::new(85, "NOONECANSTOPUS", MessageKey::CheatNooneCanStopUs),
    Cheat::new(86, "ROCKETMAYHEM", MessageKey::CheatRocketMayhem),
    Cheat::new(87, "WORSHIPME", MessageKey::CheatWorshipMe),
    Cheat::new(88, "HELLOLADIES", MessageKey::CheatHelloLadies),
    Cheat::new(89, "ICANGOALLNIGHT", MessageKey::CheatICanGoAllNight),
    Cheat::new(
        90,
        "PROFESSIONALKILLER",
        MessageKey::CheatProfessionalKiller,
    ),
    Cheat::new(91, "NATURALTALENT", MessageKey::CheatNaturalTalent),
    Cheat::new(92, "OHDUDE", MessageKey::CheatOhDude),
    Cheat::new(93, "FOURWHEELFUN", MessageKey::CheatFourWheelFun),
    Cheat::new(94, "HITTHEROADJACK", MessageKey::CheatHitTheRoadJack),
    Cheat::new(95, "ITSALLBULL", MessageKey::CheatItsAllBull),
    Cheat::new(96, "FLYINGTOSTUNT", MessageKey::CheatFlyingToStunt),
    Cheat::new(97, "MONSTERMASH", MessageKey::CheatMonsterMash),
    Cheat::new(98, "", MessageKey::CheatProstitutesPay),
    Cheat::new(99, "", MessageKey::CheatCoolTaxis),
    Cheat::new(100, "", MessageKey::CheatSlotMelee),
    Cheat::new(101, "", MessageKey::CheatSlotHandgun),
    Cheat::new(102, "", MessageKey::CheatSlotSmg),
    Cheat::new(103, "", MessageKey::CheatSlotShotgun),
    Cheat::new(104, "", MessageKey::CheatSlotAssaultRifle),
    Cheat::new(105, "", MessageKey::CheatSlotLongRifle),
    Cheat::new(106, "", MessageKey::CheatSlotThrown),
    Cheat::new(107, "", MessageKey::CheatSlotHeavy),
    Cheat::new(108, "", MessageKey::CheatSlotEquipment),
    Cheat::new(109, "", MessageKey::CheatSlotOther),
    Cheat::new(110, "", MessageKey::CheatXboxHelper),
];
