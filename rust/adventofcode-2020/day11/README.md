# Advent of Code 2020, Tag 11

## Teil 1

Gegeben:
- Matrix aus (freien) Plätzen und Gang
- Regel nächster Zeitschritt:
  - leerer Platz wird belegt bei 0 Nachbarn
  - voller Platz wird frei bei ≥ 4 Nachbarn
  - 8 Nachbarn

Gesucht:
- Anzahl der belegten Plätze, sobald sich nichts mehr ändert

Algorithmus

- zellulärer Automat
- Game of Life

Wichtig: Zustand einer Zelle darf erst geändert werden, wenn Zustand aller
neuen Zellen bekannt ist

Realisierung:
- zweites Feld für neuen Zustand
- Enums mit Werten (Änderung im nächsten Schritt)

## Teil 2

- nicht nur direkte Nachbarn, sondern der 1. Platz in der jeweiligen Richtung
- statt 4 nun 5 Nachbarn bei belegten Plätzen führen zum Aufstehen

Absuchen einer Richtung
- Start bei r + dr, c + dc
- immer dr, dc addieren
- 8 Richtungen:
    (-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1)