# Dokumentasi Pengembangan CLEO untuk GTA III — iPhone 5 (ARMv7, iOS 10.x)

## Informasi Proyek

| Parameter | Nilai |
|-----------|-------|
| **Branch Git** | `For-iPhone-5-GTA3` |
| **Dibuat dari** | `For-iPhone-5-VC` |
| **Target Game** | Grand Theft Auto III (GTA III iOS) |
| **Versi Game** | v1.3.2 (com.rockstargames.gta3ios) |
| **Perangkat** | iPhone 5 |
| **iOS** | 10.x |
| **Arsitektur CPU** | ARMv7 / 32-bit Mach-O (`armv7s-apple-ios`) |
| **TEXT Base** | `0x00001000` |
| **DATA Base** | `0x001BA000` |
| **Jailbreak** | Rootful |

---

## Status Pengembangan

> [!NOTE]
> **Analisis Binary Selesai!**
> Semua memory address dan struct offset di bawah ini telah dianalisis langsung dari binary Mach-O ARMv7 GTA III v1.3.2 dan file data (`default.ide`, `weapon.dat`) dari IPA.

---

## Tabel Memory Addresses (GTA III v1.3.2 ARMv7)

### A. Hooks Utama (`src/lib.rs`)

| Nama Target | Deskripsi | Address GTA III | Catatan / Simbol Asli |
|-------------|-----------|-----------------|----------------------|
| `script_tick` | Main script tick loop | `0x00049734` | `CTheScripts::Process()` |
| `process_touch` | Handler layar sentuh | `0x000054E0` | Dipanggil dari 4 method `[EAGLView touches...]` |
| `get_gxt_string` | Lookup teks GXT | `0x00057E80` | `CText::Get(this, key)` |
| `legal_splash` | Hook splash screen | `0x00097AD4` | `splash` texture rendering |
| `legal_splash_german`| Splash screen German | `0x000328B2` | Movie intro selector (`GTAtitlesGER.mpg.m4v`) |
| `store_crash_fix` | Fix crash StoreKit | `0x00008895` | `-[IOSBillingObserver productsRequest:didReceiveResponse:]` |
| `gen_plate` | License plate generator | `0x00000000` | GTA 3 tidak menggunakan plat dinamis |
| `do_game_state` | State machine game | `0x00032268` | `CGame::bGameState` switch statement |
| `end_dragging` | UI gesture handler | `0x000432C0` | `TouchscreenNipple` destructor / handler |
| `loading_messages` | Loading screen text | `0x00098394` | `LoadingScreen(msg1, msg2)` |
| `height_above_ceiling`| Tinggi kendaraan | `0x0009C274` | `CVehicle::GetHeightAboveRoad()` |
| `init_for_title` | Game init | `0x0008940C` | `CGame::Initialise()` |
| `load_settings` | Load setting game | `0x00107FE8` | `CMenuManager::LoadSettings()` |

---

### B. Fungsi Player & Game Engine (`src/game/player.rs`)

| Nama Fungsi | Deskripsi | Address GTA III | Catatan / Parameter |
|-------------|-----------|-----------------|---------------------|
| `find_player_ped()` | Claude pointer (`CPed*`) | `0x000F4CC8` | Returns `CPlayerPed*` Claude |
| `find_player_vehicle()` | Kendaraan aktif Claude | `0x000F4C14` | Returns `CVehicle*` atau null |
| `CWorld::Players` | Array `CPlayerInfo` | `0x003175F0` | Pointer di `0x001BA284` |
| `PlayerInFocus` | Index pemain aktif | `0x003796E4` | Pointer di `0x001BA280` |
| `ms_aInfoForModel` | Pointer tabel CStreamingInfo | `0x001BA6A4` | Struct 20 byte, offset 8 adalah `m_nLoadState` (1 = Loaded) |
| `operator new` | Alokasi memori C++ | `0x001388DC` | `extern "C" fn(size: usize) -> *mut u8` |
| `CAutomobile::CAutomobile` | Konstruktor mobil | `0x0009C23C` | Ukuran objek: `0x488` (1160 bytes) |
| `CBoat::CBoat` | Konstruktor kapal | `0x00068E1C` | Ukuran objek: `0x5AC` (1452 bytes) |
| `CWorld::AlignToGroundAndRoof` | Perata tanah & atap mobil | `0x000C5064` | `(&target_pos, veh)` — dipanggil di native vehicle cheat |
| `CMatrix::SetRotate` | Rotasi matriks entitas | `0x0005A9D0` | `(matrix, rx, ry, rz)` — rotasi Euler RenderWare |
| `CWorld::Add` | Registrasi ke dunia | `0x0003B090` | `CWorld::Add(CEntity*)` |
| `CStreaming::RequestModel` | Request load model | `0x0011AEF0` | `(model_id, flags)` |
| `CStreaming::LoadAllRequestedModels`| Load model synch | `0x0011D954` | `(priority: bool)` |
| `CPed::GiveWeapon` | Beri senjata & amunisi | `0x000DE170` | `(ped, weapon_type, ammo, bool)` |
| `CWeaponInfo::GetWeaponInfo` | Ambil info spesifikasi senjata | `0x0002C5BC` | `(weapon_type) -> *mut CWeaponInfo` (clip capacity di `+0x10`) |
| `CWanted::SetWantedLevel` | Set bintang polisi | `0x0002F2D0` | `(wanted_ptr, level)` |
| `CPed::SetWantedLevel` | Helper wanted di ped | `0x0001DE64` | `(ped, level)` |
| `MaximumWantedLevel` | Batas maksimum bintang | `0x001BEBC8` | Default: 6 |
| `MaximumChaosLevel` | Batas maksimum chaos | `0x001BEBCC` | Default: 6400 (`0x1900`) |
| `CDamageManager::SetEngineStatus` | Fix mesin kendaraan | `0x000CC560` | Offset di `veh + 0x28C`, status 0 |
| `CDamageManager::Reset` | Reset seluruh kerusakan part | `0x000CC700` | `memset(veh + 0x28C, 0, 0x1C)` |
| `CAutomobile::Fix` | Servis total bodi mobil (Pay N Spray) | `0x0005CE78` | Memulihkan mesh utuh, bodi, kap, bumper, pintu |
| `CFire::Extinguish` | Padamkan api objek / mobil | `0x00068EF0` | Padamkan pointer CFire (`veh + 0x1E8`) |
| `CClock::SetGameClock` | Set jam & menit permainan | `0x000103CC` | `(hours, minutes)` — reset detik ke 0 & update tick |

