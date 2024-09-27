# Advent of Code 2020 Tag 10

## Teil 1

Gegeben:
- Menge von Adaptern (Zahlen)
- Zusätzlich:
  - Gerät (Wert 0) (a[0])
  - Versorgung (a[n+1], Wert: max{adapter} + 3)

Gesucht: Reihenfolge der Adapter
- a[k+1] = a[k] + diff, diff in {1, 2, 3}
- Alle Adapter verwenden

Lösung: Sortieren!
- Prüfen, ob zulässig

## Teil 2

Gesucht: alle möglichen Adapterketten.
Vorgesetzt ist Teil 1, Adapter sortiert.

~~~
a[0] < a[1] < a[2] < ... < a[n] < a[n+1]
~~~

Alle möglichen Ketten suchen.

Varianten:
- Alle möglichen Ketten bilden, zulässige auswählen
  - systematisch: alle Teilmengen = Binärzahlen der Ziffernzahl n
  - Anzahl: 2^92 ≈ 10^30 -> zu langsam!!
- Backtracking
  - Varianten systematisch probieren und Zulässigkeit sofort testen, sonst abbrechen
  - alle gültigen Varianten erzeugen > 1 Mrd. -> zu langsam!

~~~
           ↓                           ↓
values:    0  1  4  5  6  7  10 11 12 15
variants:  1  1  1  1  2  4   4  4  
                              +  +  8  8
~~~

Idee [Dynamische Programmierung](https://en.wikipedia.org/wiki/Dynamic_programming)

- Suche v(k)
- Annahme: v(0), ... v(k-1) bekannt
- Formel: Bellman-Gleichung
$$v(k) = \sum_{m < k, a[k]-a[m] \le 3} v(m)$$

Vorgehen:
~~~~
           ↓                           ↓
values:    0  1  4  5  6  7  10 11 12 15
start(0)   1
idx 1      +  1
    2         +  1
    3            +  1
    4            +  +  2
    5            +  +  +  4
    6                     +   4
    7                         +  4
    8                         +  +  8
    9                               +  8            
~~~