# Testen von Funktionen

- Unit Tests: Test einzelner Funktionen
- (Integration Tests: Test des Zusammenspiels)

## Vorgehen
- Testfunktion mit `#[test]` kennzeichnen
- Enthält Testausdrücke mit `assert!(bool-Ausdruck)`
- Übersetzen: `rustc quelltext.rs --test -o testprogramm`
- zusätzlich: `assert_eq!(ausdruck_1, ausdruck_2)`

## Prüfen von Zusicherungen in Funktionen
- `assert!()`, `assert_eq!()` am Funktionsanfang verwenden
- Werden auch bei normaler Übersetzung geprüft
- erlauben Ausgabe wie `println!()`
