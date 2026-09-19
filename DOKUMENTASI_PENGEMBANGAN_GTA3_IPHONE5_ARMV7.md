# Dokumentasi Pengembangan CLEO untuk GTA III — iPhone 5 (ARMv7, iOS 10.x)

## Informasi Branch

| Parameter | Nilai |
|-----------|-------|
| **Branch Git** | `For-iPhone-5-GTA3` |
| **Dibuat dari** | `For-iPhone-5-VC` |
| **Target game** | Grand Theft Auto III (GTA 3) |
| **Versi game** | v1.x iOS (32-bit) |
| **Perangkat** | iPhone 5 |
| **iOS** | 10.x |
| **Arsitektur CPU** | ARMv7 (armv7s-apple-ios target) |
| **Jailbreak** | Rootful |

---

## Status Pengembangan

> [!CAUTION]
> **Semua memory address di bawah ini adalah PLACEHOLDER (0x00000000)!**
> Harus diisi setelah analisis binary IPA GTA 3 selesai dilakukan.

---

## Cara Mendapatkan Addresses dari IPA

### Langkah 1 — Extract IPA
```bash
# Rename .ipa ke .zip, lalu extract
cp GTA3.ipa GTA3.zip
unzip GTA3.zip -d GTA3_extracted
# Binary ada di: GTA3_extracted/Payload/GTA3.app/GTA3
```

### Langkah 2 — Cek TEXT segment base
```bash
otool -l GTA3_extracted/Payload/GTA3.app/GTA3 | grep -A 10 "__TEXT"
# Cari vmaddr — biasanya 0x00001000 untuk app iOS 32-bit
```

### Langkah 3 — Analisis fungsi dengan disassembler
Tool yang direkomendasikan:
- **IDA Pro** (free untuk ARM 32-bit) atau **Ghidra** (gratis)
- **Hopper Disassembler** (macOS)
- `otool -tV GTA3` untuk text section dump

### Langkah 4 — Cari fungsi via strings/xref
Cari string yang diketahui ada di GTA 3, lalu trace xref ke fungsi terkait.

---

## Tabel Placeholder Addresses yang Perlu Diisi

### A. Hooks Utama (`src/lib.rs`)

| Nama Target | Deskripsi | Placeholder | Address GTA 3 |
|-------------|-----------|-------------|---------------|
| `script_tick` | CTheScripts::Process — dipanggil setiap frame | `0x00000000` | **TODO** |
| `process_touch` | Handler input layar sentuh | `0x00000000` | **TODO** |
| `get_gxt_string` | CText::Get — lookup teks GXT | `0x00000000` | **TODO** |
| `legal_splash` | Hook splash screen (init awal CLEO) | `0x00000000` | **TODO** |
| `legal_splash_german` | Versi German splash screen | `0x00000000` | **TODO** |
| `store_crash_fix` | Mencegah crash App Store check | `0x00000000` | **TODO** |
| `gen_plate` | CPlate::GeneratePlate | `0x00000000` | **TODO** |
| `do_game_state` | CGame::Process / state machine | `0x00000000` | **TODO** |
| `end_dragging` | UI gesture handler | `0x00000000` | **TODO** |
| `loading_messages` | Hook pesan loading screen | `0x00000000` | **TODO** |
| `height_above_ceiling` | Deteksi ground untuk spawn kendaraan | `0x00000000` | **TODO** |
| `init_for_title` | Init menu utama | `0x00000000` | **TODO** |
| `load_settings` | Load pengaturan pemain | `0x00000000` | **TODO** |

### B. Fungsi Player (`src/game/player.rs`)

| Nama Fungsi | Deskripsi | Placeholder | Address GTA 3 |
|-------------|-----------|-------------|---------------|
| `find_player_ped()` | Pointer ke Claude (CPed*) | `0x00000000` | **TODO** |
| `find_player_vehicle()` | Pointer ke kendaraan saat ini | `0x00000000` | **TODO** |
| `CWorld::Players` | Base address array CPlayerInfo | `0x00000000` | **TODO** |
| `g_nCurrentPlayerFocusIndex` | Index player aktif | `0x00000000` | **TODO** |
| CStreaming::RequestModel | Load model dari CD | `0x00000000` | **TODO** |
| CStreaming::LoadAllRequestedModels | Load semua model yang diminta | `0x00000000` | **TODO** |
| CStreaming::SetModelIsDeletable | Tandai model bisa dihapus | `0x00000000` | **TODO** |
| CWorld::FindGroundZForCoord | Cari ketinggian tanah | `0x00000000` | **TODO** |
| operator new | Alokasi memori C++ | `0x00000000` | **TODO** |
| CAutomobile::CAutomobile | Konstruktor mobil | `0x00000000` | **TODO** |
| CBike::CBike | Konstruktor motor | `0x00000000` | **TODO** |
| CBoat::CBoat | Konstruktor kapal | `0x00000000` | **TODO** |
| CMatrix::SetRotate | Set rotasi matrix | `0x00000000` | **TODO** |
| CWorld::Add (entity) | Tambah entitas ke dunia | `0x00000000` | **TODO** |
| CPed::GiveWeapon | Beri senjata ke player | `0x00000000` | **TODO** |
| CWanted::SetWantedLevel | Set level buronan | `0x00000000` | **TODO** |
| FullHealth helper | Beri full HP | `0x00000000` | **TODO** |
| FullArmor helper | Beri full armor | `0x00000000` | **TODO** |
| ClearWanted helper | Hapus wanted level | `0x00000000` | **TODO** |
| CAutomobile::Fix | Perbaiki kendaraan | `0x00000000` | **TODO** |
| Suspension height fn | Tinggi suspensi kendaraan | `0x00000000` | **TODO** |