---

### C. Struct Offsets yang Terverifikasi

| Struct | Field | Offset GTA III | Keterangan |
|--------|-------|----------------|------------|
| `CPed` | Health (`m_fHealth`) | `0x2C4` | Single float (100.0f) — dibuktikan dari `GESUNDHEIT` (`0x000c0c0c`) |
| `CPed` | Armor (`m_fArmour`) | `0x2C8` | Single float (100.0f) — dibuktikan dari `TORTOISE` (`0x000c05d8`) |
| `CPed` | Kendaraan (`m_pMyVehicle`) | `0x314` | Pointer ke `CVehicle` |
| `CPed` | Status dalam mobil | `0x318` | `bool m_bInVehicle` |
| `CPed` | Basis slot senjata | `0x360` | 12 slot senjata |
| `CPed` | Stride tiap slot senjata | `0x18` | 24 bytes per slot |
| `CWeapon` | Clip ammo | `+0x08` | Jumlah peluru dalam magazin |
| `CWeapon` | Total ammo | `+0x0C` | Total sisa peluru |
| `CPed` | Wanted info (`CWanted*`) | `0x544` | Pointer ke objek `CWanted` |
| `CPed` | Struct total size | `0x5AC` | 1452 bytes |
| `CPlaceable` | CMatrix embed base | `+0x04` | RenderWare RwMatrix (44 bytes) |
| `CPlaceable` | Right vector | `+0x04`, `+0x08`, `+0x0C` | `cos(heading), sin(heading), 0.0` |
| `CPlaceable` | Forward vector | `+0x14`, `+0x18`, `+0x1C` | `-sin(heading), cos(heading), 0.0` |
| `CPlaceable` | Up vector | `+0x24`, `+0x28`, `+0x2C` | `0.0, 0.0, 1.0` |
| `CPlaceable` | Posisi X, Y, Z | `0x34`, `0x38`, `0x3C` | Koordinat 3D |
| `CVehicle` | Health (`m_fHealth`) | `0x204` | Single float (1000.0f) |
| `CVehicle` | Status flags | `0x53` | Status aktif driver & immunities |
| `CVehicle` | Model ID | `0x5E` / `0x5C` | ID model kendaraan |
| `CVehicle` | Damage Manager | `0x28C` | `CDamageManager` struct |
| `CPlayerInfo` | Struct total size | `0x13C` | 316 bytes |
| `CPlayerInfo` | Claude ped pointer | `+0x00` | `CPlayerPed*` |
| `CPlayerInfo` | Remote vehicle | `+0x04` | `CVehicle*` |
| `CPlayerInfo` | Uang (`m_nMoney`) | `+0xAC` | Integer saldo uang — dibuktikan dari `IFIWEREARICHMAN` (`0x000c0d90`) |
| `CPlayerInfo` | Tampilan uang HUD | `+0xB0` | Integer HUD display |
| `CPlayerInfo` | Infinite sprint | `+0x114` | `bool` flag lari tanpa lelah |
| `CPlayerInfo` | Fast reload | `+0x115` | `bool` flag reload cepat |
| `CPlayerInfo` | Bebas penjara | `+0x116` | `bool m_bGetOutOfJailFree` |
| `CPlayerInfo` | Bebas RS | `+0x117` | `bool m_bFreeHealthCare` |
| `CWanted` | Chaos Level | `+0x00` | `uint32 m_nChaosLevel` |
| `CWanted` | Wanted Stars | `+0x18` | `uint32 m_nWantedLevel` |
| `CClock` | `ms_nGameClockHours` | `0x002483B8` | `uint8` jam game saat ini (0..23) |
| `CClock` | `ms_nGameClockMinutes` | `0x002483BC` | `uint8` menit game saat ini (0..59) |
| `CClock` | `ms_nGameClockSeconds` | `0x002483C0` | `uint16` detik game saat ini |
| `CClock` | `ms_nLastClockTick` | `0x002483B4` | `uint32` waktu tick ms terakhir |

---

### D. Weapon & Model IDs (GTA III iOS)

| ID | Nama Senjata | Model ID (Streaming) | Slot |
|----|--------------|----------------------|------|
| 0 | Unarmed | - | 0 |
| 1 | Baseball Bat | **172** | Melee |
| 2 | Colt .45 | **173** | Pistol |
| 3 | Uzi | **178** | SMG |
| 4 | Shotgun | **176** | Shotgun |
| 5 | AK-47 | **171** | Rifle |
| 6 | M16 | **180** | Assault |
| 7 | Sniper Rifle | **177** | Sniper |
| 8 | Rocket Launcher | **175** | Heavy |
| 9 | FlameThrower | **181** | Heavy |
| 10 | Molotov Cocktail | **174** | Projectile |
| 11 | Grenade | **170** | Projectile |

---

### E. Vehicle Model IDs (GTA III iOS: 90 — 150)

- **Sport & Super**: Infernus (101), Cheetah (105), Banshee (119), Stinger (92), BF Injection (114)
- **Geng LCPD**: Mafia Sentinel (134), Yakuza Stinger (136), Yardie Lobo (135), Diablo Stallion (137), Cartel Cruiser (138), Hoods Rumpo XL (139)
- **Darurat & Militer**: Rhino Tank (122), Barracks OL (123), Police Car (116), Enforcer SWAT (117), FBI Kuruma (107), Ambulance (106), Firetruck (97), Securicar (118), Taxi (110), Cabbie (128), Borgnine Cabbie (148)
- **Sedan & Muscle**: Stretch Limousine (99), Sentinel (95), Kuruma (111), Stallion (129), Esperanto (109), Idaho (91), Manana (100), Perennial (94), Blista (102)
- **Van & Truk**: Landstalker (90), Patriot (96), Bobcat (112), Moonbeam (108), Rumpo (130), Pony (103), Mule (104), Yankee (146), Flatbed (145), Linerunner (93), Trashmaster (98), Bus (121), Coach (127), Mr. Whoopee (113), RC Bandit (131), Toyz (149), Belly Up (132), Mr. Wong's (133), Panlantic (144)
- **Kapal & Pesawat**: Dodo (126), Predator Police Boat (120), Speeder (142), Reefer (143), Ghost Boat (150)

