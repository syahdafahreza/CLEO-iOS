# Dokumentasi Pengembangan & Porting CLEO-iOS untuk GTA Vice City (iPhone 5 / ARMv7 32-bit)

**Target**: GTA Vice City iOS (ARMv7 32-bit)  
**Perangkat Target**: iPhone 5 (iPhone5,2 / iOS 10.3.4)  
**Branch**: `For-iPhone-5-VC`  
**Bundle ID**: `com.rockstargames.gtavc`  
**Executable**: `gta3vc` / `gtavc` / `ViceCity`  

---

## 1. Spesifikasi Teknis & Lingkungan (Target Environment)

- **Arsitektur**: ARMv7 / ARMv7s (32-bit).
- **Sistem Operasi**: iOS 10.3.4 (Jailbroken, Rootful).
- **ABI & Tipe Data**:
  - `CGFloat` pada 32-bit adalah `f32`.
  - `usize` / `isize` berukuran 4 byte (32-bit).
  - Thumb-2 mode calling convention: target fungsi internal game executable memerlukan bit LSB 1 (`address | 1`).
- **Packaging & Cydia Substrate**:
  - Filter plist: `com.rockstargames.gtavc`.
  - Kompresi package deb wajib `gzip` (`data.tar.gz`), karena dpkg di iOS 10 belum mendukung `xz`.
  - Lokasi dylib: `/Library/MobileSubstrate/DynamicLibraries/CLEO.dylib` dan `CLEO.plist`.

---

## 2. Perbedaan Arsitektur: GTA Vice City vs GTA San Andreas

| Aspek | GTA San Andreas (SA) | GTA Vice City (VC) | Catatan Porting |
| :--- | :--- | :--- | :--- |
| **Bundle ID** | `com.rockstargames.gta3sa` | `com.rockstargames.gtavc` | Disesuaikan di `deb/cleo.plist` |
| **Nama Binary** | `gta3sa` | `gta3vc` / `ViceCity` | Disesuaikan di filter Substrate |
| **Engine Script (SCM)** | SCM SA (kompleks, 0x0A8C+ opcode cleo) | SCM VC (format GTA III/VC) | Perlu mapping opcode dispatch VC |
| **Sistem Cheat** | Index function table (109 cheat di SA 32-bit) | String/Key hash atau Cheat Table VC | Cheat khas VC: *PANZER, ASPIRINE, THUGSTOOLS, dll* |
| **Touch Gesture** | `process_touch` (`0x003ece38` di SA v1.09) | `CTouchInterface` handler | Signature ARMv7: `fn(u32, f32, f32, f64)` |
| **Script Tick** | `0x0011e6f4` (`CRunningScript::Process`) | `CRunningScript::Process` di VC | Loop tick eksekusi script CLEO |
| **GXT Text Lookup** | `0x00356380` | Lookup text GXT VC (`text.gxt`) | Digunakan untuk format nama mobil/senjata |
| **FPS Cap** | `RsGlobal` / frame limiter | `RsGlobal` / frame limiter | Target 30 / 60 FPS |

---

## 3. Matriks Alamat Memori Target GTA Vice City (32-bit)

*(Tabel ini akan diisi secara lengkap setelah file binary `gta3vc` atau `.ipa` dicopas ke folder `dump_and_logs_vc/` dan dianalisis).*

### A. Core Hooks (`src/lib.rs`)

| Simbol / Fungsi | Alamat Memori (VC 32-bit) | Status | Keterangan |
| :--- | :--- | :--- | :--- |
| `script_tick` | `TBD` | Menunggu binary | `CRunningScript::Process` engine script |
| `process_touch` | `TBD` | Menunggu binary | Input gesture layar sentuh |
| `get_gxt_string` | `TBD` | Menunggu binary | Lookup teks GXT |
| `button_hack` | `TBD` | Menunggu binary | Reachability hook menu swipe |
| `idle` / `process` | `TBD` | Menunggu binary | Main game loop / frame update |
| `cycles_per_ms` | `TBD` | Menunggu binary | Timer game cycles |
| `do_cheats` | `TBD` | Menunggu binary | Cheat processing engine |

### B. Segment Base Address

| Segmen | VM Address Range | Ukuran | Keterangan |
| :--- | :--- | :--- | :--- |
| `__TEXT` Base | `TBD` | `TBD` | Base address kode mesin executable |
| `__DATA` Base | `TBD` | `TBD` | Base address variabel global |

---

## 4. Checklist Langkah Kerja Selanjutnya

1. [x] Buat branch baru `For-iPhone-5-VC`.
2. [x] Siapkan folder kerja analisis `dump_and_logs_vc/` & script helper `analyze_vc.py`.
3. [x] Update filter bundle ID Cydia Substrate ke `com.rockstargames.gtavc`.
4. [x] Update metadata `control.iphone5` dan `Cargo.toml`.
5. [x] Update workflow GitHub Actions CI untuk build branch `For-iPhone-5-VC`.
6. [ ] **User menyalin binary IPA / executable GTA Vice City ke `dump_and_logs_vc/`**.
7. [ ] Jalankan `python dump_and_logs_vc/analyze_vc.py` untuk mengekstrak segmen dan mencari simbol/offset.
8. [ ] Petakan alamat-alamat hook ke `src/targets` dan `src/game/`.
9. [ ] Sesuaikan cheat system untuk cheat khas Vice City.
10. [ ] Build paket `.deb` dan test di iPhone 5.
