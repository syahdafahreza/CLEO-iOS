# Dokumentasi Pengembangan & Porting CLEO-iOS untuk iPhone 5 (ARMv7 / 32-bit)
**Target**: GTA San Andreas iOS v1.09 (ARMv7 32-bit)  
**Perangkat Target**: iPhone 5 (iPhone5,2 / iOS 10.3.4)  
**Branch**: `For-iPhone-5`

---

## 1. Spesifikasi Teknis & Lingkungan (Target Environment)

- **Arsitektur**: ARMv7 / ARMv7s (32-bit).
- **Sistem Operasi**: iOS 10.3.4 (Jailbroken).
- **Game Binary**: GTA San Andreas v1.09 32-bit.
- **Base Memory Segment**:
  - `TEXT` segment base: `0x00004000`
  - `DATA` segment base: `0x004A0000`
- **ABI & Tipe Data Penting**:
  - `CGFloat` pada 32-bit adalah `f32` (bukan `f64`).
  - `usize` / `isize` berukuran 4 byte (32-bit).
  - Calling convention fungsi ARMv7 (Thumb-2 mode membutuhkan bit paling belakang bernilai 1 / `address | 1`).
- **Packaging & Cydia Substrate**:
  - Format paket `.deb` wajib menggunakan kompresi `gzip` (`data.tar.gz`), karena `dpkg` di iOS 10 belum mendukung kompresi `xz`.
  - Filter plist Cydia Substrate wajib mendaftarkan bundle id: `com.rockstargames.gta3sa`.

---

## 2. Daftar Alamat Memori yang Valid (GTA SA v1.09 32-bit)

Seluruh alamat berikut adalah offset dari `TEXT` / `DATA` (ditambahkan ASLR slide saat runtime via `hook::slide` atau `hook::slide_fn`):

### A. Target Hooks (`src/lib.rs`)

| Nama Target / Hook | Alamat 32-bit (v1.09) | Alamat 64-bit (v2.0+) | Keterangan & Signature |
| :--- | :--- | :--- | :--- |
| `script_tick` | `0x0011e6f4` | `0x1001d0f40` | Tick engine script GTA (`fn()`) |
| `process_touch` | `0x003ece38` | `0x1004e831c` | Input gesture layar sentuh (`fn(u32, f32, f32, f64)`) |
| `get_gxt_string` | `0x00356380` | `0x10044142c` | Lookup string teks GXT (`fn(usize, *const c_char) -> *const u16`) |
| `legal_splash` | `0x000b3e6c` | - | Hook legal splash screen awal |
| `legal_splash_german` | `0x000a5d20` | - | Hook legal splash screen Jerman |
| `store_crash_fix` | `0x00007ad0` | - | Mencegah crash store |
| `button_hack` | `0x00017338` | `0x1004ea8c4` | Hook touch target Reachability tombol menu CLEO |
| `gen_plate` | `0x002d3498` | - | Generator plat nomor |
| `idle` | `0x00189f94` | `0x100242c20` | `CGame::Idle` loop game per frame (`fn(usize, usize)`) |
| `cycles_per_millisecond` | `0x003ed694` | `0x10026e790` | Timer kalkulasi game cycle |
| `do_game_state` | `0x003c9d10` | - | State processing |
| `do_cheats` | `0x000f6bdc` | `0x1001a7f28` | `CCheat::DoCheats` tempat eksekusi cheat game |
| `reset_cheats` | `0x000f61b0` | `0x1001a82f0` | `CCheat::ResetCheats` reset status cheat |
| `reset_before_start` | `0x00253f6c` | - | Game reset handler |
| `find_absolute_path` | `0x003f0ea4` | - | Resolusi path file game |
| `init_for_title` | `0x002a8114` | - | *Dinonaktifkan di 32-bit* (mencegah early crash) |
| `load_settings` | `0x002542ec` | - | Pengaturan game |
| `display_fps` | `0x00188fd0` | `0x100241cd8` | Fungsi asli game untuk render teks FPS |
| `update_pads` | `0x001e2b48` | - | Input polling controller / tombol |
| `load_cd_directory` | `0x00265550` | - | Pembacaan direktori archive CD/IMG |
| `end_dragging` | `0x000ad2e0` | - | Touch drag ending |
| `loading_messages` | `0x002381cc` | - | Pesan loading screen |
| `height_above_ceiling` | `0x003b2a74` | - | Kalkulasi ketinggian |

---

### B. Cheat System Addresses (`src/game/cheats.rs`)

