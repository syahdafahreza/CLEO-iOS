# Dokumentasi Pengembangan & Porting CLEO-iOS untuk GTA Vice City (iPhone 5 / ARMv7 32-bit)

**Target Game**: GTA Vice City iOS v1.3 (ARMv7 32-bit)  
**Perangkat Target**: iPhone 5 (iPhone5,2 / iOS 10.3.4)  
**Branch**: `For-iPhone-5-VC`  
**Bundle ID**: `com.rockstargames.gta3vc`  
**Executable**: `gta3vc`  

---

## 1. Spesifikasi Teknis & Lingkungan (Target Environment)

- **Arsitektur Binary**: Mach-O ARMv7 32-bit (thin binary).
- **Ukuran Binary**: 2,894,448 byte (~2.76 MB).
- **Sistem Operasi**: iOS 10.3.4 (Jailbroken, Rootful).
- **Base Memory Segments**:
  - `__PAGEZERO`: `0x00000000 - 0x00001000`
  - `__TEXT` base: `0x00001000` (ukuran: `0x239000`)
  - `__DATA` base: `0x0023A000` (ukuran: `0x3A3000`)
  - `__LINKEDIT`: `0x005DD000 - 0x00630000`
- **ABI & Tipe Data**:
  - `CGFloat` pada 32-bit adalah `f32`.
  - `usize` / `isize` berukuran 4 byte (32-bit).
  - Calling convention ARM Thumb-2: alamat fungsi internal wajib berakhiran bit 1 (`address | 1`).
- **Packaging & Cydia Substrate**:
  - Filter plist: `com.rockstargames.gta3vc` (executable: `gta3vc`).
  - Format paket: `.deb` dengan kompresi `gzip` (`data.tar.gz`).
  - Target dylib: `/Library/MobileSubstrate/DynamicLibraries/CLEO.dylib`.

---

## 2. Peta Alamat Memori GTA Vice City v1.3 (ARMv7 32-bit)

### A. Engine Script & CLEO SCM Runtime (`src/game/scripts/runtime.rs` & `src/lib.rs`)

| Simbol / Fungsi | Alamat Memori (v1.3 32-bit) | Keterangan & Signature |
| :--- | :--- | :--- |
| **`CTheScripts::Process` (`script_tick`)** | `0x00138ea8` | Loop tick utama eksekusi script GTA VC (`fn()`) |
| **`CRunningScript::Process`** | `0x00138978` | Tick per-script running instance (`fn(*mut CleoScript)`) |
| **`CRunningScript::ProcessOneCommand`** | `0x00138878` | Dispatcher opcode GTA VC (tempat intercept custom opcode CLEO `>= 0x0a8c`) |
| **`CollectParameters`** | `0x00132ff0` | Mengumpulkan argumen opcode dari script (`fn(*mut CleoScript, u32)`) |
| **`ReadParamValue`** | `0x001330d4` | Membaca 1 argumen dari script (`fn(*mut CleoScript) -> T`) |
| **`StoreParameters`** | `0x00133158` | Menyimpan nilai return ke variabel script (`fn(*mut CleoScript, u32)`) |
| **`ScriptSpace` (Global Space)** | `0x00549440` | Base pointer ruang script & variabel global game |
| **`pActiveScripts` (Script List)** | `0x00588e60` | Pointer linked-list script yang sedang berjalan |
| **`CTimer::m_snTimeInMilliseconds`** | `0x005d9fe4` | Waktu permainan global (`u32`) |

---

### B. Input & Touch Handling (`src/lib.rs`)

| Simbol / Method | Alamat Memori | Keterangan |
| :--- | :--- | :--- |
| **`EAGLView touchesBegan:withEvent:`** | `0x0020b79d` | Touch input mulai disentuh |
| **`EAGLView touchesMoved:withEvent:`** | `0x0020b6a5` | Touch input digeser (swipe down menu CLEO) |
| **`EAGLView touchesEnded:withEvent:`** | `0x0020b5ad` | Touch input dilepas |
| **Native Touch Processor** | `0x0020e100` | Fungsi internal pengolah touch gesture game |
| **Native Pad / Timer Polling** | `0x0020e37c` | Polling controller & sentuhan layar per-tick |

---

### C. Teks & GXT (`src/game/text.rs`)

