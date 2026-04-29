### ==== Language settings ====

# Used in the settings menu to show the name of the language.
language-name = Bahasa Indonesia (Gaul)

# Shown when this language has been selected automatically.
language-auto-name = Otomatis ({ language-name })

# The name of the language setting.
language-opt-title = Bahasa

# The language setting description.
language-opt-desc = Bahasa yang dipakai buat CLEO. Kalo otomatis, ngikutin bahasa sistem HP lo. Bikin bahasa lu sendiri di Discord!

### ==== Splash screen ====

# First line at the bottom of the screen.
splash-legal = Copyright © 2020-2023 { $copyright_names }. Dilindungi MIT License.

# Second line.
splash-fun = Dibuat dengan penuh cinta di UK. Enjoy, cuy!

### ==== Updates ====

# PLEASE NOTE: The update menu is currently made from a game menu, and the game fonts do not
# support non-ASCII characters. Please write with ASCII only here. If you can't do that, leave it
# in English for now. I plan to add Unicode support here in the future.

# Displayed at the top of the update screen.
update-prompt-title = Ada Update Baru Nih

# Message shown on the update screen. { $new_version } will be replaced with the update's version number.
update-prompt-message = CLEO versi { $new_version } udah rilis. Mau ke GitHub sekrang buat download?

# todo: Add "Yes" and "No" for update menu to localisation files.
# The yes/no options are part of the game, so they're not directly in CLEO's control (yet).

## Release channel settings

update-release-channel-opt-title = Saluran Update
update-release-channel-opt-desc = Milih update CLEO mana yang bakal kasih lo notifikasi. Kalo Alpha fiturnya lebih baru dan cepet rilis, tapi mungkin masih nge-bug. Gak disaranin buat matiin notifnya.

update-release-channel-opt-disabled = Mati
update-release-channel-opt-stable = Stabil
update-release-channel-opt-alpha = Alpha

### ==== Menu ====

# Title for the button at the bottom of the screen that closes the CLEO menu.
menu-close = Tutup

# Title for the options tab.
menu-options-tab-title = Opsi

## Menu gesture settings

menu-gesture-opt-title = Gestur Menu
menu-gesture-opt-desc = Cara ngebuka menu CLEO pas lagi main.

# A single motion where one finger moves quickly down the screen.
menu-gesture-opt-one-finger-swipe = Usap satu jari ke bawah
# A single swipe (as above) but with two fingers at the same time instead of just one.
menu-gesture-opt-two-finger-swipe = Usap dua jari ke bawah
# A short tap on the screen with two fingers at once.
menu-gesture-opt-two-finger-tap = Ketuk dua jari
# A short tap on the screen with three fingers at once.
menu-gesture-opt-three-finger-tap = Ketuk tiga jari

### ==== Script menu ====

## Script warning overview

# Shown at the top of the script menus when at least one error has been found in any script. This
# is not shown at all if there are zero scripts with errors in them.
menu-script-warning-overview =
    { $num_scripts_with_errors ->
        [one] Ketemu masalah di satu skrip. Skripnya ditandai warna oren ya.
        *[other] Ketemu masalah di { $num_scripts_with_errors } skrip. Skripnya ditandai warna oren ya.
    }

# The second line of the warning.
menu-script-see-below = Liat ke bawah buat detailnya.

menu-script-csa-tab-title = CSA
menu-script-csi-tab-title = CSI

## Specific script warnings

# The script does things that CLEO doesn't support yet.
script-unimplemented-in-cleo = Pake fitur yang belum di-support CLEO iOS.
# The script does things that are not possible on iOS (for system reasons).
script-impossible-on-ios = Pake kode yang emang gak bakal jalan di iOS.
# The script is identical to another script. { $original_script } will be replaced with the name of
# the script that this one is a duplicate of.
script-duplicate = Skrip kembar sama { $original_script }.
# There was an error when checking the script code for problems.
script-check-failed = Gagal nge-scan skrip. Laporin bug ini ke GitHub atau Discord dong!
# No problems were found when scanning the script. This is a safe script!
script-no-problems = Aman sentosa, gak ada masalah.

# Formats for script names in the menu.
script-csa-row-title = { $script_name }
script-csi-row-title = { $script_name }

## Script status messages

# The script is running normally.
script-running = Jalan
# The script is not running.
script-not-running = Gak Jalan

# The script has been forced to run by the user, even though there are problems with it. This only
# applies to CSA scripts.
script-csa-forced-running = Dipaksa Jalan

## Script settings

script-mode-opt-title = Mode Proses Skrip
script-mode-opt-desc = Ganti cara CLEO jalanin kode. Coba utak-atik opsi ini kalo nemu masalah sama skripnya.

# In "don't break" mode, CLEO won't try to reduce script lag. This might be more stable sometimes.
script-mode-opt-dont-break = Lambat
# In "break" mode, CLEO will try to reduce script lag caused by long loops in script code.
script-mode-opt-break = Ngebut

### ==== FPS ====

## FPS lock option

