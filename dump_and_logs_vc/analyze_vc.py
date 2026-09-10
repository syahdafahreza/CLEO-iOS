#!/usr/bin/env python3
"""
GTA Vice City iOS 32-bit (Mach-O ARMv7) Binary Analyzer for CLEO Porting
Usage:
    python analyze_vc.py [path_to_binary_or_ipa]
    If no argument is given, it looks for any binary or .ipa in the current directory.
"""

import sys
import os
import struct
import zipfile
import re
from pathlib import Path

# Mach-O Constants
FAT_MAGIC = 0xcafebabe
FAT_CIGAM = 0xbebafeca
MH_MAGIC = 0xfeedface       # 32-bit
MH_CIGAM = 0xcefaedfe
MH_MAGIC_64 = 0xfeedfacf    # 64-bit
MH_CIGAM_64 = 0xcffaedfe

CPU_TYPE_ARM = 12
CPU_SUBTYPE_ARM_V7 = 9
CPU_SUBTYPE_ARM_V7S = 11
CPU_TYPE_ARM64 = 0x0100000c

LC_SEGMENT = 0x1
LC_SYMTAB = 0x2
LC_DYSYMTAB = 0xb
LC_SEGMENT_64 = 0x19


def find_target_file(dir_path: Path):
    candidates = ["gta3vc", "gtavc", "ViceCity", "gta3vc.app", "Payload"]
    for c in candidates:
        p = dir_path / c
        if p.is_file():
            return p
        if p.is_dir():
            for inner in ["gta3vc", "gtavc", "ViceCity"]:
                ip = p / inner
                if ip.is_file():
                    return ip

    for f in dir_path.glob("*.ipa"):
        return f
    for f in dir_path.glob("*.zip"):
        return f

    for f in dir_path.iterdir():
        if f.is_file() and not f.name.endswith(".py") and not f.name.endswith(".md") and not f.name.endswith(".txt"):
            if f.stat().st_size > 2 * 1024 * 1024:
                return f

    return None


def extract_binary_from_ipa(ipa_path: Path, out_dir: Path) -> Path:
    print(f"[*] Extracting executable from IPA archive: {ipa_path.name}...")
    import plistlib
    with zipfile.ZipFile(ipa_path, 'r') as zf:
        # First find Info.plist to get CFBundleExecutable
        exec_name = None
        app_dir = None
        for name in zf.namelist():
            if name.startswith('Payload/') and name.endswith('.app/Info.plist'):
                try:
                    plist_data = zf.read(name)
                    pl = plistlib.loads(plist_data)
                    exec_name = pl.get('CFBundleExecutable')
                    app_dir = str(Path(name).parent).replace('\\', '/')
                    print(f"[+] Info.plist indicates CFBundleExecutable = '{exec_name}'")
                    break
                except Exception as e:
                    print(f"[-] Failed reading Info.plist: {e}")

        if not exec_name:
            exec_name = "gta3vc"
        if not app_dir:
            app_dir = "Payload/gta3vc.app"

        target_member = f"{app_dir}/{exec_name}"
        if target_member in zf.namelist():
            extracted_path = out_dir / exec_name
            print(f"[*] Extracting {target_member} -> {extracted_path}...")
            with zf.open(target_member) as src, open(extracted_path, 'wb') as dst:
                dst.write(src.read())
            return extracted_path

        # Fallback: scan files
        for name in zf.namelist():
            parts = name.split('/')
            if len(parts) == 3 and parts[0] == 'Payload' and parts[1].endswith('.app'):
                fname = parts[2]
                if fname in ("gta3vc", "gtavc", "ViceCity") and not fname.endswith(".plist"):
                    extracted_path = out_dir / fname
                    with zf.open(name) as src, open(extracted_path, 'wb') as dst:
                        dst.write(src.read())
                    return extracted_path

    raise RuntimeError("No main executable found in Payload/*.app/ inside the IPA!")