| Simbol / Fungsi | Alamat Memori | Keterangan |
| :--- | :--- | :--- |
| **`CText::Get` (`get_gxt_string`)** | `0x001af6b8` | Lookup string GXT (`fn(usize, *const c_char) -> *const u16`) |

---

### D. Cheat System (`src/game/cheats.rs`)

| Komponen / Fungsi | Alamat Memori | Keterangan |
| :--- | :--- | :--- |
| **`CPad::UpdatePads`** | `0x000778fc` | Pembaruan buffer input pad/touch game per-frame (TIDAK BOLEH di-hard hook) |
| **`CPad::ResetCheats`** | `0x000779ac` | Reset status cheat bawaan |
| **`CCheat::WeaponCheat1` (Thugs)** | `0x000795b8` | Senjata set 1: Meminta model, memuat, dan memberikan paket senjata Thug |
| **`CCheat::WeaponCheat2` (Professionals)** | `0x0007946c` | Senjata set 2: Meminta model, memuat, dan memberikan paket senjata Professional |
| **`CCheat::WeaponCheat3` (Nutters)** | `0x00079314` | Senjata set 3: Meminta model, memuat, dan memberikan paket senjata Nutter |
| **`CCheat::HealthCheat` (ASPIRINE)** | `0x00079254` | Darah 100% dan servis mobil aktif |
| **`CCheat::ArmorCheat` (PRECIOUSPROTECTION)** | `0x00077e34` | Armor 100% |
| **`CCheat::WantedLevelUpCheat`** | `0x00077edc` | Menaikkan 2 bintang level kejaran polisi |
| **`CCheat::WantedLevelDownCheat`** | `0x00077e84` | Menghapus semua level kejaran polisi |
| **`CCheat::BlowUpCarsCheat` (BIGBANG)** | `0x00077dc4` | Meledakkan semua kendaraan di sekitar |
| **`CCheat::StrongGrip` (GRIPISEVERYTHING)** | `0x00077aa4` | Handling lengket / grip tinggi |
| **`CCheat::FlyingCars` (COMEFLYWITHME)** | `0x00077b08` | Mobil bisa terbang melayang |
| **`CCheat::WheelsOnly` (WHEELSAREALLINEED)** | `0x00077b6c` | Bodi mobil transparan / hanya roda kelihatan |
| **`CCheat::Seaways` (SEAWAYS)** | `0x0007978a` | Mobil bisa mengapung dan berjalan di atas air |
| **`CCheat::DriveBy` / Peds Armed** | `0x00077c84` | Pejalan kaki memegang senjata |

---

### E. Player & Game Loop

| Simbol / Fungsi | Alamat Memori | Keterangan |
| :--- | :--- | :--- |
| **`FindPlayerPed()`** | `0x000ea448` | Mendapatkan pointer ke Tommy Vercetti (`CPlayerPed*`) |
| **`FindPlayerVehicle()`** | `0x000ea394` | Mendapatkan pointer ke mobil pemain (`CVehicle*`) |
| **`CGame::Initialise`** | `0x00061688` | Inisialisasi awal engine game |
| **`drawFrame` (Render Loop)** | `0x0020f945` | Main render tick pada `IOSViewController` |
| **`CHud::SetHelpMessage`** | `0x00166edc` | Menampilkan teks bantuan / cheat message di layar |
| **`CHud::Draw` & FPS Counter** | `0x001662b0` | Fungsi render bawaan FPS counter dan HUD |

---

## 3. Catatan Penyelesaian Masalah "Stuck di Tap to Continue"

1. **`0x000778fc` (`do_cheats` hard target) mematikan input controller**:
   - `0x000778fc` adalah `CPad::UpdatePads()`, bukan fungsi cheat.
   - Hard hook di alamat ini menimpa eksekusi penyalinan buffer input layar sentuh ke pad aktif.
   - Solusi: Nonaktifkan hook `do_cheats` pada 32-bit; eksekusi antrean cheat CLEO dipindahkan langsung ke dalam `script_update` (`script_tick`).