---

### F. Perbaikan Fisika Spawner & Suspensi Kendaraan ("Jupiter Gravity / Squashed Roof")

1. **Penyebab Masalah Suspensi Terjepit / Ter-clamp**:
   - Rockstar `CAutomobile` membutuhkan 4 field suspensi & batas pergerakan pegas yang harus diinisialisasi setelah pembuatan objek (sama persis dengan script `CREATE_CAR` di `0x00045570` dan `CCheat::VehicleCheat` di `0x000C0A86`):
     ```rust
     *(veh.add(0x15e) as *mut u8) = 0;
     *(veh.add(0x15f) as *mut u8) = 0;
     *(veh.add(0x164) as *mut f32) = 20.0f32; // Batas panjang per suspensi (spring limit)
     *(veh.add(0x168) as *mut u8) = 20;
     ```
   - Tanpa `0x164` diisi `20.0f`, suspensi mobil bernilai 0 / garbage, menyebabkan seluruh bodi mobil jatuh menempel ke aspal seolah-olah ditarik gravitasi luar biasa berat ("planet jupiter") dan atapnya tampak terpotong/tergencet (*squashed*).

2. **Ketinggian Spawn Akurat Sesuai Kontur Tanah (Ground Z)**:
   - Sebelumnya mobil dijatuhkan dari `pz + 3.0f` di udara.
   - Diperbaiki menggunakan fungsi native Rockstar:
     - `CWorld::FindGroundZForCoord(spawn_x, spawn_y)` di `0x00038B48`.
     - `GetDistanceFromCentreOfMassToBaseOfModel(veh)` di `0x0002C970`.
     - `spawn_z = ground_z + height_from_base + 0.15f;` sehingga roda mobil langsung berdiri di atas permukaan tanah tanpa terjatuh membanting bodi.

3. **Orientasi Rotasi Mobil (Basis Orthonormal)**:
   - Menghindari pemanggilan fungsi yang mereset translasi koordinat ke (0,0,0).
   - Vektor `right`, `forward`, dan `up` langsung ditulis ke matriks `veh + 0x04` sesuai arah hadap Claude.

4. **Streaming Priority Kendaraan (Cheetah & Mobil Lainnya)**:
   - Sebelumnya `CStreaming::RequestModel(model_id, 0)` dipanggil dengan flag `0`, sehingga model non-prioritas seperti Cheetah (105) tidak langsung di-load saat `LoadAllRequestedModels` dijalankan dan berujung timeout (mobil tidak muncul, sehingga mobil sebelumnya tampak tidak terganti).
   - Diperbaiki dengan flag `1` (`STREAMING_GAME_REQUIRED`) agar engine GTA III memuat model seketika sebelum instansiasi.

---

### G. Perbaikan Crash Weapon Selector (GTA III Weapon Arsenal)

1. **Penyebab Crash (Misal: Paket 3 / Nutter Tools)**:
   - File `src/game/weapons.rs` sebelumnya masih mewarisi definisi senjata dari branch GTA Vice City dengan ID senjata hingga 36 (seperti Minigun ID 33, Chainsaw ID 11 VC, SPAS-12 ID 20 VC, MP5 ID 25 VC, Laser Sniper ID 29 VC).
   - Di GTA III, engine hanya mendukung 12 tipe senjata (ID 0 s/d 11).
   - Ketika `CPed::GiveWeapon` (`0x000de170`) dipanggil dengan weapon ID > 11, engine mengakses array internal `CWeaponInfo` secara *out-of-bounds*, menghasilkan SIGBUS / SIGSEGV (crash seketika).

2. **Perbaikan yang Diterapkan**:
   - **Restrukturisasi Senjata GTA III**: Menulis ulang `WEAPONS` di `src/game/weapons.rs` hanya dengan 11 senjata resmi GTA III:
     - *Melee*: Baseball Bat (ID 1)
     - *Pistols*: Colt .45 (ID 2)
     - *SMG*: Micro Uzi (ID 3)
     - *Shotguns*: Shotgun (ID 4)
     - *Assault*: AK-47 (ID 5), M16 (ID 6)
     - *Snipers*: Sniper Rifle (ID 7)
     - *Heavy*: Rocket Launcher (ID 8), FlameThrower (ID 9)
     - *Thrown*: Molotov Cocktail (ID 10), Grenade (ID 11)
   - **Restrukturisasi Paket Lengkap (Bundles)**:
     - **Paket 1 (Standard / Street)**: Bat (1), Colt .45 (2), Micro Uzi (3), Shotgun (4), Grenade (11)
     - **Paket 2 (Tactical / SWAT)**: Bat (1), Colt .45 (2), Micro Uzi (3), Shotgun (4), AK-47 (5), M16 (6), Sniper (7), Molotov (10)
     - **Paket 3 (Heavy / Mayhem)**: Semua 11 senjata lengkap (1 s/d 11) termasuk RPG & Flamethrower.
   - **Hardened Range Guard**: Menambahkan validasi `weapon_id >= 1 && weapon_id <= 11` di `player::queue_give_weapon` dan sebelum eksekusi `CPed::GiveWeapon` di `src/game/player.rs` untuk mencegah pemanggilan senjata tidak valid.

---

### H. Perbaikan Servis Mobil Instan (Instant Vehicle Repair ala Pay N Spray)

1. **Akar Masalah Kerusakan Tidak Diperbaiki (Bumper, Cap/Hood, Pintu, Mesin)**:
   - Kode sebelumnya membaca tipe kendaraan dengan mengecek offset `veh + 0x1F8`:
     ```rust
     let veh_type = *(veh.add(0x1f8) as *const u8) & 0x07;
     if veh_type == 0 {
         hook::slide_fn::<extern "C" fn(*mut u8)>(0x0005ce78)(veh);
     }
     ```
   - Namun di GTA III iOS ARMv7, offset `veh + 0x1F8` sebenarnya adalah field `m_nCreatedBy` (`1` untuk random traffic car, `2` untuk spawned/mission car), bukan tipe kendaraan!
   - Akibatnya nilai `veh_type == 0` tidak pernah terpenuhi (`false`), dan pemanggilan native `CAutomobile::Fix` (`0x0005CE78`) **selalu terlewat (skip)**!
   - Di GTA III, tipe kendaraan sebenarnya (`m_vehType`: `0` = Automobile, `1` = Boat, dll) terletak di offset `veh + 0x288` (terbukti dari analisis binary native Pay N Spray di `0x000F2436` dan `CCheat::HealthCheat` di `0x000C0C56`).

