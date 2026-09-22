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