2. **`uiscreen_size()` merusak memori `UIScreen` (`objc_msgSend_stret`)**:
   - Pemanggilan `[[UIScreen mainScreen] nativeBounds]` mengembalikan struct 16-byte `CGRect`. Pada 32-bit ARM ABI, method struct return wajib menggunakan `objc_msgSend_stret`. Penggunaan `msg_send!` standar tanpa stret menulis data float ke pointer `UIScreen`, merusak event loop UIKit.
   - Solusi: Hardcode resolusi layar `(1136.0, 640.0)` pada build 32-bit iPhone 5 tanpa query runtime.
3. **Konvensi Pemanggilan & Urutan Parameter `process_touch`**:
   - Menggunakan `extern "C"` dengan urutan register `(touch_type: u32, y_raw: u32, x_raw: u32, p3: u32, p4: u32)`.
   - Wajib memanggil `call_original!` terlebih dahulu agar game engine menerima input sentuhan tanpa terhambat.
4. **Guard `reset_before_start`**:
   - Hook `reset_before_start` dibatasi hanya untuk arsitektur 64-bit (`#[cfg(target_pointer_width = "64")]`).
5. **Perbaikan Rendering Menu (Kotak Putih / Teks Tertutup)**:
   - **Penyebab**:
     1. Fungsi `gui::colours::white_with_alpha` memanggil `+[UIColor colorWithWhite:alpha:]` melalui `objc_msgSend`. Pada ABI ARMv7 32-bit, argumen floating-point tidak terpetakan dengan benar sehingga menghasilkan warna solid putih opaque (`[UIColor whiteColor]`) bahkan saat diminta `alpha = 0.0` (transparan).
     2. Komponen `UILabel` (`value_label` dan `detail_label`) tidak memiliki background transparan eksplisit sehingga merender latar putih opaque menutupi baris menu.
     3. Pemanggilan `msg_send![self.button, frame]` dan `msg_send![self.scroll_view, frame]` pada arsitektur 32-bit ARM melanggar konvensi ABI `stret` (`CGRect` berukuran 16 byte memerlukan `objc_msgSend_stret`), yang menimpa memori pointer receiver.
   - **Solusi**:
     1. Ubah `white_with_alpha`: jika `alpha <= 0.001` langsung return `[UIColor clearColor]`; jika `white == 1.0` dan `alpha >= 0.9` return `[UIColor whiteColor]`; selebihnya dialihkan ke `+[UIColor colorWithRed:green:blue:alpha:]` yang terbukti bekerja dengan benar pada 32-bit.
     2. Set `setBackgroundColor: clearColor` dan `setOpaque: false` pada `button`, `value_label`, dan `detail_label`.
     3. Simpan `frame: CGRect` di `struct Row` dan `scroll_frame: CGRect` di `struct Tab` agar tidak perlu memanggil getter `frame` melalui Objective-C runtime.
6. **Perbaikan Pemetaan Fungsi & Indeks Cheat GTA Vice City v1.3 (ARMv7)**:
   - **Penyebab**:
     - Tabel alamat fungsi `VC_CHEAT_FUNCS` lama sebelumnya tertukar secara acak karena diasumsikan urutan cheat sama dengan SA/Android atau ditebak dari urutan alamat.
     - Contohnya, `0x00077dc4` (fungsi `BIGBANG` / Blow Up Cars) sebelumnya dipetakan ke index 8 (`ICANTTAKEITANYMORE`), sedangkan `0x0007877c` (fungsi ganti skin `STILLLIKEDRESSINGUP`) dipetakan ke index 20 (`BIGBANG`).
     - Alamat `0x00077b6c` (`WHEELSAREALLINEED` / Invisible Cars Wheels Only) sebelumnya dipetakan ke index 2 (`NUTTERTOOLS`).
     - Hal ini menyebabkan ketika memilih cheat "Mobil Meledak" (BIGBANG), fungsi yang terpanggil justru fungsi yang salah sehingga efek yang muncul adalah roda mobil saja atau hal lain.
   - **Solusi Awal**:
     - Melakukan reverse engineering terhadap dispatcher cheat asli GTA Vice City (`0x0007971c` s/d `0x0007a050`).
     - Mendekripsi algoritma enkripsi string cheat GTA VC (`0x00076120` dengan jump table TBB) dan memetakan cheat asli game ke fungsi targetnya.

