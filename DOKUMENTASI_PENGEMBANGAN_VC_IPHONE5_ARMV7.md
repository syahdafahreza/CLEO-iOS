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
| **`CCheat::DoCheats`** | `0x000778fc` | Eksekusi loop cheat game |
| **`CCheat::ResetCheats`** | `0x000779ac` | Reset flag & status cheat |
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

## 3. Langkah Selanjutnya

1. [x] Binary diekstrak dan dibedah secara komprehensif.
2. [x] Base segments `__TEXT` (`0x1000`) dan `__DATA` (`0x23A000`) terverifikasi.
3. [x] Seluruh alamat hook inti (script tick, opcode dispatcher, touch, GXT, cheat, player) berhasil dipetakan.
4. [ ] Perbarui `src/lib.rs` dan `src/targets` dengan alamat memori GTA Vice City ini.
5. [ ] Sesuaikan modul cheat `src/game/cheats.rs` untuk daftar cheat khas GTA Vice City.
6. [ ] Lakukan build pengujian `.deb` via GitHub Actions CI.