fps-lock-opt-title = Baterai vs FPS
fps-lock-opt-desc = Maksimal framerate buat main gamenya. 30 FPS keliatan agak patah-patah tapi irit baterai bos.

fps-lock-opt-30 = 30 FPS
fps-lock-opt-60 = 60 FPS

## FPS counter option

fps-counter-opt-title = Tunjukin FPS
fps-counter-opt-desc = Nyalain atau matiin angka FPS di layar.

fps-counter-opt-hidden = Sembunyi
fps-counter-opt-enabled = Muncul

### ==== Cheat system ====

## Menu

cheat-tab-title = Cheat

# Two lines of text shown at the top of the cheats menu.
cheat-menu-warning = Nge-cheat bisa bikin game crash sama ngerusak progress savean lo.
  Mendingan lo backup save data lo di slot lain dulu sebelum nyobain cheat!

## Status messages for cheats

cheat-on = Nyala
cheat-off = Mati

# Cheat will be turned on when the menu is closed.
cheat-queued-on = Antre Nyala
# Cheat will be turned off when the menu is closed.
cheat-queued-off = Antre Mati

# Formats for cheat codes in the menu.
cheat-code-row-title = { $cheat_code }
cheat-no-code-title = ???

## Cheat saving option

cheat-transience-opt-title = Mode Simpan Cheat
cheat-transience-opt-desc = Ngatur cheat pas lagi muat ulang atau restart gamenya.

cheat-transience-opt-transient = Reset Semua
cheat-transience-opt-persistent = Simpan Status

### ==== Cheat descriptions ====

## Weapon sets
cheat-thugs-armoury = Paket Senjata 1 (Preman)
cheat-professionals-kit = Paket Senjata 2 (Pro)
cheat-nutters-toys = Paket Senjata 3 (Sinting)
cheat-weapons-4 = Dildo, minigun, kacamata tembus malam/panas

## Debug cheats
cheat-debug-mappings = Debug (tunjukin mapping)
cheat-debug-tap-to-target = Debug (tunjukin tap to target)
cheat-debug-targeting = Debug (tunjukin targeting)

## Properly cheating
cheat-i-need-some-help = Nyawa, armor, sama duit $250.000
cheat-skip-mission = Skip misi langsung kelar

## Superpowers
cheat-full-invincibility = Kebal dari segala arah
cheat-sting-like-a-bee = Pukulan maut
cheat-i-am-never-hungry = Gak pernah laper
cheat-kangaroo = Lompat setinggi kanguru (10x lipat)
cheat-noone-can-hurt-me = Nyawa gak abis-abis
cheat-man-from-atlantis = Napas di air tanpa batas

## Social player attributes
cheat-worship-me = Respect paling pol
cheat-hello-ladies = Karisma pol buat narik cewek

## Physical player attributes
cheat-who-ate-all-the-pies = Paling gendut sedunia
cheat-buff-me-up = Otot kawat tulang besi
cheat-max-gambling = Skill judi tingkat dewa
cheat-lean-and-mean = Kurus kering
cheat-i-can-go-all-night = Stamina kuda

## Player skills
cheat-professional-killer = Skill nembak dewa (Hitman) semua senjata
cheat-natural-talent = Nyetir sejago pembalap

## Wanted level
cheat-turn-up-the-heat = Tambah 2 bintang polisi
cheat-turn-down-the-heat = Polisi minggat (bersih)
cheat-i-do-as-i-please = Kunci bintang polisi di posisi sekarang
cheat-bring-it-on = Polisi bintang 6, gaslah!

## Weather
cheat-pleasantly-warm = Cuaca cerah
cheat-too-damn-hot = Cuaca panas banget
cheat-dull-dull-day = Mendung cuy
cheat-stay-in-and-watch-tv = Ujan deres
cheat-cant-see-where-im-going = Berkabut
cheat-scottish-summer = Badai petir
cheat-sand-in-my-ears = Badai pasir

## Time
cheat-clock-forward = Cepetin jam 4 jam
cheat-time-just-flies-by = Waktu kerasa lebih cepet
cheat-speed-it-up = Game makin cepet (Fast forward)
cheat-slow-it-down = Game gerak lambat (Slo-mo)
cheat-night-prowler = Tengah malem mulu
cheat-dont-bring-on-the-night = Mentok di jam 9 malem

## Spawning wearables
cheat-lets-go-base-jumping = Munculin Parasut
cheat-rocketman = Munculin Jetpack

## Spawning vehicles

