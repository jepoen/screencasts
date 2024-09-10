# Advent of Code 2020 Tag 3 Teil 1

Gegeben: Matrix mit Boolean-Werten (true = Baum, false = Lücke)

Gesucht:
- Gehe Weg von (row=0, col=0) nach (row=last_row, ?) mit Schrittweite
  (d_row = 1, d_col = 3)
- Zähle Bäume = Matrixeinträge, die true sind.

Algorithmus:
- Problem: Matrix muss nach rechts periodisch fortgesetzt werden
- Lösung:
  - Entweder Matrix auf auf notwendige Breite erweitern:
    Größe 300 * (30 * 10) = 300 * 300 = 90_000
  - Berechnen erwarteten Wert
    ~~~
    m(row, 0) == m(row, width) == m(row, 2*width) == m(row, 3*width)
    m(row, 1) == m(row, width + 1) ...
    ...
    ~~~
    m(row, col) == m(row % height, col % width)
  
