# Folder Analisis Binary GTA III iOS 32-bit (ARMv7)

Folder ini disiapkan khusus untuk meletakkan file binary game **GTA III iOS (32-bit)** atau file `.ipa` yang akan di-porting ke CLEO.

> **Catatan:** Folder ini beserta file `.ipa`, `.log`, `.ips`, dan binary hasil ekstrak sudah terdaftar di `.gitignore` sehingga tidak akan membebani repository git. Hanya `README.md` dan `analyze_gta3.py` yang di-track.

---

## Cara Penggunaan:

1. **Salin binary atau file IPA ke folder ini**:
   - Jika Anda memiliki file `.ipa`: Cukup copas file tersebut ke folder ini (misal `GTA3.ipa` atau `grandtheftauto3.ipa`).
   - Jika Anda memiliki binary executable langsung: Beri nama `GTA3`, `gta3`, atau `grandtheftauto3`.

2. **Jalankan script analisis otomatis**:
   Buka terminal di root project atau folder ini, lalu jalankan:
   ```bash
   python dump_and_logs_gta3/analyze_gta3.py
   ```

3. **Apa yang dilakukan script**:
   - Jika mendeteksi file `.ipa`, script otomatis mengekstrak executable Mach-O dari `Payload/*.app/`.
   - Memeriksa apakah binary berisi slice `armv7` / `armv7s` (32-bit).
   - Menghitung dan menampilkan base memory address untuk segmen `__TEXT` dan `__DATA`.
   - Mem-dump tabel simbol fungsi (jika unstripped) ke file `_symbols.txt`.
   - Mencari offset dan lokasi memori untuk string & engine GTA III (seperti `CRunningScript`, `FindPlayerPed`, `CTheScripts`, cheat strings, dll).

4. **Gunakan hasil analisis** untuk mengisi placeholder `0x00000000` di:
   - [`src/lib.rs`](../src/lib.rs) — target hook utama
   - [`src/game/player.rs`](../src/game/player.rs) — fungsi player/ped
   - [`src/game/vehicles.rs`](../src/game/vehicles.rs) — spawner kendaraan
   - Lihat tabel lengkap di [`DOKUMENTASI_PENGEMBANGAN_GTA3_IPHONE5_ARMV7.md`](../DOKUMENTASI_PENGEMBANGAN_GTA3_IPHONE5_ARMV7.md)
