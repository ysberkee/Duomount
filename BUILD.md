# DuoMount beta 1

Ein gemeinsamer PS5-Payload: vollständige ShadowMountPlus-Basis mit eingebautem PS5-Debug-FPKG-Leser und Launch-Runtime aus Kraken Mount. Es wird **eine `duomount.elf`** gestartet. Ein separater Kraken-Payload oder zweiter Webserver wird dabei nicht gestartet.

Die gewohnte ShadowMountPlus-Oberfläche, Einstellungen, Bibliothek und Funktionen für Ordner und Images bleiben erhalten. Die Oberfläche kennzeichnet diesen eigenen Build als DuoMount. Originalautoren, Credits, Lizenzen und Supportlinks bleiben erhalten. Dies ist ein eigener Fork, keine offizielle Veröffentlichung der beiden Projekte.

## Benutzung

1. Wie bisher OnionHEN und das passende kstuff starten.
2. Alte ShadowMountPlus- und Kraken-Instanzen beenden und `duomount.elf` laden.
3. Spiele normal über ihre Kachel starten. Die vorhandene ShadowMount-Kachel öffnet die gemeinsame Oberfläche, standardmäßig auf Port **10101**.

Die bestehenden Scanpfade und `/data/shadowmount/config.ini` werden übernommen. Dazu gehören `/data/homebrew` und die interne M.2 unter `/mnt/ext1/homebrew`. Ein vollständiges FPKG auf internem Speicher benötigt keine angeschlossene USB-SSD. Eine auf USB verbliebene Quelldatei benötigt diese USB-SSD weiterhin.

Zusätzlich zu den ursprünglichen Ordner-/Imageformaten werden vollständige **PS5-Debug-FPKGs mit FIH-Header und PPSA-Titelkennung** unterstützt. Die große PKG bleibt an ihrem Speicherort. Nur Metadaten, ausführbare Dateien, Bibliotheken und der Zugriffsindex werden unter `/data/shadowmount/fpkg/caches/<PPSA>/` vorbereitet.

Retail-PKGs, LIH-Patches und unvollständige Übertragungen sind keine unterstützten FPKG-Quellen. PS4-PKGs werden weiterhin über den bestehenden Paketinstaller installiert. Die neue direkte Mount-Funktion ist für PS5-FPKGs gedacht.

## Stand der Prüfung

- ELF vollständig mit dem PS5 SDK gebaut; strukturelle ELF-Prüfung bestanden.
- FreeBSD-Prüfung der eingebetteten Rust-Bibliothek ohne Fehler oder Warnungen.
- Format-/Teiltransferprüfung mit ASan/UBSan, drei FFI-Tests, fünf Paketleser-Tests und die bestehenden Shadow-Tests für Scanrechte, Sandbox-Cleanup und Speicherpfade bestanden.
- Auf der PS5 mit **Firmware 13.60 / OnionHEN** gestartet. Sieben verwaltete Titel gemeinsam erkannt: Ordner, exFAT, FFPFSC und drei FPKGs.
- Mounten und Freigeben von zwei FPKGs, einem exFAT-Image, einem FFPFSC-Container und einem Ordner auf derselben laufenden Instanz erfolgreich geprüft. Ausgelieferte Weboberfläche entspricht dem eingebetteten Quelltext.
- Der tatsächliche Start bis ins Spiel/Hauptmenü wird getrennt durch den Konsolentest bestätigt. Das Mount-Ergebnis allein bestätigt noch keinen erfolgreichen Spielstart.

FPKGs haben einen eigenen, erforderlichen AMPR-Laufzeitpfad. Deshalb ist bei diesen Titeln der native Fakelib-Aus-Schalter nicht verfügbar. Das Entpacken eines FPKGs über die Image-Entpackfunktion ist gesperrt: Der Cache enthält kein vollständig entpacktes Spiel. Die ursprüngliche Entpackfunktion für unterstützte Images bleibt erhalten. Einstellungen und Aktionen sind ansonsten Teil der unveränderten Shadow-Basis; eine Aussage über jedes Spiel, DLC oder jede Firmware ist damit nicht verbunden.

## Quellcode und Herkunft