7. **Investigasi Mendalam Spawn Rhino (Muncul Landstalker) & Audit Menyeluruh Cheat Vice City**:
   - **Penyebab Spawn Rhino Memunculkan Landstalker**:
     - Pada kode asli Rockstar, cheat `PANZER` di `0x00079b4e` memanggil fungsi internal `0x00079130`.
     - Hasil disassembly mendalam terhadap `0x00079130` membuktikan bahwa fungsi tersebut bukanlah fungsi pemuncul tank khusus, melainkan fungsi debug internal Rockstar (`TankCheat %d`) yang mengiterasi seluruh ID kendaraan mulai dari `130..239`.
     - Variabel static di `0x264cb0` diinisialisasi dengan angka `130` (yaitu ID model `landstal` / Landstalker di `default.ide`). Setiap kali fungsi `0x00079130` dipanggil, ia menaikkan index model sebesar +1. Akibatnya, pemanggilan pertama selalu memunculkan mobil Landstalker!
     - Model asli Rhino Tank di GTA Vice City adalah model `162` (`rhino`).
     - Fungsi generik spawner kendaraan yang benar adalah `0x00078920(model_id: u32)`. Memanggil `0x00078920(162)` secara langsung memunculkan Rhino Tank asli 100%!
   - **Penyebab Muncul Hunter Malah Jadi Limo & Masalah Mobil Balap**:
     - GTA Vice City tidak memiliki kode teks cheat asli untuk helikopter Hunter, namun model helikopter militer Hunter (`155`) ada di dalam file `default.ide`.
     - Sebelumnya `CheatOhDude` ("Munculin Hunter") secara keliru dipetakan ke `ROCKANDROLLCAR` (`0x00078920(201)`, Love Fist Limousine) sehingga men-spawn limousine.
     - Cheat `GETTHEREFAST` (`0x00078920(206)`, Sabre Turbo) sebelumnya dipetakan ke deskripsi `CheatNotForPublicRoads` ("Munculin Hotring Racer A"). Seharusnya Hotring Racer A adalah model `232` (`GETTHEREVERYFASTINDEED`), dan Hotring Racer B adalah model `233` (`GETTHEREAMAZINGLYFAST`).
   - **Penyebab Potensi Crash / Kerusakan Stack Frame pada Cheat Flag**:
     - Cheat seperti `LIFEISPASSINGMEBY`, `CHASESTAT`, `GREENLIGHT`, `MIAMITRAFFIC`, `AHAIRDRESSERSCAR`, dan `IWANTITPAINTEDBLACK` bukanlah fungsi standalone (tidak memiliki prolog fungsi `push {..., lr}`).
     - Alamat tersebut adalah snippet inline di dalam `CCheat::DoCheats` yang melakukan `ldr r0, [r0]` dan diakhiri dengan `pop {r4, r5, r7, pc}`. Memanggil alamat tersebut secara langsung merusak stack frame pemanggil (stack corruption).
     - **Solusi**: Diimplementasikan dengan mengubah/toggle variabel global flag game secara aman via memori pointer:
       - `GREENLIGHT` (Lampu lalu lintas hijau): `0x423b80`
       - `MIAMITRAFFIC` (Lalu lintas agresif): `0x5ad7a0`
       - `AHAIRDRESSERSCAR` (Mobil warna pink): `0x473398 = 1`, `0x473394 = 0`
       - `IWANTITPAINTEDBLACK` (Mobil warna hitam): `0x473394 = 1`, `0x473398 = 0`
       - `SEAWAYS` (Mobil bisa jalan di air): `0x5aaea0` dan `0x434644`
       - `LIFEISPASSINGMEBY` (Waktu cepat): `0x3dd1b0`
       - `CHASESTAT` (Media level debug): `0x5ce4ec`
   - **Daftar Lengkap 42 Cheat Terverifikasi 100%**:
     - **Weapons (3)**: `THUGSTOOLS` (1), `PROFESSIONALTOOLS` (2), `NUTTERTOOLS` (3)
     - **Health & Wanted (4)**: `PRECIOUSPROTECTION` (Armor 100%), `ASPIRINE` (Health 100% + Repair Mobil), `YOUWONTTAKEMEALIVE` (Wanted +2), `LEAVEMEALONE` (Wanted 0)
     - **Cuaca (5)**: `APLEASANTDAY` (Panas), `ALOVELYDAY` (Cerah), `ABITDRIEG` (Mendung), `CATSANDDOGS` (Hujan), `CANTSEEATHING` (Kabut)
     - **Kendaraan (11)**:
       - `PANZER` -> Model `162` (Rhino Tank)
       - `HUNTER` -> Model `155` (Hunter Military Helicopter)
       - `TRAVELINSTYLE` -> Model `234` (Bloodring Banger A)
       - `GETTHEREQUICKLY` -> Model `235` (Bloodring Banger B)
       - `GETTHEREVERYFASTINDEED` -> Model `232` (Hotring Racer A)
       - `GETTHEREAMAZINGLYFAST` -> Model `233` (Hotring Racer B)
       - `GETTHEREFAST` -> Model `206` (Sabre Turbo)
       - `THELASTRIDE` -> Model `172` (Romero Hearse)
       - `ROCKANDROLLCAR` -> Model `201` (Love Fist Limousine)
       - `RUBBISHCAR` -> Model `138` (Trashmaster)
       - `BETTERTHANWALKING` -> Model `187` (Caddy Golf Cart)
     - **Traffic & Physics (9)**: `BIGBANG` (Ledakkan semua mobil), `WHEELSAREALLINEED` (Mobil tembus pandang/roda saja), `COMEFLYWITHME` (Mobil terbang), `GRIPISEVERYTHING` (Handling mantap), `SEAWAYS` (Mobil jalan di air), `GREENLIGHT` (Lampu hijau), `MIAMITRAFFIC` (Lalu lintas agresif), `AHAIRDRESSERSCAR` (Lalu lintas pink), `IWANTITPAINTEDBLACK` (Lalu lintas hitam)
     - **Player & World (10)**: `LIFEISPASSINGMEBY` (Waktu cepat), `ONSPEED` (Gameplay cepat), `BOOOOOORING` (Slo-mo), `STILLLIKEDRESSINGUP` (Ganti baju/skin), `ICANTTAKEITANYMORE` (Bunuh diri), `FIGHTFIGHTFIGHT` (Kerusuhan), `NOBODYLIKESME` (Pejalan kaki serang Tommy), `OURGODGIVENRIGHTTOBEARARMS` (Pejalan kaki bersenjata), `CHICKSWITHGUNS` (Pejalan kaki wanita bersenjata), `CHASESTAT` (Media level)

