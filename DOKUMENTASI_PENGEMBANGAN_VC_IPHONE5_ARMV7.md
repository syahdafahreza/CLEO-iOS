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
| **Weapons Cheat 1** | `0x00077aa4` | Senjata set 1 |
| **Weapons Cheat 2** | `0x00077b08` | Senjata set 2 |
| **Weapons Cheat 3** | `0x00077b6c` | Senjata set 3 |
| **Spawn Panzer (Tank)** | `0x00079254` | Memunculkan Tank Rhino/Panzer |
| **Spawn Sabre Turbo** | `0x00079314` | Memunculkan mobil Sabre Turbo |
| **Spawn Bloodring Banger** | `0x0007946c` | Memunculkan mobil Bloodring Banger |
| **Spawn Caddy** | `0x000795b8` | Memunculkan mobil Golf Caddy |

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
   - **Solusi**:
     - Melakukan reverse engineering penuh terhadap fungsi dispatcher cheat asli GTA Vice City (`0x0007971c` s/d `0x0007a050`).
     - Mendekripsi algoritma enkripsi string cheat GTA VC (`0x00076120` dengan jump table TBB) dan memetakan ke-37 cheat asli game secara akurat ke fungsi targetnya:
       - `BIGBANG` (Blow Up Cars): `0x00077dc4`
       - `ASPIRINE` (Health & Car Repair): `0x00079254` (parameter `r0 = 1`)
       - `PRECIOUSPROTECTION` (Armor): `0x00077e34`
       - `THUGSTOOLS` (Weapon 1): `0x000795b8`
       - `PROFESSIONALTOOLS` (Weapon 2): `0x0007946c`
       - `NUTTERTOOLS` (Weapon 3): `0x00079314`
       - `PANZER` (Rhino Tank): `0x00079130`
       - `WHEELSAREALLINEED` (Wheels Only): `0x00077b6c`
       - Kendaraan (`TRAVELINSTYLE`, `THELASTRIDE`, `ROCKANDROLLCAR`, `RUBBISHCAR`, `GETTHEREFAST`, `BETTERTHANWALKING`): dipanggil via fungsi generik spawner `0x00078920(model_id)`.
     - Jumlah cheat VC diupdate menjadi 37 cheat terverifikasi penuh.