| Komponen Cheat | Alamat Memori | Keterangan |
| :--- | :--- | :--- |
| **Cheat Function Table** | `0x004f6cf4 + (index * 4)` | Pointer ke array function cheat di GTA SA 32-bit |
| **Cheat Active Flags** | `0x005bcde4 + index` | Array byte boolean status aktif tiap cheat (1 byte per cheat) |
| **Total Cheat 32-bit** | **109 Cheats** | Berbeda dengan versi 64-bit yang memiliki 111 cheat |

---

### C. CLEO Script Runtime & Opcodes (`src/game/scripts/runtime.rs`)

| Komponen Script SCM | Alamat Memori | Keterangan |
| :--- | :--- | :--- |
| **Opcode Handler Table** | `0x004a3494` | Tabel pointer fungsi opcode SCM bawaan |
| **Default Opcode Handler** | `0x0015616c` | Menangani semua opcode `>= 0x0a8c` |
| **Collect Parameters** | `0x0011d2a4` | `hook::slide_fn::<fn(*mut CleoScript, u32)>(0x0011d2a4)` |
| **Read Param Value** | `0x0011d690` | `hook::slide_fn::<fn(*mut CleoScript) -> T>(0x0011d690)` |
| **Store Return Condition** | `0x0012c632` | `hook::slide_fn::<fn(*mut CleoScript, bool)>(0x0012c632)` |
| **Global Script Space** | `0x006ffac8` | Pointer memori ruang script global game |
| **Game Timer Global** | `0x0071c55c` | Variabel global waktu game (`u32`) |

---

### D. FPS Counter & Game Settings (`src/game/extras.rs`)

| Variabel Global | Alamat Memori | Keterangan |
| :--- | :--- | :--- |
| **Native Show FPS Flag** | `0x00746bb1` | Pointer `*mut bool`. Jika diset `true`, game otomatis memanggil `display_fps` bawaan |
| **FPS Cap (RsGlobal)** | `0x007e0d88` | Pointer `*mut u32`. Pengatur limit FPS game (30 / 60) |
| **Controller State** | `0x005f812c` | Pointer struct controller |
| **Controller Connected**| `0x001e28b4` | Fungsi pengecekan controller terhubung |

---

## 3. Masalah Utama yang Dihadapi & Solusinya

### 1. Thumb-2 Mode Execution pada ARMv7 Hooking
- **Gejala**: Aplikasi langsung crash (EXC_BAD_ACCESS / SIGSEGV) saat hook diaktifkan.
- **Penyebab**: Binary GTA SA 32-bit dikompilasi dengan instruksi ARM Thumb-2. Pointer fungsi Thumb memerlukan Least Significant Bit (LSB / bit 0) bernilai 1 (`addr | 1`). Jika dipanggil dengan alamat genap, CPU mengeksekusi sebagai instruksi ARM klasik dan crash.
- **Solusi**: Diimplementasikan fungsi `hook::slide_fn` yang otomatis mengaplikasikan `address | 1` hanya untuk target internal game binary.

### 2. Orientasi Menu CLEO Terbalik / Portrait
- **Gejala**: Splashscreen dan menu navigasi CLEO muncul dengan rotasi portrait/gepeng saat game berada pada mode landscape di iPhone 5.
- **Penyebab**: CLEO 64-bit lama menambahkan subview langsung ke `UIWindow` mentah tanpa memperhitungkan rotasi orientation transform dari `UIViewController` utama iOS 10.
- **Solusi**: Menambahkan container menu ke `rootViewController.view` dan menggunakan ukuran bound layar landscape iPhone 5 (568x320 point / 1136x640 pixel).

### 3. Reachability Class Crash (`button_hack`)
- **Gejala**: Game crash saat swipe untuk membuka menu CLEO karena `Reachability` class tidak ditemukan.
- **Penyebab**: Di iOS 10 / game binary 32-bit, nama class-nya adalah `IOSReachability`, bukan `Reachability`.
- **Solusi**: Menambahkan pengecekan runtime dynamic: jika `Reachability` tidak ada, gunakan `IOSReachability`, serta mengoreksi alamat hook `button_hack` ke `0x00017338`.

### 4. Cheat Salah Eksekusi (Index Offset Mismatch)
- **Gejala**: Cheat yang aktif tidak sesuai dengan yang dipilih (misal cheat meledakkan mobil malah mengaktifkan wheels only).
- **Penyebab**: Array cheat pada branch `main` (64-bit) berjumlah 111 cheat karena memiliki cheat debug ekstra (`CheatDebugMappings` dan `CheatXboxHelper`). Pada binary 32-bit v1.09, dua cheat debug tersebut tidak ada (total 109 cheat), sehingga seluruh index cheat setelahnya bergeser 1–2 posisi.
- **Solusi**: Dibuat array khusus 32-bit `static CHEATS: [Cheat; 109]` yang dicocokkan 100% dengan tabel fungsi `0x004f6cf4` di binary 32-bit GTA SA v1.09.

