<h1 style="text-align: center">CLEO iOS — GTA III (iPhone 5)</h1>

> [!WARNING]
> **Ini adalah branch `For-iPhone-5-GTA3`** — dikhususkan untuk **Grand Theft Auto III v1.3.2** pada **iPhone 5 (ARMv7 32-bit, iOS 10.x, jailbreak rootful)**.
> Branch `main` untuk iPhone 6S+ (arm64), branch `For-iPhone-5` (GTA SA), dan branch `For-iPhone-5-VC` (GTA VC) **tidak dipengaruhi** oleh branch ini.

<div style="text-align: center">
  <p>
    <a href="#-bahasa-indonesia">🇮🇩 Bahasa Indonesia</a> | <a href="#-english">🇬🇧 English</a>
  </p>
  <p>
    <a href="https://discord.gg/cXwkTUasJU">
    <img src="https://img.shields.io/discord/767478053139775528?color=7289DA&amp;label=DISCORD&amp;style=for-the-badge" alt="Discord">
    </a>
    <img src="https://img.shields.io/github/downloads/squ1dd13/CLEO-iOS/total?style=for-the-badge" alt="Downloads">
    <img src="https://img.shields.io/github/license/squ1dd13/CLEO-iOS?style=for-the-badge" alt="Licence">
  </p>
  <p><a href="https://discord.gg/cXwkTUasJU"><strong>Join the Discord server for support, info and script suggestions</strong></a></p>
</div>

---

<h2 id="-bahasa-indonesia">🇮🇩 Bahasa Indonesia</h2>

Selamat datang di CLEO iOS untuk **Grand Theft Auto III**! Ini adalah porting *mod menu* dan *script loader* legendaris CLEO yang disesuaikan secara khusus untuk GTA III versi 32-bit (ARMv7/ARMv7s) pada perangkat iPhone 5 (iOS 10.x).

---

### 📱 Spesifikasi & Lingkungan Target

| Parameter | Keterangan |
|---|---|
| **Device Target** | iPhone 5 (Apple A6 SoC) |
| **Arsitektur CPU** | ARMv7 / ARMv7s (32-bit) |
| **Versi iOS** | iOS 10.0 s/d 10.3.4 |
| **Target Game** | **Grand Theft Auto III v1.3.2** (`com.rockstargames.gta3ios`, biner `gta3`) |
| **Tipe Jailbreak** | Rootful Jailbreak (h3lix, kok3shi9, socket-based) |
| **Substrate Loader** | Cydia Substrate ≥ 0.9.6000 |
| **File Hasil Build** | `cleogta3.iphone5.deb` |
| **Lokasi Tweak** | `/Library/MobileSubstrate/DynamicLibraries/CLEOGTA3.dylib` & `CLEOGTA3.plist` |
| **Lokasi Log** | `<Data Kontainer GTA 3>/Documents/cleo/cleo.log` |

---

### 🚀 Cara Instalasi (iPhone 5)

1. **Unduh Paket `.deb`**:
   - Masuk ke tab **[Actions](../../actions/workflows/build-iphone5.yml)** di repositori GitHub ini.
   - Pilih *workflow run* terbaru dari branch `For-iPhone-5-GTA3` → unduh berkas di bagian **Artifacts** (`cleogta3-iphone5-deb`).
   - Atau unduh langsung dari tab **[Releases](../../releases)** jika tersedia versi rilis.
2. **Transfer ke iPhone 5**:
   - Kirim file `cleogta3.iphone5.deb` menggunakan **3uTools**, **SFTP/SCP**, atau **Filza WebDAV**.
3. **Instal Paket**:
   - Buka aplikasi **Filza File Manager**, ketuk file `.deb`, lalu pilih **Install**.
   - Atau melalui Terminal (SSH / MTerminal):
     ```bash
     dpkg -i /path/to/cleogta3.iphone5.deb
     ```
4. **Respring**: Lakukan respring pada iPhone 5 Anda.
5. **Jalankan GTA III**: Buka game GTA III dan nikmati fitur CLEO!

---

### 🎮 Cara Membuka & Menggunakan Menu CLEO

