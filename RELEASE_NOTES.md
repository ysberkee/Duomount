Experimental beta of DuoMount, combining the ShadowMountPlus host with the embedded Kraken Mount FPKG runtime.

- One ELF and a shared interface on port 10101.
- Inherited folder, image and storage workflows, plus direct supported PS5 debug FPKGs.
- English and German documentation, original project credits and component notices.
- Predecessor OneMount: one user on firmware 13.60 / OnionHEN reported nine detected games and successful launch tests for multiple titles.
- The renamed build passed local build, package-format, FFI, mount-cleanup and storage-path checks. It has not yet had its own console smoke test.

Use a compatible OnionHEN/kstuff-lite environment. Stop standalone ShadowMountPlus and Kraken Mount before loading DuoMount. Existing /data/shadowmount paths are retained. The management API has no authentication; use a trusted LAN.

Only duomount.elf is required for your payload loader. The logo is an optional project asset. Source and checksums are provided separately.
