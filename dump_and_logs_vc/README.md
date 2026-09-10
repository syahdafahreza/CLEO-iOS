# Folder Analisis Binary GTA Vice City iOS 32-bit (ARMv7)

Folder ini disiapkan khusus untuk meletakkan file binary game GTA Vice City iOS (32-bit) atau file `.ipa` yang akan di-porting ke CLEO.

> **Catatan:** Folder ini beserta file `.ipa`, `.log`, `.ips`, dan binary hasil ekstrak sudah terdaftar di `.gitignore` sehingga tidak akan membebani repository git.

---

## Cara Penggunaan:

1. **Salin binary atau file IPA ke folder ini**:
   - Jika Anda memiliki file `.ipa`: Cukup copas file tersebut ke folder ini (misal `ViceCity.ipa` atau `gtavc.ipa`).
   - Jika Anda memiliki binary executable langsung: Beri nama `gta3vc`, `gtavc`, atau `ViceCity`.

2. **Jalankan script analisis otomatis**:
   Buka terminal di root project atau folder ini, lalu jalankan:
   ```bash
   python dump_and_logs_vc/analyze_vc.py
   ```

3. **Apa yang dilakukan script**:
   - Jika mendeteksi file `.ipa`, script otomatis mengekstrak executable Mach-O dari `Payload/*.app/`.
   - Memeriksa apakah binary berisi slice `armv7` / `armv7s` (32-bit).
   - Menghitung dan menampilkan base memory address untuk segmen `__TEXT` dan `__DATA`.
   - Mem-dump tabel simbol fungsi (jika unstripped) ke file `_symbols.txt`.
   - Mencari offset dan lokasi memori untuk string & engine GTA VC (seperti `CRunningScript`, `CCheat`, `RsGlobal`, dll).

Hasil analisis tersebut nantinya akan kita gunakan langsung untuk mengisi target hook di `src/lib.rs` dan file terkait.
