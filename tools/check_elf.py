#!/usr/bin/env python3
"""Structural preflight only; a PS5 launch is still required."""
import hashlib
import json
import struct
from pathlib import Path

path = Path(__file__).resolve().parents[1] / "duomount.elf"
data = path.read_bytes()
header = struct.unpack_from("<16sHHIQQQIHHHHHH", data)
assert header[0][:6] == b"\x7fELF\x02\x01", "not little-endian ELF64"
assert header[1] in (2, 3) and header[2] == 62, "wrong executable type or machine"
phoff, phsize, phcount = header[5], header[9], header[10]
assert phsize == 56 and 0 < phcount < 128
programs = [struct.unpack_from("<IIQQQQQQ", data, phoff + i * phsize) for i in range(phcount)]
loads = [p for p in programs if p[0] == 1]
assert loads and any(p[1] & 1 and p[3] <= header[4] < p[3] + p[6] for p in loads), "entry is not executable"
assert not any(p[0] in (3, 7) for p in programs), "unexpected interpreter or native TLS segment"
for p in loads:
    assert p[2] + p[5] <= len(data) and p[5] <= p[6]

def file_offset(address):
    for p in loads:
        if p[3] <= address < p[3] + p[5]:
            return p[2] + address - p[3]
    raise ValueError("dynamic string table is outside loaded data")

dynamic = next(p for p in programs if p[0] == 2)
entries = [struct.unpack_from("<qQ", data, i) for i in range(dynamic[2], dynamic[2] + dynamic[5], 16)]
strings = file_offset(next(value for tag, value in entries if tag == 5))
needed = []
for tag, value in entries:
    if tag == 1:
        start = strings + value
        needed.append(data[start:data.index(0, start)].decode("ascii"))
assert all("GLIBC" not in s and ".so." not in s for s in needed), "host libraries leaked into payload"
report = {"file":str(path), "bytes":len(data), "sha256":hashlib.sha256(data).hexdigest(),
          "machine":"x86_64", "elf_type":header[1], "native_tls":False, "needed":needed,
          "note":"Structural preflight passed; runtime verification requires the PS5."}
(path.parent / ".build/elf-preflight.json").write_text(json.dumps(report, indent=2), encoding="utf-8")
print(json.dumps(report, indent=2))
