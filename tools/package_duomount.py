#!/usr/bin/env python3
"""Package the exact checked ELF plus source, excluding local SDK caches and console data."""
import hashlib
import json
import shutil
import zipfile
from pathlib import Path

root = Path(__file__).resolve().parents[1]
out = root.parent / "outputs/DuoMount/release"
out.mkdir(parents=True, exist_ok=True)
elf = root / "duomount.elf"
preflight = json.loads((root / ".build/elf-preflight.json").read_text(encoding="utf-8"))
assert hashlib.sha256(elf.read_bytes()).hexdigest() == preflight["sha256"], "ELF differs from preflight"
binary = out / "duomount.elf"
shutil.copyfile(elf, binary)
excluded_dirs = {".git", ".github", ".deps", ".build", "target", "__pycache__"}
generated = {"notify_icon_asset.c", "config_ini_example_asset.c", "web_index_asset.c", "shell_icon_param_asset.c"}
manifest = {"release": "DuoMount-beta1", "elf_sha256": preflight["sha256"], "files": []}
source = out / "DuoMount-beta1-source.zip"
with zipfile.ZipFile(source, "w", compression=zipfile.ZIP_DEFLATED, compresslevel=9) as archive:
    for path in sorted(root.rglob("*")):
        rel = path.relative_to(root)
        if not path.is_file() or excluded_dirs.intersection(rel.parts) or path.suffix in {".o", ".so", ".elf", ".log", ".pyc"} or path.name in generated:
            continue
        data = path.read_bytes()
        archive.writestr("DuoMount/" + rel.as_posix(), data)
        manifest["files"].append({"path": rel.as_posix(), "bytes": len(data), "sha256": hashlib.sha256(data).hexdigest()})
    archive.writestr("DuoMount/SOURCE-MANIFEST.json", json.dumps(manifest, indent=2))
with zipfile.ZipFile(source) as archive:
    assert archive.testzip() is None, "source archive CRC failure"
    assert "DuoMount/src/sm_fpkg.c" in archive.namelist()
    assert "DuoMount/fpkg-bridge/src/bridge.rs" in archive.namelist()
    assert "DuoMount/tools/sce_stubs/libkernel_sys.c" in archive.namelist()
shutil.copyfile(root / "README.md", out / "README.md")
shutil.copyfile(root / "LICENSE", out / "LICENSE")
shutil.copyfile(root / ".build/elf-preflight.json", out / "elf-preflight.json")
files = [binary, source]
(out / "SHA256SUMS").write_text("".join(hashlib.sha256(p.read_bytes()).hexdigest() + "  " + p.name + "\n" for p in files), encoding="utf-8")
print(json.dumps({"output": str(out), "elf_bytes": binary.stat().st_size, "source_bytes": source.stat().st_size,
                  "source_files": len(manifest["files"]), "elf_sha256": preflight["sha256"]}, indent=2))