### 5. FPS Counter: Black Rectangle & HUD Corruption
- **Gejala**: Angka FPS tidak muncul, ada kotak hitam aneh (black rectangle) di pojok layar, dan teks HUD (uang, jam) hilang atau game crash.
- **Penyebab**: CLEO mencoba meng-hook `display_fps` (`0x00188fd0`) dan menggambar teksnya sendiri menggunakan `CFont`. Pada binary 32-bit, struktur CFont berbeda sehingga pemanggilan CFont merusak state render HUD game.
- **Solusi**: Membatalkan custom hook `display_fps` pada 32-bit. Sebagai gantinya, CLEO cukup mengubah flag bawaan game di address `0x00746bb1` menjadi `true`. Mesin game asli sendiri yang merender tampilan FPS resmi Rockstar secara sempurna tanpa merusak HUD.

### 6. Script CSI / CSA Tidak Membuka Menu
- **Gejala**: Script mod CLEO (seperti Vehicle Spawner) berstatus *Running* tapi menu dialog spawn tidak muncul.
- **Penyebab**: Opcode dispatch untuk instruksi SCM `>= 0x0a8c` belum terhubung ke handler default `0x0015616c`, dan tabel opcode `0x004a3494` belum terdefinisi untuk 32-bit.
- **Solusi**: Menambahkan handler default dan tabel pointer opcode yang tepat untuk arsitektur 32-bit.

### 7. Filter Kategori Menu Cheat
- **Implementasi**: 
  - Seluruh 111 `MessageKey` cheat diklasifikasikan ke 8 kategori logis (*Senjata, Kesehatan & Polisi, Spawn Kendaraan, Lalu Lintas, Karakter CJ, Cuaca & Waktu, Kekacauan, Lain-lain*).
  - Baris index 0 di tab `CHEAT` dipasang `CategoryFilterRow` dinamis (`[ KATEGORI CHEAT ]`) yang menunjukkan kategori aktif dan jumlah cheat (misal `SENJATA (16) ▸`).
  - Menambahkan message `MenuMessage::RebuildTab(2)` pada engine menu CLEO sehingga saat kategori diklik, baris-baris cheat langsung dibangun ulang seketika dan scroll view kembali ke paling atas (`y = 0`).

### 8. Ketidakcocokan ABI & Signature Parameter `process_touch`
- **Gejala**: Game crash seketika saat layar disentuh pertama kali (touch gesture hook gagal).
- **Penyebab**: Perbedaan ABI antara ARM64 dan ARMv7:
  - Pada 64-bit: `fn(f32, f32, f64, f32, u64)`.
  - Pada 32-bit: `fn(touch_type: u32, x: f32, y: f32, timestamp: f64)`.
  - Pada iOS 32-bit, `CGFloat` setara dengan `f32` (bukan `f64`). Urutan dan ukuran register parameter pada ARM calling convention berbeda.
- **Solusi**: Mengoreksi signature function pointer `process_touch` di `targets` menjadi `fn(u32, f32, f32, f64)` untuk build 32-bit.

### 9. Penghapusan Dependensi `reqwest` & Thread Update Checker di iOS 10
- **Gejala**: Aplikasi crash saat startup dengan error dynamic linker (`dyld: Symbol not found: _SecKeyCopyExternalRepresentation`).
- **Penyebab**: Crate `reqwest` bawaan branch `main` menggunakan Security API modern Apple yang belum tersedia pada iOS 10.3.4.
- **Solusi**: Menghapus dependensi `reqwest` dan menonaktifkan background update checker thread pada target pointer width 32-bit (`#[cfg(target_pointer_width = "32")]`).

---

## 4. Struktur Direktori & Path Runtime pada Perangkat

| File / Folder | Path Lengkap di iOS | Fungsi / Keterangan |
| :--- | :--- | :--- |
| **Folder Script CLEO** | `/var/mobile/Documents/CLEO/` | Tempat meletakkan file script mod (`.csi` dan `.csa`) |
| **File Log CLEO** | `/var/mobile/Documents/CLEO/cleo.log` | Log diagnosa runtime untuk melacak hook & error |
| **Status Cheat Tersimpan** | `/var/mobile/Documents/CLEO/cleo_saved_cheats.u8` | Binary state persistensi 109 cheat (1 byte per cheat) |
| **Lokasi Tweak Dylib** | `/Library/MobileSubstrate/DynamicLibraries/` | Berisi `CLEO.dylib` dan `CLEO.plist` hasil instalasi `.deb` |
| **Folder Cadangan Kode Asli** | `backup/` (di repo git) | Salinan cadangan `src/game/cheats.rs` dan `src/meta/menu.rs` (di-ignore oleh git) |