def parse_macho_slices(data: bytes):
    if len(data) < 4:
        raise ValueError("File is too small to be a Mach-O file.")

    magic = struct.unpack(">I", data[:4])[0]
    slices = []

    if magic in (FAT_MAGIC, FAT_CIGAM):
        endian = ">" if magic == FAT_MAGIC else "<"
        nfat_arch = struct.unpack(f"{endian}I", data[4:8])[0]
        print(f"[+] Universal Fat Binary detected with {nfat_arch} architectures.")

        for i in range(nfat_arch):
            offset = 8 + i * 20
            cputype, cpusubtype, arch_offset, arch_size, align = struct.unpack(
                f"{endian}IIIII", data[offset:offset+20]
            )
            arch_name = "unknown"
            if cputype == CPU_TYPE_ARM:
                if cpusubtype == CPU_SUBTYPE_ARM_V7:
                    arch_name = "armv7"
                elif cpusubtype == CPU_SUBTYPE_ARM_V7S:
                    arch_name = "armv7s"
                else:
                    arch_name = f"arm (subtype {cpusubtype})"
            elif cputype == CPU_TYPE_ARM64:
                arch_name = "arm64"

            slices.append({
                "arch": arch_name,
                "cputype": cputype,
                "cpusubtype": cpusubtype,
                "offset": arch_offset,
                "size": arch_size,
            })
            print(f"    - Slice {i}: {arch_name} (offset: 0x{arch_offset:X}, size: {arch_size:,} bytes)")
    elif magic in (MH_MAGIC, MH_CIGAM):
        slices.append({
            "arch": "armv7 (32-bit Mach-O thin)",
            "offset": 0,
            "size": len(data)
        })
    elif magic in (MH_MAGIC_64, MH_CIGAM_64):
        slices.append({
            "arch": "arm64 (64-bit Mach-O thin)",
            "offset": 0,
            "size": len(data)
        })
    else:
        raise ValueError(f"Unknown file signature: 0x{magic:08X}")

    return slices


def analyze_32bit_slice(data: bytes, slice_info: dict, out_symbols_path: Path):
    offset = slice_info["offset"]
    slice_data = data[offset : offset + slice_info["size"]]

    magic = struct.unpack("<I", slice_data[:4])[0]
    endian = "<"
    if magic == MH_CIGAM:
        endian = ">"
    elif magic != MH_MAGIC:
        print(f"[-] Slice at 0x{offset:X} is not a 32-bit Mach-O (magic=0x{magic:X})")
        return

    cputype, cpusubtype, filetype, ncmds, sizeofcmds, flags = struct.unpack(
        f"{endian}IIIIII", slice_data[4:28]
    )

    print(f"\n==================================================")
    print(f"  ANALYSIS FOR 32-BIT SLICE ({slice_info['arch']})")
    print(f"==================================================")
    print(f"Header: ncmds={ncmds}, sizeofcmds={sizeofcmds}, flags=0x{flags:X}")

    cmd_offset = 28
    segments = []
    symtab_info = None

    for _ in range(ncmds):
        if cmd_offset + 8 > len(slice_data):
            break
        cmd, cmdsize = struct.unpack(f"{endian}II", slice_data[cmd_offset : cmd_offset + 8])

        if cmd == LC_SEGMENT:
            segname = slice_data[cmd_offset + 8 : cmd_offset + 24].decode("latin1").rstrip("\x00")
            vmaddr, vmsize, fileoff, filesize, maxprot, initprot, nsects, flags_seg = struct.unpack(
                f"{endian}IIIIIIII", slice_data[cmd_offset + 24 : cmd_offset + 56]
            )
            segments.append({
                "name": segname,
                "vmaddr": vmaddr,
                "vmsize": vmsize,
                "fileoff": fileoff,
                "filesize": filesize,
                "nsects": nsects
            })
        elif cmd == LC_SYMTAB:
            symoff, nsyms, stroff, strsize = struct.unpack(
                f"{endian}IIII", slice_data[cmd_offset + 8 : cmd_offset + 24]
            )
            symtab_info = {
                "symoff": symoff,
                "nsyms": nsyms,
                "stroff": stroff,
                "strsize": strsize
            }

        cmd_offset += cmdsize

    print("\n[+] Mach-O Memory Segments (Base addresses):")
    text_base = None
    data_base = None

    for seg in segments:
        print(f"    - Segment: {seg['name']:<16} VM: 0x{seg['vmaddr']:08X} - 0x{seg['vmaddr'] + seg['vmsize']:08X} (size: 0x{seg['vmsize']:X})")
        if seg['name'] == "__TEXT":
            text_base = seg['vmaddr']
        elif seg['name'] == "__DATA":
            data_base = seg['vmaddr']

    if text_base is not None:
        print(f"\n>>> Base TEXT address: 0x{text_base:08X}")
    if data_base is not None:
        print(f">>> Base DATA address: 0x{data_base:08X}")

    # Parse Symbol Table if present
    symbols_found = []
    if symtab_info and symtab_info["nsyms"] > 0:
        print(f"\n[+] Processing Symbol Table ({symtab_info['nsyms']:,} symbols)...")
        symoff = symtab_info["symoff"]
        stroff = symtab_info["stroff"]
        strtab = slice_data[stroff : stroff + symtab_info["strsize"]]

        for i in range(symtab_info["nsyms"]):
            n_offset = symoff + i * 12
            if n_offset + 12 > len(slice_data):
                break
            n_strx, n_type, n_sect, n_desc, n_value = struct.unpack(
                f"{endian}IBBhI", slice_data[n_offset : n_offset + 12]
            )
            if n_strx < len(strtab):
                end_str = strtab.find(b"\x00", n_strx)
                if end_str != -1:
                    sname = strtab[n_strx:end_str].decode("latin1", errors="replace")
                    if sname and n_value != 0:
                        symbols_found.append((n_value, sname))

        print(f"[+] Total valid defined symbols: {len(symbols_found):,}")
        with open(out_symbols_path, "w", encoding="utf-8") as f:
            for val, sname in sorted(symbols_found, key=lambda x: x[0]):
                f.write(f"0x{val:08X} {sname}\n")
        print(f"[+] Saved symbol dump to: {out_symbols_path.name}")
    else:
        print("\n[-] Binary is stripped (no LC_SYMTAB or symbols removed).")

    print("\n[+] Searching for GTA Vice City String References...")
    vc_signatures = [
        b"CRunningScript",
        b"CTheScripts",
        b"CCheat",
        b"PANZER",
        b"ASPIRINE",
        b"PRECIOUSPROTECTION",
        b"THUGSTOOLS",
        b"PROFESSIONALTOOLS",
        b"NUTTERTOOLS",
        b"LEAVEMEALONE",
        b"BIGBANG",
        b"SEAWAYS",
        b"CTouchInterface",
        b"RsGlobal",
        b"display_fps",
        b"gMobileMenu",
        b"FindPlayerPed",
        b"CGame::Initialise",
        b"CGame::Process",
        b"text.gxt",
        b"american.gxt",
        b"Reachability",
        b"IOSReachability",
    ]

    for sig in vc_signatures:
        matches = [m.start() for m in re.finditer(re.escape(sig), slice_data)]
        if matches:
            vm_locations = []
            for m in matches:
                for seg in segments:
                    if seg["fileoff"] <= m < seg["fileoff"] + seg["filesize"]:
                        vm = seg["vmaddr"] + (m - seg["fileoff"])
                        vm_locations.append(f"0x{vm:08X}")
                        break
            loc_str = ", ".join(vm_locations[:3])
            if len(vm_locations) > 3:
                loc_str += f" (+{len(vm_locations)-3} more)"
            print(f"    [MATCH] '{sig.decode('latin1')}' found at VM: {loc_str}")
        else:
            print(f"    [ - ] '{sig.decode('latin1')}' not found directly in strings")