8. **Perbaikan Spawner Kendaraan Tepat di Depan Player & Perbaikan Bug Efek Darah Karakter**:
   - **Penyebab Spawner Kendaraan Bawaan Sering Gagal / Hadap Melenceng**:
     - Fungsi cheat bawaan `0x00078920` (`CCheat::VehicleCheat`) memanggil `0x0004cae8` (`ThePaths.FindNodeClosestToCoors`). Jika player berada di area tanpa node jalan raya (pantai, atap mansion Vercetti, lapangan golf, rerumputan), game me-return kegagalan sehingga kendaraan **tidak pernah muncul**.
     - Ketika kendaraan muncul, arah hadapnya (`0x00075798`) diselaraskan dengan arah jalur jalan raya tersebut, bukan arah pandang/hadap Tommy.
   - **Solusi Native Spawner (Mirip VehicleSpawn.csi GTA SA)**:
     - Dibuat fungsi native `spawn_vehicle_direct(model_id: u32)` di `src/game/player.rs`:
       1. Request & synchronous stream model (`0x00099414` & `0x0009c55c`).
       2. Ambil forward facing vector Tommy dari CMatrix di `ped + 0x04` (`fx, fy`).
       3. Hitung spawn offset di depan Tommy ($X_{\text{spawn}} = P_x + \text{dir}_x \times D$, $Y_{\text{spawn}} = P_y + \text{dir}_y \times D$).
       4. Ambil elevasi tanah presisi via `CWorld::FindGroundZForCoord` (`0x0004909c`).
       5. Klasifikasi kendaraan presisi:
          - Motor roda 2 (`CBike`): model `166` (Angel), `178` (Pizza Boy), `191` (PCJ-600), `192` (Faggio), `193` (Freeway), `198` (Sanchez), atau helper `0x0019132c`. Alokasi ukuran `0x4ec` via `operator new` dan panggil konstruktor asli `CBike::CBike` (`0x00152710`).
          - Perahu (`CBoat`): model `136, 153, 176, 182, 183, 184, 200, 201, 212` atau helper `0x001912d4`. Alokasi `0x4c0` dan panggil `CBoat::CBoat` (`0x0005b5f0`).
          - Mobil (`CAutomobile`): alokasi `0x5dc` dan panggil `CAutomobile::CAutomobile` (`0x001d7620`).
       6. Orientasi Menyamping Menghadap Pintu Sopir:
          - Arah hadap kendaraan diatur menyamping 90 derajat via `veh_heading = heading + PI/2` menggunakan `CMatrix::SetRotate` (`0x00075798`).
          - Dengan sudut ini, sisi kiri kendaraan (pintu sopir pada mobil / posisi naik pada motor) menghadap **tepat ke arah Tommy**, sehingga Tommy langsung berada di depan kursi pengemudi.
       7. Inisialisasi Flag & Status:
          - Pada mobil (`CAutomobile`): buka kunci pintu `*(veh.add(0x230) as *mut u32) = 1;`.
          - Pada perahu (`CBoat`): set batas buoyancy air pada `veh + 0x15c` dan `veh + 0x160`.
          - Pada motor (`CBike`): tidak menyentuh offset pintu mobil `0x230`.
       8. Set status player (`0x52`), elevasi roda via `CVehicle::GetHeightAboveRoad` (`0x0015d0d4`), dan daftarkan ke dunia via `CWorld::Add` (`0x0004bc24`).
   - **Penyebab & Solusi Motor Melayang & Menimbulkan Efek Angin Helikopter (Downwash)**:
     - **Gejala Bug**: Motor berhasil di-spawn, tetapi posisinya melayang ~8 meter di udara dan menghasilkan efek partikel angin kencang baling-baling helikopter (seperti diliput helikopter berita VCN / helikopter polisi).
     - **Akar Masalah**:
       - Alamat `0x000f3294` / `0x000f30dc` dengan ukuran `0x360` dan helper `0x000f0c0c` ternyata adalah konstruktor **`CHeli::CHeli`** (helikopter), bukan motor! Akibatnya objek motor terdaftar sebagai entitas helikopter di engine Vice City, yang secara otomatis memicu mekanik melayang (*hover*) setinggi 8 meter di atas tanah dan mengeluarkan efek angin baling-baling helikopter (*downwash*).
       - Konstruktor asli motor **`CBike::CBike`** berada di alamat **`0x00152710`** (wrapper dari fungsi `0x00152320`) dengan ukuran objek sebesar **`0x4ec`** byte (berdampingan langsung dengan vtable `0x0026f378` dan method native `CBike::GetHeightAboveRoad` di `0x0015271c`).
     - **Solusi**:
       1. Ubah ukuran alokasi motor ke `0x4ec` byte dan panggil `CBike::CBike` di `0x00152710(veh, model_id, 1)`.
       2. Hapus seluruh pemanggilan `CHeli` (`0x000f0c0c`).
       3. Atur ketinggian elevasi `final_z = spawn_z + 0.25` sehingga roda motor mendarat tepat di aspal jalanan secara mulus dan normal tanpa efek angin helikopter.
   - **Penyebab & Solusi Bug Efek Darah Mengucur Terus-Menerus**:
     - **Penyebab**: Kode amunisi tak terbatas sebelumnya menulis `*(ped.add(0x14c) as *mut u32) |= 0x04000000;`. Pada GTA Vice City, bit `0x04000000` pada `0x14c` (yaitu bit ke-2 dari byte `0x14f`) adalah flag **`bIsBleeding`**! Akibatnya setiap kali toggle amunisi tak terbatas aktif atau memilih paket senjata, game mendeteksi Tommy pendarahan hebat sehingga efek partikel dan genangan darah terus keluar di bawah kaki Tommy.
     - **Solusi**:
       1. Hapus penulisan bit perusak ke `ped + 0x14c`.
       2. Clear flag pendarahan secara aktif di `tick()`: `*(ped.add(0x14f) as *mut u8) &= !4` dan `*(ped.add(0x51f) as *mut u8) = 0` (`m_nBleeding`).
       3. Implementasikan Infinite Ammo yang aman dan native dengan mengiterasi 10 slot senjata Tommy di `ped + 0x400 + slot * 0x18` dan menjaga total ammo serta clip tetap 9999.