2. **Mekanisme Lengkap Servis ala Garasi Pay N Spray (Tanpa Ganti Warna)**:
   - **Pemulihan Health**: Nilai health kendaraan (`veh + 0x204`) diset ke `1000.0f`.
   - **Reset Timer Kerusakan / Kebakaran**: Offset `veh + 0x534` diset ke `0` (persis seperti di native Pay N Spray `0x000F245E`).
   - **Pemadaman Api Total**: Jika mobil sedang terbakar / mesin meledak, pointer api `m_pFire` di `veh + 0x1E8` dipadamkan menggunakan native `CFire::Extinguish` (`0x00068EF0`) dan pointer di-null-kan.
   - **Pemanggilan `CAutomobile::Fix` (`0x0005CE78`)**:
     1. Mereset seluruh damage manager di `veh + 0x28C` (`0x000CC700`), menghapus status kerusakan mesin, pintu, kap, bagasi, lampu, dan ban.
     2. Menghapus bit flag asap / terbakar `bIsDamaged` (`veh + 0x1FB &= !2`).
     3. Mengganti semua mesh atomik RenderWare yang rusak/penyok (`bonnet`, `bumper front/rear`, `boot`, `doors`) kembali ke mesh utuh mulus pabrik melalui callback `SetComponentAtomicFlagsCB(atomic, 2)`.
     4. Menyelaraskan kembali matriks transformasi seluruh engsel pintu, kap mesin, bagasi, dan bumper (nodes 7 s/d 20) ke posisi tertutup rapat dan lurus sempurna.
   - **Pembalikan Mobil Terbalik (Overturned Flip)**:
     - Jika mobil berada dalam posisi terbalik (`up.z < 0.0`), vektor matriks RenderWare langsung dibalik (`up` & `right` di-negasikan dan `UpdateRW` dipanggil) sehingga mobil seketika berdiri tegak di atas rodanya kembali persis logika Pay N Spray native (`0x000F2482..0x000F2528`).
   - **Proteksi Warna Asli**:
     - Warna primer (`veh + 0x1A0`) dan sekunder (`veh + 0x1A1`) sengaja tidak diubah sehingga warna kendaraan pemain tetap terjaga 100%.

---

### I. Perbaikan Cheat Amunisi Tak Terbatas (Infinite Ammo)

1. **Akar Masalah Amunisi Berkurang Saat Menembak**:
   - Kode sebelumnya hanya mengecek `if *total_ptr < 9000 { *total_ptr = 9999; }` pada offset `wep + 0x0C`.
   - Hal ini menyebabkan dua kelemahan fatal:
     1. **Peluru Berkurang Terlihat di HUD**: Total amunisi tidak di-refill sebelum mencapai angka 9000. Setiap peluru yang ditembakkan menyebabkan angka amunisi berkurang dari 9999 menjadi 9998, 9997, dan seterusnya.
     2. **Magazin / Clip Tetap Berkurang & Terpaksa Reload**: Pada engine GTA III, struct `CWeapon` (24 bytes) memiliki dua field penting:
        - `+0x04`: `m_eState` (Status senjata: 0 = READY, 1 = FIRING, 2 = RELOADING, 3 = OUT_OF_AMMO)
        - `+0x08`: `m_nAmmoInClip` (Jumlah peluru di dalam magazin aktif)
        - `+0x0C`: `m_nAmmoTotal` (Total cadangan peluru)
     - Logika lama sama sekali tidak menyentuh `m_nAmmoInClip`. Senjata dengan sistem magazin (Colt .45 kapasitas 12, Micro Uzi kapasitas 25, AK-47 kapasitas 30, M16 kapasitas 60) menampilkan HUD berformat `[magazin]-[cadangan]` (contoh: `12-9987`). Saat ditembakkan, angka magazin terus berkurang hingga 0 dan Claude terpaksa melakukan animasi reload (mengisi ulang peluru).

2. **Perbaikan yang Diterapkan**:
   - **Query Kapasitas Magazin Native**: Memanggil fungsi native `CWeaponInfo::GetWeaponInfo` (`0x0002C5BC(wep_type)`) untuk membaca field `nAmountofAmmonInClip` di offset `+0x10`. Jika tidak tersedia, sistem fallback ke kapasitas standar GTA III.
   - **Penguncian Magazin Penuh (`m_nAmmoInClip`)**: Setiap tick, nilai `*clip_ptr` dikunci ke kapasitas magazin maksimum (`clip_cap`) jika `*clip_ptr < clip_cap`. Dengan demikian, peluru magazin tidak pernah habis dan pemain dapat menembak terus-menerus tanpa terpotong animasi reload.
   - **Penguncian Total Peluru (`m_nAmmoTotal`)**: Setiap tick, jika `*total_ptr < 9999`, nilainya langsung di-lock kembali ke 9999.
   - **Pemulihan Status Senjata**: Jika state senjata sempat berada di `WEAPONSTATE_OUT_OF_AMMO` (`3`), statusnya otomatis dipulihkan ke `WEAPONSTATE_READY` (`0`).

---

### J. Fitur Baru: Cheat Cepetin Waktu Game 3 Jam (Advance Game Time)

1. **Latar Belakang & Mekanisme Game Clock GTA III iOS**:
   - Di GTA III, siklus waktu (siang/malam, cuaca, lampu jalan, lalu lintas, dan bayangan) dikendalikan oleh class `CClock`.
   - Melalui reverse engineering binary `gta3` (ARMv7), ditemukan struktur dan fungsi manajemen waktu engine:
     - Fungsi native `CClock::SetGameClock(uint8 hours, uint8 minutes)` berada di `0x000103CC`.
     - Fungsi native `CClock::GetGameClockMinutesUntil(uint8 hours, uint8 minutes)` berada di `0x00010418`.
     - Variabel global `ms_nGameClockHours` (`uint8`) berada di `0x002483B8`.
     - Variabel global `ms_nGameClockMinutes` (`uint8`) berada di `0x002483BC`.
     - Variabel global `ms_nGameClockSeconds` (`uint16`) berada di `0x002483C0`.
     - Variabel global `ms_nLastClockTick` (`uint32`) berada di `0x002483B4`.

