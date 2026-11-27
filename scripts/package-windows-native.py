#!/usr/bin/env python3
"""Package native Windows release artifacts for Discy:
- Portable ZIP: Discy-<version>-windows-x64.zip
- Inno Setup Installer: Discy-<version>-windows-x64-setup.exe
- Checksums: SHA256SUMS.txt
"""
import argparse
import hashlib
import os
import re
import shutil
import struct
import subprocess
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent


def read_version() -> str:
    text = (ROOT / "Cargo.toml").read_text(encoding="utf-8")
    match = re.search(r'^version\s*=\s*"([^"]+)"', text, re.MULTILINE)
    if not match:
        raise SystemExit("Could not find version in Cargo.toml")
    return match.group(1)


def find_iscc() -> Path:
    found = shutil.which("iscc") or shutil.which("ISCC")
    if found:
        return Path(found)
    candidates = [
        Path(os.environ.get("ProgramFiles(x86)", r"C:\Program Files (x86)"))
        / "Inno Setup 6"
        / "ISCC.exe",
        Path(os.environ.get("ProgramFiles", r"C:\Program Files"))
        / "Inno Setup 6"
        / "ISCC.exe",
        Path(os.environ.get("LOCALAPPDATA", ""))
        / "Programs"
        / "Inno Setup 6"
        / "ISCC.exe",
    ]
    for candidate in candidates:
        if candidate.is_file():
            return candidate
    raise SystemExit("Inno Setup 6 (ISCC.exe) not found")


def verify_pe_x64(path: Path) -> None:
    with path.open("rb") as f:
        hdr = f.read(64)
        if hdr[:2] != b"MZ":
            raise SystemExit(f"{path} is not a Windows MZ executable")
        f.seek(struct.unpack_from("<I", hdr, 60)[0])
        pe = f.read(6)
        if pe[:4] != b"PE\0\0" or struct.unpack_from("<H", pe, 4)[0] != 0x8664:
            raise SystemExit(f"{path} is not an AMD64 Windows PE executable")


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            digest.update(chunk)
    return digest.hexdigest()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--exe",
        type=Path,
        default=ROOT / "target" / "release" / "discy.exe",
        help="Path to built discy.exe (default: target/release/discy.exe)",
    )
    parser.add_argument(
        "--output",
        type=Path,
        default=ROOT / "target" / "dist" / "discord",
        help="Output directory (default: target/dist/discord)",
    )
    parser.add_argument(
        "--version",
        default=None,
        help="Version string (default: read from Cargo.toml)",
    )
    parser.add_argument(
        "--tag",
        default=None,
        help="Git release tag (default: v<version>)",
    )
    args = parser.parse_args()

    version = args.version or read_version()
    version_number = version.split("-")[0]
    tag = args.tag or f"v{version}"
    exe = args.exe.resolve()
    out_dir = args.output.resolve()
    stage_root = out_dir.parent / "staging"
    stage = stage_root / f"Discy-{version}-windows-x64"

    if not exe.is_file():
        raise SystemExit(f"Release binary not found: {exe}")
    verify_pe_x64(exe)
    iscc = find_iscc()

    if stage_root.exists():
        shutil.rmtree(stage_root)
    stage.mkdir(parents=True, exist_ok=True)
    out_dir.mkdir(parents=True, exist_ok=True)

    shutil.copy2(exe, stage / "discy.exe")
    shutil.copy2(ROOT / "LICENSE", stage / "LICENSE.txt")

    (stage / "SOURCE.txt").write_text(
        f"Discy {version} (tag {tag})\n\n"
        "Source repository:\n"
        "  https://github.com/x2super/Discy\n\n"
        "Forked from Yapfast (GPL-3.0-or-later):\n"
        "  https://github.com/yapfast/yapfast\n",
        encoding="utf-8",
        newline="\r\n",
    )

    (stage / "README.txt").write_text(
        f"Discy {version} for 64-bit Windows 10/11 (Portable)\n\n"
        "1. Extract the entire ZIP to a folder.\n"
        "2. Double-click discy.exe (or run-discy.cmd to keep console output visible on error).\n"
        "3. Link your Discord account via QR code or token.\n\n"
        "No Rust, Visual Studio, or separate runtime installation is required.\n"
        "Discy writes diagnostics to its data folder under %LOCALAPPDATA%\\discy\\Discy\\data.\n\n"
        "Discy is free software under GPL-3.0-or-later (see LICENSE.txt),\n"
        "forked from Yapfast (https://github.com/yapfast/yapfast).\n",
        encoding="utf-8",
        newline="\r\n",
    )

    (stage / "run-discy.cmd").write_text(
        "@echo off\n"
        'cd /d "%~dp0"\n'
        "discy.exe %*\n"
        "if errorlevel 1 (\n"
        "  echo Discy exited with an error.\n"
        "  pause\n"
        ")\n",
        encoding="utf-8",
        newline="\r\n",
    )

    zip_path = out_dir / f"Discy-{version}-windows-x64.zip"
    with zipfile.ZipFile(
        zip_path, "w", compression=zipfile.ZIP_DEFLATED, compresslevel=6
    ) as zf:
        for item in sorted(stage.rglob("*")):
            if item.is_file():
                arcname = f"{stage.name}/{item.relative_to(stage).as_posix()}"
                zf.write(item, arcname)

    iss = ROOT / "packaging" / "windows" / "yapfast.iss"
    cmd = [
        str(iscc),
        "/Q",
        f"/DAppVersion={version}",
        f"/DVersionNumber={version_number}",
        f"/DSourceDir={stage}",
        f"/DLicenseFile={ROOT / 'LICENSE'}",
        f"/O{out_dir}",
        str(iss),
    ]
    subprocess.run(cmd, check=True)

    setup_path = out_dir / f"Discy-{version}-windows-x64-setup.exe"
    if not setup_path.is_file():
        raise SystemExit(f"Expected installer was not created: {setup_path}")
    with setup_path.open("rb") as f:
        if f.read(2) != b"MZ":
            raise SystemExit(f"{setup_path} is not a Windows MZ executable")

    sums_path = out_dir / "SHA256SUMS.txt"
    sums_path.write_text(
        f"{sha256_file(setup_path)}  {setup_path.name}\n"
        f"{sha256_file(zip_path)}  {zip_path.name}\n",
        encoding="utf-8",
    )

    shutil.rmtree(stage_root)
    print(f"Installer: {setup_path} ({setup_path.stat().st_size} bytes)")
    print(f"Portable:  {zip_path} ({zip_path.stat().st_size} bytes)")
    print(f"Checksums: {sums_path}")


if __name__ == "__main__":
    main()
