# Advent of Code 2020 Tag 6 Teil 1

## Teil 1

Gegeben:

- Gruppen von Zeilen
- Zeile enthält Buchstaben aus {a, ..., z} ohne Leerzeichen
- Gruppen durch Leerzeile getrennt

Gesucht:

- Anzahl der verschiedenen Buchstaben jeder Gruppe
- davon die Summe

Algorithmus:

- Histogramm (-> [u32; 26])
- Menge (-> HashSet)

## Teil 2

Gesucht:
- Anzahl identischer Buchstaben in der Gruppe für jede Person

Algorithmus:

- Histogramm
- Mengendurchschnitt der Antwortmengen jeder Person (HashSet::intersection())