2. **Implementasi Cheat "CEPETIN WAKTU (+3 JAM)"**:
   - Menambahkan varian aksi `PlayerAction::AdvanceTimeHours(u8)` pada `src/game/player.rs`.
   - Ketika dieksekusi di `player::tick()`:
     1. Membaca jam (`ms_nGameClockHours`) dan menit (`ms_nGameClockMinutes`) saat ini dari alamat memori native.
     2. Menghitung jam baru: `(current_hours + 3) % 24`.
     3. Memanggil fungsi native `CClock::SetGameClock(new_hours, current_mins)` (`0x000103CC`). Fungsi ini otomatis mereset detik ke 0 dan menyinkronkan `ms_nLastClockTick` dengan `CTimer::m_snTimeInMilliseconds` agar waktu terus mengalir mulus tanpa desinkronisasi engine.
   - Di `src/game/cheats.rs`:
     - Dibuat komponen interaktif `AdvanceTimeRow` yang membaca jam game saat ini dan menampilkannya langsung di menu cheat (contoh: `14:25 ▸ +3 JAM`).
     - Setelah diketuk, tombol memberikan konfirmasi visual instan (contoh: `17:25 (+3 JAM OK)` dengan tint warna hijau).
     - Tombol dapat diketuk berulang kali untuk terus memajukan waktu (tiap ketukan +3 jam: 14:00 -> 17:00 -> 20:00 -> 23:00 -> 02:00, dst).
     - Cheat ini terdaftar di kategori **SEMUA** (`CheatCategory::All`) dan kategori **CUACA & WAKTU** (`CheatCategory::WeatherTime`).

---

### K. Fitur Baru: Cheat Warp Forward (Tembus Rintangan & Pagar)

1. **Latar Belakang & Kebutuhan Fitur**:
   - Di GTA III, terdapat banyak area misi, markas geng, atau gerbang pulau (seperti gerbang Porter Tunnel, area bandara Francis International, dermaga Asuka, atau gerbang villa) yang terkunci oleh pembatas fisik atau pagar berduri.
   - Cheat "WARP FORWARD (TEMBUS DEPAN)" memungkinkan pemain melakukan teleportasi instan beberapa langkah lurus ke depan menembus barrier tanpa harus memanjat atau merusak pagar.
   - Cheat ini bekerja mulus baik saat **jalan kaki (on foot)** maupun saat **mengemudikan kendaraan (in vehicle)**.

2. **Daftar Alamat & Fungsi Native yang Digunakan**:
   - `find_player_ped()`: `0x000F4CC8` (Pointer Claude `CPed*`)
   - `find_player_vehicle()`: `0x000F4C14` (Pointer kendaraan `CVehicle*`)
   - `CWorld::FindGroundZForCoord(x, y)`: `0x00038B48` (Query ketinggian tanah via raycast vertikal)
   - `GetDistanceFromCentreOfMassToBaseOfModel(veh)`: `0x0002C970` (Query jarak pusat massa bodi kendaraan ke tanah)
   - `CEntity::PruneFromSectorList(entity)`: `0x0003A508` (Unregister entitas dari sektor spasial lama)
   - `CMatrix::UpdateRW(matrix)`: `0x0005ABE0` (Sinkronisasi transformasi matriks RenderWare)
   - `CEntity::UpdateRwFrame(entity)`: `0x0002CDD4` (Sinkronisasi frame hierarki model RenderWare)
   - `CWorld::Add(entity)`: `0x0003B090` (Register entitas ke sektor spasial baru)
   - `CCarCtrl::ClearAreaAroundVehicle(pos, veh)`: `0x000C5064` (Membersihkan objek & lalu lintas liar di titik pendaratan)

3. **Logika & Penanganan Kondisi Khusus (Edge Cases)**:
   - **Jarak Proporsional**:
     - *On foot*: `4.5 meter` (~5–6 langkah Claude), cukup lebar untuk menembus ketebalan pagar dan *collision box* pembatas.
     - *In vehicle*: `8.5 meter` untuk mengakomodasi panjang sasis mobil (4.5–5.5 meter) agar bumper depan dan belakang bersih melewati rintangan.
   - **Penyesuaian Medan Miring (Anti-Amblas ke Tanah)**:
     - Menghitung $\Delta Z = \text{ground}_{target} - \text{ground}_{current}$.
     - Jika tanjakan, ketinggian target dinaikkan sebanding dengan sudut miring bukit.
     - Ketinggian dikunci minimal $\text{ground}_{target} + 1.1\text{m}$ (pejalan kaki) atau $\text{ground}_{target} + \text{height}_{base} + 0.25\text{m}$ (kendaraan), sehingga kaki Claude atau roda mobil tidak amblas ke dalam tanah miring.
   - **Proteksi Terowongan & Flyover (Multi-Level / Underground)**:
     - Jika raycast tanah mengenai atap terowongan / jembatan di atas pemain (selisih ketinggian $> 5.0\text{m}$), raycast diabaikan dan sistem menggunakan pitch vektor arah hadap entitas ($pz + fz \times dist$).
   - **Proteksi Kendaraan Kapal / Perahu**:
     - Jika entitas adalah perahu (`bIsBoat` / tipe 1), koordinat $Z$ tidak diarahkan ke dasar laut melainkan tetap dikunci di permukaan air ($pz$).
   - **Proteksi Dinding / Gedung Tinggi**:
     - Lonjakan $\Delta Z > 3.5\text{m}$ dibatasi (clamp) agar pemain tidak terlempar ke genteng gedung bertingkat saat menatap dinding.
   - **Auto-Upright Mobil Terguling**:
     - Jika mobil sedang miring atau terbalik saat menabrak pagar ($up_z < 0.2$), rotasi mobil otomatis ditegakkan kembali berdiri di atas keempat roda.
   - **Netralisasi Pantulan Tabrakan & Angular Spin**:
     - Putaran liar (`m_vecTurnSpeed`) dinolkan dan arah laju (`m_vecMoveSpeed`) diselaraskan ke depan agar mobil tidak terpental mundur kembali ke pagar.
   - **Kompensasi Claude Terpental / Terjatuh**:
     - Hierarki RenderWare diperbarui seketika sehingga Claude dapat langsung berdiri di posisi baru tanpa glitch pose.