### C. Struct Offsets yang Perlu Diverifikasi

| Struct | Field | Offset VC (referensi) | Offset GTA 3 |
|--------|-------|-----------------------|--------------|
| `CPed` | Health | `0x34c` | **TODO** |
| `CPed` | Armor | `0x350` | **TODO** |
| `CPed` | CWanted* | `0x5f0` | **TODO** |
| `CPed` | bIsBleeding bit | `0x14f` | **TODO** |
| `CPed` | Weapon slots base | `0x400` | **TODO** |
| `CPed` | Weapon slot stride | `0x18` | **TODO** |
| `CWeapon` (dalam slot) | Clip ammo offset | `+0x08` | **TODO** |
| `CWeapon` (dalam slot) | Total ammo offset | `+0x0c` | **TODO** |
| `CPlaceable` | Position X | `0x34` | **TODO** |
| `CPlaceable` | Position Y | `0x38` | **TODO** |
| `CPlaceable` | Position Z | `0x3c` | **TODO** |
| `CPlaceable` | Forward X | `0x14` | **TODO** |
| `CPlaceable` | Forward Y | `0x18` | **TODO** |
| `CVehicle` | Health | `0x204` | **TODO** |
| `CVehicle` | Status flags | `0x52` | **TODO** |
| `CVehicle` | Model ID | `0x5c` | **TODO** |
| `CVehicle` | Door lock (auto) | `0x230` | **TODO** |
| `CVehicle` | CBike worn flags | `0x1fb`, `0x1fd` | **TODO** |
| `CVehicle` | Engine state (auto) | `0x2a4` | **TODO** |
| `CPlayerInfo` | Money | `+0xa0` (VC) | **TODO** |
| `CPlayerInfo` | Display money | `+0xa4` (VC) | **TODO** |
| `CPlayerInfo` | Infinite sprint flag | `+0x140` (VC) | **TODO** |
| `CPlayerInfo` | Fast reload flag | `+0x141` (VC) | **TODO** |
| `CPlayerInfo` | Struct size | `0x174` (VC) | **TODO** |
| `CWanted` | m_nChaosLevel | `+0x00` | **TODO** |
| `CWanted` | m_nWantedLevel | `+0x20` (VC) | **TODO** |
| Global | MaximumWantedLevel | `0x0026a6d8` (VC) | **TODO** |
| Global | MaximumChaosLevel | `0x0026a6dc` (VC) | **TODO** |

---

## GTA III vs GTA Vice City — Perbedaan Penting

| Aspek | GTA VC (referensi) | GTA III |
|-------|-------------------|---------|
| Player character | Tommy Vercetti | Claude |
| Weapon slots di CPed | 10 slot | 12 slot (perlu konfirmasi) |
| Jumlah senjata | ~36 jenis | ~17 jenis |
| Map | Vice City | Liberty City |
| CPlayerInfo size | 0x174 (372 bytes) | Belum diketahui |
| CWanted offset di CPed | 0x5f0 | Belum diketahui |

---

## Weapon IDs GTA III

| ID | Nama | Catatan |
|----|------|---------|
| 0 | Fist/Unarmed | - |
| 1 | Brass Knuckles | - |
| 2 | Screwdriver | - |
| 3 | Golf Club | - |
| 4 | Nightstick | - |
| 5 | Knife | - |
| 6 | Baseball Bat | - |
| 7 | Grenade | - |
| 8 | Molotov Cocktail | - |
| 9 | Rocket | - |
| 10 | Colt .45 | Pistol |
| 11 | Uzi | - |
| 12 | Shotgun | - |
| 13 | M16 | - |
| 14 | Sniper Rifle | - |
| 15 | Rocket Launcher | RPG |
| 16 | Flamethrower | - |

> [!NOTE]
> Weapon IDs di atas berdasarkan data GTA III umum. Perlu diverifikasi dengan binary IPA.
> Model IDs untuk streaming juga perlu dikonfirmasi.

---

## Checklist Setelah IPA Tersedia

- [ ] Extract IPA dan dapatkan binary GTA3
- [ ] Tentukan TEXT segment base address (`otool -l`)  
- [ ] Cari `CTheScripts::Process` → isi `script_tick`
- [ ] Cari `CText::Get` → isi `get_gxt_string`
- [ ] Cari `FindPlayerPed` / `CWorld::FindPlayerPed`
- [ ] Cari `FindPlayerVehicle`
- [ ] Cari `CWorld::Players` (global array)
- [ ] Cari `CStreaming::RequestModel`
- [ ] Cari `CStreaming::LoadAllRequestedModels`
- [ ] Cari `CWorld::FindGroundZForCoord`
- [ ] Cari konstruktor kendaraan (CAutomobile, CBike, CBoat)
- [ ] Cari `CPed::GiveWeapon`
- [ ] Cari `CWanted::SetWantedLevel`
- [ ] Verifikasi semua struct offsets di tabel di atas
- [ ] Build dan test di device iPhone 5 iOS 10.x

---

## Build Command

```bash
# Build untuk target ARMv7 (iPhone 5)
cargo build --target armv7s-apple-ios

# Build release
cargo build --target armv7s-apple-ios --release
```

> [!WARNING]
> Build dengan placeholder addresses akan COMPILE tapi TIDAK akan berfungsi di device!
> Isi semua `0x00000000` dengan address yang benar terlebih dahulu.

---

## Referensi

- Branch VC: `For-iPhone-5-VC` (referensi untuk porting)
- Dokumentasi VC: `DOKUMENTASI_PENGEMBANGAN_VC_IPHONE5_ARMV7.md`
- hook.rs: `src/hook.rs` — mekanisme hook sama persis dengan VC
- build.py: `build.py` — build script sama
