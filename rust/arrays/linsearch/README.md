# Arrays (Felder) fester Größe

Gegeben: Zahlenfolge positiver Zahlen, Abschluss mit -1

z. B. `1 3 11 7 2 4 9 8 2 3 1 -1`

Folge von Suchwerten: `1 2 23 4 -1`

Gesucht: Ist eine gesuchte Zahl in dieser Folge vorhanden, und wenn ja,
an welcher Position? (nur erste Position)

Problem: Folge von Werten speichern für mehrfache Verarbeitung!

Lösung: Feld/Array

- Feste Größe: Muss zur Entwicklungszeit feststehen!
- merken uns genutzten Anteil
- Zugriff: Index → Wert


Besonderheit: neuer Datentyp usize

## Feld als Referenz übergeben

- Umkopieren großer Felder ist »teuer«
- passiert bei Funktionsruf
- statt dessen Referenz übergeben: `&array`
- im Funktionskopf ist Referenzfeldtyp: `&[i32]`
- im Zugriff auf Elemente ändert sich nichts