4. **Analisis & Perbaikan Crash Kendaraan (SIGSEGV di `0x000669D0`)**:
   - **Analisis Crash Dump IPS (`gta3-2026-10-04-152915.ips`)**:
     - Thread 12 mengalami `EXC_BAD_ACCESS (SIGSEGV)` pada alamat `0x001099D0` (unslid: `0x000669D0`) di dalam loop 4 roda `CAutomobile::ProcessControl()`.
     - Register `r0` bernilai `0x18`, dari instruksi:
       `0x669be: ldr r0, [r1, #0x44]`
       `0x669c0: add r0, r8` (r8 = 0x18)
       `0x669d0: vldr s0, [r0]` (membaca `[0x18]`)
   - **Akar Masalah Fundamental**:
     - Pada offset `veh + 0x470` terdapat 4 nilai float `fWheelsSuspensionCompression[4]` yang merepresentasikan rasio panjang pegas suspensi ke-4 roda.
     - Nilai `1.0f` berarti roda sedang di udara (suspensi terentang bebas).
     - Pada instruksi `0x66970..0x6697C`:
       `vcmpe.f32 s0, 1.0f; bpl #0x66a26;`
       Jika rasio suspensi $\ge 1.0\text{f}$ (di udara), engine **melompati (skip)** kalkulasi kompresi permukaan tanah pada `0x669D0`.
     - Namun jika bernilai `0.0f` (bottomed out), engine mengira roda menabrak poligon tanah dan mencoba membaca pointer poligon tanah pada `[r1 + 0x44]`. Karena mobil baru diteleport dan pointer poligon masih null/dangling, operasi membaca `[0 + 0x18]` memicu SIGSEGV seketika!
   - **Solusi yang Diterapkan**:
     1. Memanggil fungsi virtual native `vtable[12]` (`Teleport`) pada `0x00061DD8` (`CAutomobile::Teleport`) atau `0x00099D0C` (`CBoat::Teleport`) persis seperti native script opcode `SET_CAR_COORDINATES` (`0x00045A3C`). Fungsi ini secara otomatis menginisialisasi `fWheelsSuspensionCompression[0..4]` ke `1.0f` dan mereset pointer kontak suspensi.
     2. Menjaga nilai `veh + 0x470` tetap bernilai `1.0f32` (suspensi di udara) sehingga engine game melakukan raycast baru secara mulus di frame berikutnya.
     3. Untuk rotasi arah hadap mobil (`heading`), memanggil `CEntity::PruneFromSectorList` (`0x0003A508`), memutar matriks dengan `CMatrix::SetRotateZOnly` (`0x0005AAB0`), memperbarui clump RenderWare (`0x0005ABE0` & `0x0002CDD4`), dan mendaftarkan kembali ke sektor dunia dengan `CWorld::Add` (`0x0003B090`) persis seperti opcode native `SET_CAR_Z_ANGLE` (`0x000487D0`).
     4. Membersihkan area pendaratan mobil via `CCarCtrl::ClearAreaAroundVehicle` (`0x000C5064`).

5. **Peningkatan Engine Logging Real-time & Crash Handler**:
   - **Real-Time Disk Flushing**: Menambahkan `file.flush()` pada setiap pesan log di `src/logging.rs`, sehingga tidak ada baris log yang tertahan di memory buffer saat game crash.
   - **Native Signal Handler**: Menginstal handler untuk signal fatal (`SIGSEGV`, `SIGBUS`, `SIGABRT`, `SIGILL`, `SIGFPE`) yang langsung menulis diagnosis crash dan ASLR slide ke file `PANIC.txt` sebelum meneruskan crash ke crash reporter iOS.
   - **Verbose Step-by-Step Tracing**: Setiap fase warp (on foot maupun in vehicle) mencatat koordinat awal, koordinat tujuan, status elevasi, dan eksekusi virtual method secara mendetail di `cleo.log`.

6. **Verbose Logging Interaksi Menu, Cheat & Spawner**:
   - **Tab Cheats**: Setiap kali pemain menekan baris cheat, quick action, servis mobil, toggle status, atau tombol waktu di menu, log mencatat jenis cheat, kode, dan index yang dipilih. Saat cheat dieksekusi di `call_gta3_cheat`, nama lengkap cheat beserta efeknya dicatat ke `cleo.log`.
   - **Tab Kendaraan (Vehicle Spawner)**: Setiap pemilihan mobil, motor, tank, maupun kapal mencatat nama kendaraan, model ID, kategori, serta koordinat titik spawn di depan pemain.
   - **Tab Senjata (Weapon Arsenal)**: Setiap pemilihan paket senjata atau senjata satuan mencatat bundle ID atau weapon ID beserta jumlah amunisi yang diberikan.
   - **Tab Skrip (Scripts)**: Setiap eksekusi skrip CSI atau pergantian status skrip CSA dicatat secara eksplisit ke dalam log.

---

### L. Fitur Baru: Cheat Lompat Tinggi (Super Jump / Mega Jump)

1. **Latar Belakang & Kebutuhan Fitur**:
   - Di GTA III, kemampuan melompat Claude sangat terbatas (hanya setinggi ~1.0 s/d 1.2 meter dengan impuls vertikal standar $v_z \approx 0.12\text{f}$). Ketinggian ini tidak cukup untuk melompati pagar besi, kawat berduri, pintu gerbang pelabuhan, atau pembatas misi.
   - Terinspirasi dari cheat lompat tinggi legendaris GTA San Andreas (`cjphonehome` / `kangaroo`), cheat **"LOMPAT TINGGI (SUPER JUMP)"** memungkinkan Claude melompat 4-5 kali lebih tinggi (~4.5 s/d 5.5 meter) saat tombol loncat touchscreen ditekan.
   - Pemain dapat dengan mudah melompati pagar tinggi dan barrier yang mengunci area tertentu tanpa terjebak atau terhalang fisik collider.