9. **Cheat Anti Polisi / Bebas Polisi Permanen (Never Wanted)**:
   - **Mekanisme Native Game**:
     - Variabel global batas maksimal level bintang buronan di Vice City ARMv7 terletak di `0x0026a6d8` (`CWanted::MaximumWantedLevel: i32`, default: `6`).
     - Fungsi internal reset dan pembubaran kejaran polisi di Vice City terletak di `0x001f51d8(info: *mut u8, level: u32)`.
   - **Implementasi**:
     - Dibuat toggle `pub static NEVER_WANTED: AtomicBool = AtomicBool::new(false);`.
     - Ketika aktif (`true`):
       1. Mengunci `CWanted::MaximumWantedLevel` (`0x0026a6d8`) ke `0`. Semua laporan kejahatan (`RegisterCrime`) otomatis di-clamp ke `0` sehingga bintang buronan tidak pernah bisa naik.
       2. Jika player memiliki bintang buronan saat cheat diaktifkan, level buronan langsung dibersihkan ke `0` (`info + 0x20 = 0`) dan memanggil `0x001f51d8(info, 0)` untuk membubarkan polisi, helikopter, dan sirene yang sedang mengejar.
     - Ketika dinonaktifkan (`false`):
       - Mengembalikan `CWanted::MaximumWantedLevel` ke nilai normal `6`.

