# Payload Manager icon

The Payload Manager currently shows a generic package icon for locally uploaded
payloads. Copying a PNG beside an ELF does not attach an icon to that local entry.
Repository entries obtain their icons from the source's icon catalog.

This project supplies:

- `payloads_v2.json`: the DuoMount entry and a reference to the icon catalog.
- `icons.json`: the compact embedded PNG used by the manager.
- `assets/duomount-icon.png`: the 64 × 64 icon source.

Once the repository and release are publicly reachable, add this source in
Payload Manager and install DuoMount from that source:

`https://raw.githubusercontent.com/ysberkee/Duomount/main/payloads_v2.json`

The existing local upload remains a separate entry and keeps its generic icon.
Do not run the local and repository copies together. Finish active transfers
and close games before changing your normal payload setup or autoload entries.

The catalog references the existing beta ELF and verifies its SHA-256. Adding
the catalog does not require a different DuoMount binary. Only `duomount.elf`
is needed to execute the payload; the manager handles catalog icons separately.
