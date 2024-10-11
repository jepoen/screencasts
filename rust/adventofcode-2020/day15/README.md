# Advent of Code 2020 Tag 15

## Teil 1

~~~
0 1 2 3 4 5 6 7 8 9
0 3 6
    ^ 0
^     ^ 3
  ^     ^ 3
        ^ ^ 1
            ^ 0
      ^       ^ 4
                ^ 0
~~~

Gegeben: Startfolge von Zahlen (nichtnegativ)

Gesucht: Folgewert:

- Betrachten letzte Zahl \
  - Wenn schon vorhanden, dann: neuer Wert = Abstand vom vorigen
    Auftreten zur letzten Position
  - sonst 0
- Wiederholen bis Länge DURATION (= 2020)

## Algorithmus

Variante 1:
- Feld der Größe 2020
- Startwerte übernehmen
- Feld positionsweise füllen
- Problem: Wert der letzten Position nach links im Feld suchen
- Aufwand N ≈ 2000: 2000 · 2000 ≈ 4 Mio.

Schneller Feldzugriff: Position → Wert

Aber: Wert → Position (Werte nicht fortlaufend)

Daraus ergibt

Variante 2:

- Feld der Größe MAX_VALUE = 2020,
  Index = Wert, Zellinhalt = letzte Position, sonst -1 nicht vorhanden
- Datentyp für Dictionary Key → Wert

Algorithmus

~~~
dict ← (Startwerte ohne letzten, Pos)
prev_val ← letzter Startwert
pos ← len(Startwerte) - 1
forall steps:
  if prev_val in dict:
    new_val ← pos - dict[prev_val]
  else:
    new_val ← 0
  print(new_val)
  dict ← (prev_val, pos)
  prev_val ← new_val
  pos ← pos + 1
print(prev_val)
~~~
