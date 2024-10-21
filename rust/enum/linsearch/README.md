# Behandlung von »Ausnahmen«

Reguläre Ausnahmen

- Funktionsergebnis existiert nicht
- keine geeigneten Daten vorhanden
- keine Daten vorhanden

⇒ Ergebnis: konkreter Wert oder »Nichts«

Gegensatz: echte Fehler

- eingelesener Wert keine Zahl
- Zuviele Werte für Feld

Lösung: spezieller enum-Typ: `Option`

- zwei Varianten:
  - Some(wert)
  - None
- wert benötigt Typ
- Definition: `Option<Typ>`
- Auswertung über `match`, Zweige `Some(val)` und `None`
- Auswertung über `if let Some(val)`
- Wenn Fehler normalerweise nicht auftritt: `unwrap()` oder `expect("Text")`