# The vehicle names (in capitals) should not be changed, as they are part of the game. The
# descriptions (in brackets) do need to be translated.
cheat-time-to-kick-ass = Munculin Rhino (Tank tentara)
cheat-old-speed-demon = Munculin Bloodring Banger (Mobil tabrakan)
cheat-tinted-rancher = Munculin Rancher kaca gelap (Mobil SUV)
cheat-not-for-public-roads = Munculin Hotring Racer A (Mobil balap A)
cheat-just-try-and-stop-me = Munculin Hotring Racer B (Mobil balap B)
cheat-wheres-the-funeral = Munculin Romero (Mobil jenazah)
cheat-celebrity-status = Munculin Stretch Limousine (Mobil Limo)
cheat-true-grime = Munculin Trashmaster (Truk sampah)
cheat-18-holes = Munculin Caddy (Mobil golf)
cheat-jump-jet = Munculin Hydra (Jet tempur)
cheat-i-want-to-hover = Munculin Vortex (Hovercraft)
cheat-oh-dude = Munculin Hunter (Helikopter tempur)
cheat-four-wheel-fun = Munculin Quad (Motor roda empat)
cheat-hit-the-road-jack = Munculin Tanker sama trailer (Truk tangki)
cheat-its-all-bull = Munculin Dozer (Buldoser)
cheat-flying-to-stunt = Munculin Stunt Plane (Pesawat akrobat)
cheat-monster-mash = Munculin Monster Truck (Truk monster)

## Gang recruitment
cheat-wanna-be-in-my-gang = Rekrut orang jadi geng pake pistol
cheat-noone-can-stop-us = Rekrut orang jadi geng pake AK-47
cheat-rocket-mayhem = Rekrut orang jadi geng pake RPG

## Traffic
cheat-all-drivers-are-criminals = Semua sopir jadi agresif n punya bintang
cheat-pink-is-the-new-cool = Mobil warga jadi warna pink semua
cheat-so-long-as-its-black = Mobil warga jadi item legam
cheat-everyone-is-poor = Isinya mobil butut dari desa
cheat-everyone-is-rich = Jalanan isinya mobil sport orang kaya

## Pedestrians
cheat-rough-neighbourhood = Dapet stik golf, pejalan kaki pada ngamuk
cheat-stop-picking-on-me = Pejalan kaki pada ngincer kita
cheat-surrounded-by-nutters = Pejalan kaki pada bawa senjata
cheat-blue-suede-shoes = Semua orang jadi Elvis Presley
cheat-attack-of-the-village-people = Pejalan kaki nembakin kita pake senpi & roket
cheat-only-homies-allowed = Isinya anak geng doang
cheat-better-stay-indoors = Geng pada berkuasa di jalan
cheat-state-of-emergency = Kota rusuh total
cheat-ghost-town = Kota mati, sepi kendaraan & orang

## Themes
cheat-ninja-town = Tema Yakuza/Triad
cheat-love-conquers-all = Tema germo & PSK
cheat-lifes-a-beach = Tema pesta pantai
cheat-hicksville = Tema orang kampung
cheat-crazy-town = Tema sirkus karnaval

## General vehicle cheats
cheat-all-cars-go-boom = Ledakin semua mobil di sekitar
cheat-wheels-only-please = Mobil jadi tembus pandang (keliatan bannya doang)
cheat-sideways-wheels = Ban mobilnya miring
cheat-speed-freak = Semua mobil dapet NOS/Nitro
cheat-cool-taxis = Taksi dapet hidrolik sama Nitro

## Vehicle cheats for the player
cheat-chitty-chitty-bang-bang = Mobil terbang
cheat-cj-phone-home = Sepeda BMX loncatnya tinggi bet
cheat-touch-my-car-you-die = Nabrak kendaraan lain langsung meledak
cheat-bubble-cars = Ditabrak melayang ke angkasa
cheat-stick-like-glue = Handling & suspensi makin ciamik
cheat-dont-try-and-stop-me = Lampu lalu lintas ijo mulu
cheat-flying-fish = Kapal bisa terbang

## Weapon usage
cheat-full-clip = Peluru gak bakal abis, auto-reload
cheat-i-wanna-driveby = Bebas nembak pas lagi nyetir

## Player effects
cheat-goodbye-cruel-world = Bundir (Bunuh Diri)
cheat-take-a-chill-pill = Efek adrenalin naik
cheat-prostitutes-pay = Dibayar PSK abis ena-ena

## Miscellaneous
cheat-xbox-helper = Sesuaikan stats biar dapet achievement Xbox

## Pointless cheats

# Tells the user that a cheat will ALWAYS crash their game.
cheat-crash-warning = PASTI CRASH!

cheat-slot-melee = { cheat-crash-warning } Slot Melee (Jarak Dekat)
cheat-slot-handgun = { cheat-crash-warning } Slot Pistol
cheat-slot-smg = { cheat-crash-warning } Slot SMG
cheat-slot-shotgun = { cheat-crash-warning } Slot Shotgun
cheat-slot-assault-rifle = { cheat-crash-warning } Slot Assault Rifle
cheat-slot-long-rifle = { cheat-crash-warning } Slot Senapan Laras Panjang
cheat-slot-thrown = { cheat-crash-warning } Slot Lemparan
cheat-slot-heavy = { cheat-crash-warning } Slot Senjata Berat
cheat-slot-equipment = { cheat-crash-warning } Slot Equipment (Peralatan)
cheat-slot-other = { cheat-crash-warning } Slot Lainnya

cheat-predator = Gak ngaruh ngab
