# Testen von Funktionen

- Unit Tests: Test einzelner Funktionen
- (Integration Tests: Test des Zusammenspiels)

## Vorgehen
- Testfunktion mit `#[test]` kennzeichnen
- Enthält Testausdrücke mit `assert!(bool-Ausdruck)`
- Übersetzen: `rustc quelltext.rs --test -o testprogramm`
- zusätzlich: `assert_eq!(ausdruck_1, ausdruck_2)`