| Bestandteil | Repository / Stand |
| --- | --- |
| ShadowMountPlus-Basis | [drakmor/shadowMountPlus](https://github.com/drakmor/shadowMountPlus), `0b56a1c25e8f07584135fd537ce49736562d6b07` |
| Eingebetteter FPKG-Teil | [quer3q/kraken-mount](https://github.com/quer3q/kraken-mount), `e1e784560fe3a56e3b5ffd579f47ed5b2cf194f7` |
| SDK-Syscall-Stub | [ps5-payload-dev/sdk](https://github.com/ps5-payload-dev/sdk), `4122b714c7e7a9449d403812b9d640bf7a55af3f` |
| Native Abhängigkeitsrezepte | [ps5-payload-dev/pacbrew-repo](https://github.com/ps5-payload-dev/pacbrew-repo), `722187f0ee246c0d7d57401c8a97de1c7f4e2ff0` |

Die ursprüngliche Shadow-Dokumentation liegt in [docs/ShadowMountPlus-upstream.md](docs/ShadowMountPlus-upstream.md). Der Vertrag für die eingebettete Bibliothek steht in [fpkg-bridge/DESIGN.md](fpkg-bridge/DESIGN.md). Die originale Kraken-Dokumentation bleibt als `README.upstream.md` und `DESIGN.upstream.md` erhalten. Ihr eigenständiger Payload-Build ist nicht der DuoMount-Build.

GPL-3.0 bzw. GPL-3.0-or-later gemäß den jeweiligen Originaldateien. Siehe [LICENSE](LICENSE), [fpkg-bridge/THIRD_PARTY_NOTICES](fpkg-bridge/THIRD_PARTY_NOTICES) und [licenses](licenses). Die gebündelten Runtime-/Emulator-Releases behalten ihre Originalhinweise und Prüfsummen.

## Bauen

Der getestete Build läuft unter Ubuntu/WSL mit PS5 SDK **v0.43** in `/opt/ps5-payload-sdk`, LLVM **19** und Rust **1.99.0** einschließlich `rust-src` und FreeBSD-Target. Die taskeigene Rust-Installation liegt standardmäßig in `/opt/kraken-unified-build/{cargo,rustup}`; diese Verzeichnisbezeichnung ist ausschließlich ein lokaler Toolchain-Pfad. Die Shell-Skripte können über `RUSTUP_HOME`/`CARGO_HOME` auf eine andere Installation zeigen.

SDK-Archiv: [v0.43](https://github.com/ps5-payload-dev/sdk/releases/download/v0.43/ps5-payload-sdk.zip), SHA-256 `a9cc9929f21b2b2c5d5b309f3bab4997067c45281c0622cf4838b1aecba66fcb`.

Benötigte Host-Werkzeuge: `clang-19 lld-19 llvm-19 make cmake build-essential curl unzip git xxd autoconf automake libtool pkg-config nasm python3`.

```sh
# Einmalig mit der vorgesehenen Rust-Installation:
rustup toolchain install 1.99.0 --profile minimal --component rust-src
rustup target add --toolchain 1.99.0 x86_64-unknown-freebsd
RUSTUP_TOOLCHAIN=1.99.0 python3 fpkg-bridge/ps5/prepare-rust-std.py
RUSTUP_TOOLCHAIN=1.99.0 cargo fetch --locked --manifest-path fpkg-bridge/Cargo.toml

# Native Bibliotheken (benötigt Schreibrechte im installierten SDK):
bash tools/build_native_deps.sh

# Eingebettete Bibliothek, ABI-Shims und gesamter C-Payload:
bash tools/build_duomount.sh
bash tools/check_duomount.sh
python3 tools/check_elf.py
```

`build_native_deps.sh` baut openlibm, zlib, libpng, libmicrohttpd, libjpeg-turbo, SQLite und libsmb2 aus den gepinnten Rezepten. SQLite verwendet das offizielle 3.46.1-Amalgamation mit geprüfter SHA-256. JSON-C wird durch den bestehenden Shadow-Build vorbereitet. Die historischen CI-Workflows der Originalprojekte richten diese kombinierte Buildumgebung nicht ein.

## Sicherung und Rückweg

Die Konsolenmigration wird separat protokolliert. Originale Payloads, Konfiguration, Mount-Links und eine lokal auf Integrität geprüfte Kopie der App-Datenbank wurden vor dem Test gesichert. Alte Kraken-Caches werden nicht gelöscht, da sie auch vom Spiel geschriebene Dateien enthalten können.

Vorübergehend ist beim ersten Test das automatische Entfernen fehlender Titel ausgeschaltet. Dauerhafter Autostart und Entfernung der alten Kraken-Verknüpfung sind eigene Migrationsschritte nach dem Spielstarttest. Ein Rückweg muss zuerst DuoMount bei geschlossenem Spiel sauber stoppen; anschließend können die gesicherten Startpfade, Einstellungen und bisherigen Payloads wieder verwendet werden. Die App-Datenbank wird nicht automatisch zurückgeschrieben.
