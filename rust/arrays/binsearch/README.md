# Binäre Suche

Gegeben:

- *sortiertes* Feld
- Suchwerte

Gesucht:

- Position des Suchwerts oder MAX_COUNT

Beispiel:

~~~
Suchwert: 13

Idx:  0  1  2  3  4  5  6  7  8  9 10
Val:  1  2  4  7  9 11 11 13 15 21 42
                     ^ < 13
     ----------------| 11 13 15 21 42
                              ^ >13
                       11 13  |------
                        ^ < 13
                        | 13  |
~~~

Algorithmus:
- Ergänzen Feld virtuell um 2 Einträge:
  - a[-1] = -infinity
  - a[N]  = infinity
- Bedingung erfüllt: a[-1] < suchwert ≤ a[N]
- idx_links ← 0, idx_rechts ← N \

  (a[idx_links-1] < suchwert ≤ a[idx_rechts])

- rekursiv:
  ~~~
  if idx_links == idx_rechts:
    return idx_rechts
  m ← (idx_links + idx_rechts)/2
  if a[m] < suchwert:
    idx_links ← m + 1
  else: // a[m] ≥ suchwert
    idx_rechts ← m  
  ~~~
- Prüfe `idx_rechts`
  - < N: a[idx_rechts] ist der erste Wert ≥ suchwert
  - = N: nicht gefunden, alle Werte im Feld sind kleiner als Suchwert