def main():
    script_dir = Path(__file__).resolve().parent
    target_arg = sys.argv[1] if len(sys.argv) > 1 else None

    if target_arg:
        target_path = Path(target_arg)
        if not target_path.exists():
            print(f"[-] Specified target file does not exist: {target_path}")
            return 1
    else:
        target_path = find_target_file(script_dir)

    if not target_path:
        print("[-] No executable or .ipa found in dump_and_logs_vc/")
        print("    Please copy your GTA Vice City binary (gta3vc) or .ipa here, then rerun:")
        print("    python analyze_vc.py")
        return 0

    print(f"[*] Target file: {target_path.name}")

    bin_path = target_path
    if target_path.suffix.lower() == ".ipa":
        bin_path = extract_binary_from_ipa(target_path, script_dir)

    with open(bin_path, "rb") as f:
        data = f.read()

    print(f"[*] Binary file size: {len(data):,} bytes ({len(data)/(1024*1024):.2f} MB)")

    try:
        slices = parse_macho_slices(data)
    except Exception as e:
        print(f"[-] Error parsing Mach-O: {e}")
        return 1

    target_slice = None
    for s in slices:
        if "armv7" in s["arch"]:
            target_slice = s
            break

    if not target_slice and slices:
        print("[!] No ARMv7 slice found. Checking first available slice...")
        target_slice = slices[0]

    if target_slice:
        out_syms = script_dir / f"{bin_path.name}_symbols.txt"
        analyze_32bit_slice(data, target_slice, out_syms)

    print("\n[+] Analysis complete! Next step: map hook addresses to src/lib.rs & src/game/")
    return 0


if __name__ == "__main__":
    sys.exit(main())