- **Gestur Sentuh**: **Usap (swipe) layar dari atas ke bawah** di bagian tengah layar saat permainan berlangsung.
- **Navigasi Menu**:
  - Sentuh item untuk memilih atau mengaktifkan cheat/kendaraan/senjata.
  - Tab **Cheats**: Cheat instan maupun status aktif/nonaktif.
  - Tab **Vehicles**: Memunculkan mobil atau perahu secara langsung di depan pemain.
  - Tab **Weapons**: Memberikan paket senjata resmi GTA III.
  - Tab **CSA / CSI**: Mengelola skrip CLEO kustom.

---

### ✨ Fitur Unggulan CLEO GTA III (ARMv7)

1. **Menu Cheat Lengkap**:
   - **Health & Armor Penuh**: Memulihkan darah dan armor ke 100%.
   - **Uang Tunai ($250,000)**: Menambah nominal uang dan langsung memperbarui tampilan HUD.
   - **Never Wanted (Bebas Polisi)**: Mengunci wanted level dan chaos level ke 0 sehingga polisi tidak mengejar.
   - **Infinite Ammo**: Amunisi tidak terbatas untuk semua senjata yang dibawa.
2. **Weapon Giver (11 Senjata Resmi GTA III)**:
   - Baseball Bat (Model 172)
   - Pistol / Colt45 (Model 173)
   - Uzi (Model 178)
   - Shotgun (Model 176)
   - AK-47 (Model 171)
   - M16 (Model 180)
   - Sniper Rifle (Model 177)
   - Rocket Launcher (Model 175)
   - FlameThrower (Model 181)
   - Molotov (Model 174)
   - Grenade (Model 170)
   - *Dilengkapi fitur sinkronisasi streaming model otomatis untuk mencegah glitch atau crash.*
3. **Vehicle Spawner (61 Model Kendaraan GTA III)**:
   - Seluruh mobil jalanan dan dinas (`CAutomobile`).
   - Seluruh perahu (`CBoat`): Predator, Speeder, Reefer, Ghost.
   - Deteksi ketinggian tanah akurat via `CWorld::FindGroundZForCoord`.
4. **Script & Mod Loader**:
   - Eksekusi skrip `.csa` (berjalan otomatis) dan `.csi` (dijalankan manual).
   - Penggantian aset game via folder `Replace` di dalam direktori `Documents/cleo/`.
   - Modding arsip `.img` tanpa merusak file asli aplikasi.

---

### 🛠️ Ringkasan Pembaruan Teknis Porting GTA III

- **Reverse Engineering ARMv7 Mach-O**: Alamat memori, pointer global, dan *offset struct* dianalisis langsung dari biner resmi GTA III iOS v1.3.2 (2.2 MB thin ARMv7).
- **Thumb-2 PIC Resolution**: Menghitung secara presisi pasangan instruksi `movw`/`movt` + `add Rd, pc` untuk pointer global seperti `CWorld::Players` (`0x003175F0`), `PlayerInFocus` (`0x003796E4`), `MaximumChaosLevel` (`0x001BEBCC`), dan `MaximumWantedLevel` (`0x001BEBC8`).
- **Penyesuaian Struct Engine RenderWare**:
  - `CPed`: Ukuran `0x5AC`, Health di `0x2C8`, Armor di `0x2C4`, Wanted ptr di `0x544`, Weapon array di `0x360` (stride `0x18`).
  - `CPlayerInfo`: Ukuran `0x13C`, Money di `0xB0`, Display Money di `0xB4`.
  - `CWanted`: Chaos level di `+0x00`, Wanted Stars di `+0x18`.
- **Integrasi CI/CD Otomatis**: Workflow GitHub Actions khusus mem-build file `.deb` menggunakan compiler cross-compile ARMv7s dan linking otomatis ke stub `libSystem.tbd`.

---

<h2 id="-english">🇬🇧 English</h2>

Welcome to CLEO iOS for **Grand Theft Auto III**! This is the dedicated port of the legendary CLEO mod menu and script loader specifically rebuilt for 32-bit ARMv7/ARMv7s devices running GTA III v1.3.2 on iOS 10.x (iPhone 5).

