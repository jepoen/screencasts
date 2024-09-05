# Advent of Code 2020, Tag 1

- Eingabe: Liste von ganzen Zahlen
- Ausgabe: a * b, wobei a, b in der Liste stehen und a + b = 2020

Idee:

- Liste erzeugen
- Wähle a der Reihe nach aus
  - Wähle für b alle Werte, die nach a folgen
  - Wenn a + b = 2020, dann gib (a, b) zurück
- Für alle Paare a, b berechne a * b

## Teil 2

- Liste erzeugen
- Wähle a der Reihe nach aus
  - Bestimme rest = 2020 - a
  - wähle alle b nach a aus
    - wähle alle c nach b aus
    - wenn a + b + c = 2020, dann gib (a, b, c) zurück
- Für alle Tripel (a, b, c) berechne a * b * c

Laufzeit:
- 2 Schleifen verschachtelt, Laufzeit < 200 * 200 * 200 = 8_000_000
