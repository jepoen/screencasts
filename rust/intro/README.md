# Notizen zu Installation und Programmerstellung

- Rust installieren: [rust-lang.org](https://www.rust-lang.org)
- Geany installieren: `sudo apt install geany`
- Arbeitsverzeichhnis erstellen:
  ~~~
  mkdir rust-course
  cd rust-course
  mkdir intro
  cd intro
  ~~~
- Quelltext erstellen: `geany hello1.rs`
- Anzeige Quelltext:
  - `ls -l`
  - `cat hello1.rs`
  - `less hello1.rs`
- Übersetzen Quelltext: `rustc hello1.rs`
- Kontrollanzeige: `ls -l`
- Ausführen: `./hello1`

## VSCodium

- Installation nach [vscodium.com](https://vscodium.com)
- Wechsel ins Projektverzeichnis
- Start: `codium -a .`
- Installation der Extension `rust-analyzer`
- Konsole öffnen mit `Strg` `J`