2. **Analisis Reverse Engineering Binary GTA III iOS ARMv7**:
   - **State Mesin Animasi Karakter (`m_ePedState`)**:
     - Terletak pada offset **`ped + 0x228`** (`uint32`).
     - Saat pemain menekan tombol loncat di layar touchscreen (`es2/jump.png`), engine mengeksekusi `CPed::SetJump` di alamat **`0x000D71C0`**.
     - Fungsi ini mengatur `m_ePedState = 35` (`0x23` = `PEDSTATE_JUMP`) dan memicu animasi `ANIM_STD_JUMP_LAUNCH` (`148` / `0x94`).
   - **Vektor Kecepatan Fisika Entitas (`m_vecMoveSpeed` di `CPhysical`)**:
     - Dibuktikan langsung dari pembongkaran fungsi native `CPhysical::GetSpeed(CVector const &offset)` di **`0x00137A6C`**:
       - `m_vecMoveSpeed.x`: `+0x7C`
       - `m_vecMoveSpeed.y`: `+0x80`
       - `m_vecMoveSpeed.z`: `+0x84`
     - Kecepatan sudut putaran (`m_vecTurnSpeed`) berada di `+0x88`, `+0x8C`, `+0x90`.
     - Kecepatan gesekan (`m_vecMoveFriction`) berada di `+0x94`, `+0x98`, `+0x9C`.

3. **Implementasi & Penanganan Fisika Lompat**:
   - **Impuls Loncat Vertikal (Upward Impulse Boost)**:
     - Di dalam loop `player::tick()`, jika `SUPER_JUMP` aktif dan Claude terdeteksi memulai state `PEDSTATE_JUMP` (`35`):
       - Nilai vertikal `m_vecMoveSpeed.z` diatur ke **`0.36f`** (mencapai puncak lompatan sekitar 5 meter di udara).
       - Menggunakan *single-boost guard flag* (`JUMP_BOOSTED`) agar impuls hanya diberikan sekali pada frame awal inisiasi lompatan.
   - **Dorongan Momentum Maju (Forward Propulsion)**:
     - Jika Claude melompat sambil berlari atau berjalan maju ($v_x^2 + v_y^2 > 0.02$), kecepatan horizontal dikalikan faktor $1.35\times$ (dibatasi maksimal $0.30\text{f}$) sehingga Claude memiliki trayektori parabola yang anggun melewati ketebalan pagar tanpa terbentur di tepi atas.
   - **Proteksi Pendaratan Lembut (Soft Landing / Anti Fall Damage)**:
     - Di GTA III, pendaratan dengan kecepatan jatuh $v_z < -0.26\text{f}$ normalnya memicu *fall damage* (darah berkurang bahkan tewas seketika).
     - Saat Claude berada di udara dan mulai jatuh ($v_z < 0.0$), nilai $v_z$ dikunci (clamped) pada batas aman **`-0.22f`**.
     - Efek ini memberikan pendaratan *superhero glide* yang mulus dan nyaman, di mana Claude dapat melompat dari tempat tinggi tanpa kehilangan darah sedikit pun.

4. **Integrasi Menu Cheat**:
   - Ditambahkan ke tab **SEMUA** (`CheatCategory::All`) dan tab **KARAKTER & STATUS** (`CheatCategory::Player`) sebagai toggle `PlayerToggleRow` yang menampilkan teks hijau saat aktif dan memberikan toast pesan native GTA III.

---

### M. Fitur Baru: Cheat Hapus Batas Map (Remove Edge Map Barrier)

1. **Latar Belakang Masalah (Gaya Tolak Engine di Laut Lepas)**:
   - Ketika pemain menaiki perahu (Speeder, Reefer, Predator) atau menggunakan cheat warp melaju jauh ke arah laut lepas di tepi map (*edge of the map*), engine game tiba-tiba memberikan gaya tolak balik (*repulsive force*) yang memukul mundur perahu ke arah daratan.
   - Meskipun pemain menginjak pedal gas penuh atau men-trigger teleport maju, perahu tertahan dan terpelanting mundur oleh kalkulasi batasan dunia internal game.

2. **Analisis Reverse Engineering Binary GTA III iOS ARMv7**:
   - **Pada Perahu (`CBoat::ProcessControl`, `0x00099D9C`)**:
     - Pada rentang alamat **`0x0009B1C4` – `0x0009B24C`**, engine melakukan pemeriksaan koordinat posisi perahu terhadap 4 batas peta:
       - Batas Timur: `X > 1900.0f`
       - Batas Barat: `X < -1515.0f`
       - Batas Utara: `Y > 600.0f`
       - Batas Selatan: `Y < -1900.0f`
     - Jika posisi perahu melewati batas koordinat tersebut, engine mengeksekusi:
       ```cpp
       m_vecMoveSpeed.x = Min(m_vecMoveSpeed.x, -(GetPosition().x - 1900.0f) * 0.01f);
       m_vecMoveSpeed.x = Max(m_vecMoveSpeed.x, -(GetPosition().x - -1515.0f) * 0.01f);
       m_vecMoveSpeed.y = Min(m_vecMoveSpeed.y, -(GetPosition().y - 600.0f) * 0.01f);
       m_vecMoveSpeed.y = Max(m_vecMoveSpeed.y, -(GetPosition().y - -1900.0f) * 0.01f);
       ```
     - Rumus ini menghasilkan vektor kecepatan bernilai negatif secara agresif, memaksa perahu terlempar mundur setiap frame.
   - **Pada Mobil & Pesawat Dodo (`CAutomobile::ProcessControl`, `0x00068346` – `0x0006850A`)**:
     - Pada koordinat $|X| > 1900.0$ atau $|Y| > 1900.0$, engine membalikkan arah kecepatan ($v \times -1.0$) dan memutar haluan kendaraan 180 derajat ke belakang.

3. **Implementasi Patch Memori Dinamis (`patch_code_memory`)**:
   - Disediakan helper `crate::hook::patch_code_memory` yang menggunakan `MSHookMemory` dari Substrate (dengan fallback Darwin `vm_protect` + Copy-on-Write) untuk menembus proteksi `__TEXT` segment.
   - **Patch Perahu (`0x0009B1C4`)**:
     - Byte Asli: `[0x0A, 0x98, 0x1F, 0xED]` (`ldr r0, [sp, #0x28] ; vldr s0, [pc, #-0x268]`)
     - Byte Patch: `[0x12, 0x98, 0x42, 0xE0]` (`ldr r0, [sp, #0x48] ; b #0x9b24e`)
     - Mengisi parameter `onLand` pada `r0` secara valid dan melompati seluruh kalkulasi pembatasan kecepatan serta penulisan `vstr s6, [r4, #0x80]`. Kecepatan perahu tetap 100% utuh tanpa hambatan.
   - **Patch Mobil/Pesawat (`0x00068346`)**:
     - Byte Asli: `[0x9F, 0xED, 0xAE, 0x9A]` (`vldr s18, [pc, #0x2b8]`)
     - Byte Patch: `[0xE0, 0xE0, 0x00, 0xBF]` (`b #0x6850a ; nop`)
     - Melompati seluruh logika pembalikan arah dan rotasi paksa haluan.
   - **Proteksi Jangkar (`m_bIsAnchored` di `+0x1EC`)**:
     - Di dalam loop `tick()`, saat cheat aktif, jika perahu berada di perairan jauh, flag jangkar otomatis dibersihkan agar kapal tidak terkunci diam.

