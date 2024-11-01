# 2D-Felder (Vec)

Problem: Beschreibung einer »Gitterwelt«: Garten, der durch Mauer
vollständig begrenzt ist, Mauer hat keine Abzweigungen.
Suchen Gartenfläche.

Gegeben (Eingabe):
- Rechteckfläche
- Dimensionen
- jede Zelle durch Zahl beschrieben:
  - 0: frei
  - 1: Mauer
  - …
  
Beispiel (beliebige Werte):
~~~
3 4 # Zeilen × Spalten
1 2 3 4 # 1. Zeile
5 6 7 8 # 2. Zeile
4 3 2 6 # 3. Zeile
~~~

Datentyp:

- Eindim. Feld: `Vec<i32>`
- Zweidim. Feld: Feld von Feldern:
  - `rows: Vec` für die Zeilen
  - Jede Zeile enthält ein `Vec`-Objekt für die Zellen (Spalten) dieser
    Zeile
    
~~~
rows: 0 -> [0, 1, 2, 3]
      1 -> [4, 5, 6, 7]
      2 -> [8, 9, 10, 11]
~~~

Zugriff:
- Über Iteratoren:
  - äußerer Iterator über Zeilen
  - innerer Iterator über Zellen/Spalten einer Zeile
- Über Index: `array[row_idx][col_idx]`

Dimensionen:
- Zeilenzahl: `array.len()`
- Spaltenzahl: `array[0].len()` (wenn Feld rechteckig)

*Anmerkung*: Die Variable `area` wurde in `size` umbenannt, um die
Namensdoppelung mit dem Funktionsnamen `area()` zu beseitigen.
