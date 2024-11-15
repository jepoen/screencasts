# Advent of Code 2020, Tag 14

## Teil 1

Gegeben:
- Programm
- Folge von Masken und Speicherzuweisungen
- Maske: 36 Zeichen, 0, 1, X
- Maske wird auf Wert angewendet: Bits mit Maske 0 auf 0, mit Maske 1 auf 1  
  gesetzt, bei X unverändert

Gesucht:
- Summe aller Speicherzellen

Zusammenfassung:
- Bitmaske erzeugen: Horner-Schema
- Anwenden von Bitmasken:
  - Setze Bit:  val | mask
  - Lösche Bit: val & !mask

## Teil 2

- Adresse wird durch Maske manipuliert
- Maske Wert 1 → In addr wird dieses Bit auf 1 gesetzt
- Maske Wert X → In addr werden für dieses Bit sowohl 0 als auch 1
  gesetzt!

Das bedeutet: in Maske
- 1*X → 2 Adressen
- 2*X → 4 Adressen

Rekursiv Adresse erzeugen:

- Legen Basiswert fest
- gehen Positionen von 0 bis N durch
  - wenn Position flutet:
    - In Basiswert bit[pos] = 0 und fortsetzen
    - und bit[pos] = 1 und fortsetzen
  - pos = N: Zahl generiert