4. **Integrasi Menu Cheat**:
   - Kategori Menu: **LAIN-LAIN (`CheatCategory::Misc`)**, **SEMUA (`CheatCategory::All`)**, **SPAWN KENDARAAN (`CheatCategory::Vehicles`)**, dan **KARAKTER & STATUS (`CheatCategory::Player`)**.
   - Judul Cheat: `"REMOVE EDGE MAP BARRIER (BEBAS JELAJAH)"`.
   - Deskripsi: *"Disable pembatas & gaya tolak pinggiran map agar perahu/kendaraan bebas pergi ke lautan lepas tanpa terdorong mundur"*.

---

### N. Fitur Baru: Cheat Terbang Dodo Mudah & Stabil (Easy Dodo Flight)

1. **Latar Belakang & Masalah Fisika Dodo Bawaan GTA III**:
   - Pesawat Dodo (Model ID 126) di GTA III sangat terkenal sulit dikendalikan karena memiliki sayap yang terpotong (*clipped wings*).
   - Di engine game:
     - Ketika mengudara, roda terangkat dari aspal sehingga gaya dorong mesin mobil menghilang (tidak ada dorongan baling-baling / *propulsion* di udara).
     - Gaya angkat (*lift*) dihitung dari kuadrat kecepatan maju `fwdSpeed^2`. Begitu hidung pesawat dinaikkan sedikit saja, kecepatan maju turun drastis, gaya angkat seketika lenyap (*aerodynamic stall*), dan pesawat langsung terjun bebas menukik ke tanah/laut.
     - Di fungsi `CVehicle::FlyingControl`, terdapat batas ketinggian keras (*altitude ceiling*) pada koordinat $Z > 100\text{m}$. Jika pesawat mencapai tinggi di atas 100 meter, engine secara paksa memangkas gaya angkat menjadi $0.9 \times \text{GRAVITY}$, sehingga pemain tidak akan pernah bisa terbang tinggi melintasi gedung pencakar langit.
     - Dodo tidak memiliki stabilisasi guling (*roll auto-leveling*), sehingga sedikit saja menyenggol belokan di layar sentuh, sayap miring permanen dan pesawat berputar tak terkendali.

2. **Analisis Reverse Engineering Binary GTA III iOS ARMv7**:
   - **Fungsi Kendali Terbang (`CVehicle::FlyingControl`)**:
     - Terletak di alamat **`0x0013891C`**.
     - Pemanggilan dari `CAutomobile::ProcessControl` di alamat **`0x00067BA0`** dengan parameter `r1 = 0` (`FLIGHT_MODEL_DODO`).
   - **Bypass Batas Ketinggian 100 Meter (`0x00138BEA`)**:
     - Pada alamat **`0x00138BEA`**, instruksi asli adalah `ble #0x138bfc` (bytes `[0x07, 0xDD]`) yang memeriksa apakah $Z > 100.0\text{f}$.
     - Dengan melakukan patch menjadi branch tanpa syarat `b #0x138bfc` (bytes `[0x07, 0xE0]`), kalkulasi pemangkasan gaya angkat di atas 100 meter berhasil di-bypass 100%. Pesawat bebas membubung tinggi ke awan tanpa batas.

3. **Implementasi Model Aerodinamika Asistif (`process_dodo_easy_flight`)**:
   - Di dalam loop `tick()`, saat cheat aktif dan Claude menaiki pesawat Dodo (Model ID 126):
     - **Dorongan Mesin Baling-Baling (Continuous Air Propulsion)**:
       Menghasilkan daya dorong terarah sepanjang vektor depan `GetForward()`:
       - Saat pedal gas ditekan / menanjak: kecepatan maju dipercepat hingga $0.95\text{f} - 1.15\text{f}$ (~150-180 km/jam).
       - Saat lepas landas di landasan pacu (*runway*): Dodo langsung melesat dan terangkat ke udara hanya dalam 1-2 detik.
       - Saat rem ditekan: kecepatan melambat terkendali hingga $0.35\text{f}$ untuk memudahkan pendaratan mulus.
     - **Gaya Angkat & Pendakian Tanpa Batas (Lift & Unlimited Climbing)**:
       Merespon input kemudi atas/bawah touchscreen (`steer_ud` dari `CPad::GetSteeringUpDown` di `0x000BEFAC`):
       - Saat menarik hidung ke atas: memberikan impuls vertikal positif $+0.38\text{f}$ yang stabil dan kuat untuk mendaki setinggi apa pun melampaui gedung tertinggi Liberty City.
       - Mengimbangi gravitasi bumi saat terbang datar agar pesawat tidak ambles atau menukik sendiri.
     - **Stabilisasi & Auto-Leveling (Anti-Stall & Anti-Spin)**:
       - Ketika pemain melepas kontrol belok: sayap secara otomatis disejajarkan mendatar (*horizontal auto-leveling*) dengan meredam laju guling $w_y$ dan mengoreksi kemiringan sayap $r_z \to 0$.
       - Menghilangkan guncangan hidung liar (*pitch damping*) sehingga pesawat terbang stabil bagaikan autopilot.
       - Belok kiri/kanan menghasilkan kombinasi *yaw* dan *bank* yang proporsional dan responsif khas pesawat modern.

4. **Integrasi Menu Cheat**:
   - Kategori Menu: **LAIN-LAIN (`CheatCategory::Misc`)**, **SEMUA (`CheatCategory::All`)**, **SPAWN KENDARAAN (`CheatCategory::Vehicles`)**, dan **KARAKTER & STATUS (`CheatCategory::Player`)**.
   - Judul Cheat: `"TERBANG DODO MUDAH (EASY FLIGHT)"`.
   - Deskripsi: *"Dodo mudah lepas landas, terbang stabil & auto-leveling, bebas terbang tinggi tanpa batas"*.