10. **Cheat Servis Kendaraan Instan (Instant Vehicle Repair) & Kendaraan Kebal (Vehicle God Mode)**:
    - **Servis Mobil Instan (`SERVIS MOBIL AKTIF (INSTAN)`)**:
      - **Validasi State**: Hanya dapat dieksekusi jika Tommy sedang berada di dalam kendaraan (`find_player_vehicle() != null`). Jika dipicu saat Tommy berjalan kaki (*on-foot*), menu secara otomatis menampilkan status teks merah `"HARUS NAIK MOBIL"`. Saat berhasil dipicu dalam mobil, menu menampilkan teks hijau `"BERHASIL"`.
      - **Native Engine Implementation**:
        1. Set health kendaraan ke `1000.0f` pada offset `veh + 0x204`.
        2. Panggil fungsi native game `CDamageManager::Reset` pada alamat `0x000c1cc0(veh + 0x2a0)` untuk mereset data kerusakan bodi dan ban.
        3. Deteksi kategori kendaraan menggunakan helper native: `CModelInfo::IsBike` (`0x0019132c`) dan `CModelInfo::IsBoat` (`0x001912d4`).
        4. Jika tipe mobil biasa/truk (`!is_bike && !is_boat`), panggil fungsi native `CAutomobile::Fix` pada alamat `0x001c6100(veh)`. Fungsi ini secara utuh memasang kembali semua pintu/kap/bagasi yang copot, membersihkan retakan kaca, dan mengembalikan bodi mobil ke kondisi 100% mulus dari pabrik.
        5. Jika motor atau perahu, reset status ban kempes pada offset `veh + 0x1fb` dan `veh + 0x1fd`.
    - **Kendaraan Kebal (`MOBIL KEBAL (VEHICLE GOD MODE)`)**:
      - Menggunakan toggle atomic `pub static GOD_MODE_VEHICLE: AtomicBool = AtomicBool::new(false);`.
      - Pada loop per-frame (`tick()`), jika cheat aktif dan Tommy sedang berada di dalam kendaraan (`find_player_vehicle() != null`):
        1. **Health Locking**: Nilai health kendaraan (`veh + 0x204`) dikunci pada `1000.0f`.
        2. **Total Damage Immunity**: Mengaktifkan bit-bit kekebalan pada `CEntity` flags (`veh + 0x52`):
           - `0x00010000`: `bBulletProof` (kebal tembakan peluru).
           - `0x00020000`: `bFireProof` (kebal semprotan api / molotov).
           - `0x00040000`: `bCollisionProof` (kebal penyok & damage saat menabrak gedung/kendaraan lain).
           - `0x00080000`: `bMeleeProof` (kebal serangan fisik/pentungan).
           - `0x00200000`: `bExplosionProof` (kebal ledakan granat, RPG, tank).
           - Mask gabungan: `*(veh.add(0x52) as *mut u32) |= 0x002f0000;`.
        3. **Ban Anti Bocor (*Tyres Don't Burst*)**: Mengaktifkan bit ke-1 pada byte flag ban: `*(veh.add(0x1fd) as *mut u8) |= 2;`. Ban tidak akan pernah meletus atau kempes meski ditembak sniper atau terkena spike.
        4. **Engine Status Protection**: Nilai status mesin di `veh + 0x2a4` dikunci ke `0` (intact), mencegah mobil terbakar atau mesin meledak saat terbalik.

11. **Penyelesaian Bug Cheat Senjata, Model Hilang (Tangan Kosong), dan Crash Saat Menembak**:
    - **Penyebab Utama**:
      1. **Crash & Tangan Kosong**: `CPed::GiveWeapon(0x00126cd4)` memberikan status kepemilikan senjata ke slot inventory Tommy, namun **tidak** memuat asset 3D model senjata ke streaming pool game. Saat pemain beralih (*cycle/scroll*) ke senjata tersebut, `CPed::SetCurrentWeapon(0x00126148)` memanggil `CWeaponModelInfo::CreateInstance()`. Karena model belum di-load oleh `CStreaming`, instance clump bernilai `NULL` (Tommy memegang senjata tak terlihat / tangan kosong). Ketika tombol tembak ditekan, `CWeapon::Fire` melakukan dereference pointer clump yang bernilai `NULL`, memicu `EXC_BAD_ACCESS` / Crash game seketika.
      2. **Weapon ID Tertukar**: Konfigurasi `weapon_id` pada menu CLEO sebelumnya tidak sesuai dengan enum `eWeaponType` dan data `weapon.dat` GTA Vice City. Contoh: RPG salah di-set ke ID `28` (Sniper Rifle), Minigun ke ID `30` (RPG), Katana ke ID `12` (Grenade), dsb.
    - **Solusi & Implementasi**:
      1. **Model Streaming Pre-loading**: Dibuat pemetaan model ID native (`get_weapon_models`) di `src/game/player.rs`. Setiap kali senjata diberikan (`tick()` step 3):
         - Model diminta via `CStreaming::RequestModel(model_id, 1)` pada alamat `0x00099414`. (Khusus senjata seperti Minigun dimuatkan model bodi `290` dan laras putar `294`, Detonator Grenade dimuatkan `270` dan `291`).
         - Engine dipaksa memuat model secara sinkron via `CStreaming::LoadAllRequestedModels(0)` pada alamat `0x0009c55c`.
         - Senjata diberikan ke Tommy via `CPed::GiveWeapon(ped, wid, ammo, 1)` pada alamat `0x00126cd4`.
         - Model ditandai dapat dibebaskan memory manager via `CStreaming::SetModelIsDeletable(model_id)` pada alamat `0x0009ab9c` (persis seperti pola fungsi cheat native `0x000795b8`).
      2. **Normalisasi ID Senjata & Bundles**: Seluruh `weapon_id` di `src/game/weapons.rs` dinormalisasi sesuai enum internal Vice City:
         - Melee: Brass Knuckles (`1`), Screwdriver (`2`), Golf Club (`3`), Nightstick (`4`), Knife (`5`), Bat (`6`), Hammer (`7`), Cleaver (`8`), Machete (`9`), Katana (`10`), Chainsaw (`11`).
         - Pistols: Colt .45 (`17`), .357 Python (`18`).
         - Shotguns: Chrome Shotgun (`19`), SPAS-12 (`20`), Stubby (`21`).
         - SMG: Tec-9 (`22`), Uzi (`23`), Mac-10 (`24`), MP5 (`25`).
         - Assault: M4 (`26`), Kruger (`27`).
         - Snipers: Sniper Rifle (`28`), PSG-1 (`29`).
         - Heavy: Rocket Launcher / RPG (`30`), Flamethrower (`31`), M60 (`32`), Minigun (`33`).
         - Thrown: Grenade (`12`), Remote Grenade (`13`), Tear Gas (`14`), Molotov (`15`).
         - Bundles: Set 1 Thugs (`[1, 6, 15, 17, 19, 22, 27, 28, 31]`), Set 2 Professionals (`[10, 13, 18, 21, 24, 26, 29, 30]`), Set 3 Nutters (`[11, 12, 18, 20, 25, 26, 29, 33]`).