---

### 📱 Target Device & Specifications

| Property | Detail |
|---|---|
| **Target Device** | iPhone 5 (Apple A6 SoC) |
| **CPU Architecture** | ARMv7 / ARMv7s (32-bit) |
| **iOS Version** | iOS 10.0 to 10.3.4 |
| **Target Game** | **Grand Theft Auto III v1.3.2** (`com.rockstargames.gta3ios`, binary `gta3`) |
| **Jailbreak Type** | Rootful Jailbreak (h3lix, kok3shi9, socket-based) |
| **Substrate Loader** | Cydia Substrate ≥ 0.9.6000 |
| **Debian Package** | `cleogta3.iphone5.deb` |
| **Tweak Files** | `/Library/MobileSubstrate/DynamicLibraries/CLEOGTA3.dylib` & `CLEOGTA3.plist` |
| **Log Path** | `<GTA 3 Data Container>/Documents/cleo/cleo.log` |

---

### 🚀 Installation Guide (iPhone 5)

1. **Download `.deb` Package**:
   - Go to the **[Actions](../../actions/workflows/build-iphone5.yml)** tab of this repository.
   - Select the latest run on branch `For-iPhone-5-GTA3` → download `cleogta3-iphone5-deb` from **Artifacts**.
   - Or download directly from **[Releases](../../releases)** when tagged.
2. **Transfer to iPhone 5**:
   - Transfer `cleogta3.iphone5.deb` via **3uTools**, **SFTP/SCP**, or **Filza WebDAV**.
3. **Install Package**:
   - Open **Filza File Manager**, tap the file, and press **Install**.
   - Or run via terminal:
     ```bash
     dpkg -i /path/to/cleogta3.iphone5.deb
     ```
4. **Respring**: Respring your iPhone 5.
5. **Launch GTA III**: Start GTA III and enjoy CLEO!

---

### 🎮 Opening the CLEO Menu

- **Swipe Gesture**: **Swipe down** from the top-center of the screen during gameplay.
- **Menu Tabs**:
  - **Cheats**: Toggle health, armor, money, police wanted status, and ammo.
  - **Vehicles**: Spawn any of the 61 GTA III vehicles (automobiles and boats).
  - **Weapons**: Give Claude any of the 11 authentic GTA III weapons.
  - **CSA / CSI**: Run and manage custom CLEO scripts.

---

### 🛠️ Building From Source

This branch automatically builds via **GitHub Actions** on every push (`.github/workflows/build-iphone5.yml`).

To build manually on macOS:
```bash
# 1. Install prerequisites
brew install ldid dpkg

# 2. Configure Rust nightly with rust-src
rustup override set nightly
rustup component add rust-src

# 3. Compile static library
cargo build --target armv7s-apple-ios --release

# 4. Link using Clang with stub libSystem (ARMv7s)
SDK=$(xcrun --sdk iphoneos --show-sdk-path)
clang -fpic -shared -Wl,-all_load \
  target/armv7s-apple-ios/release/libcleo.a \
  -o target/armv7s-apple-ios/release/libcleo.dylib \
  -target armv7s-apple-ios10.0 -arch armv7s \
  -Wl,-undefined,dynamic_lookup \
  -lobjc -lc++

# 5. Sign and package
ldid -S target/armv7s-apple-ios/release/libcleo.dylib
```

---

## 👥 Credits & Acknowledgments

- **Original CLEO iOS Creator**: [squ1dd13](https://github.com/squ1dd13)
- **ARMv7 GTA III & Vice City Port / Reverse Engineering**: **Syahda Fahreza**
- **CLEO Library**: [Seemann](https://github.com/x87)
- **CLEO Android**: [Alexander Blade](http://www.dev-c.com/)
- **Game Engine Research & SDK**: [plugin-sdk (DK22Pac)](https://github.com/DK22Pac/plugin-sdk) & [gta-reversed](https://github.com/codenulls/gta-reversed)
- Translators and GTA modding community for ongoing support!
