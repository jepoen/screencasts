# Berechnung »ganzzahlige« Quadratwurzel

- Gegeben: a ≥ 0, ganzzahlig
- Gesucht: x ≈ √a, ganzzahlig

Genauer:
- x² soll möglichst nahe an a liegen, also |a - x²| soll minimal sein.

Wissen: x ≥ 0, x ≤ a (x < a, wenn a > 1)

Teilproblem: suchen x so, dass x² ≤ a < (x+1)²

Algorithmus Teilproblem:

1. x ← 0 (x² ≤ a garantiert)
2. Testen (x+1)² > a
   - falls ja: gib x zurück
   - falls nein: x ← x+1 und gehe zu 2.
   
Gesamtproblem:
- Fehler e_0 ← a - x²
- Fehler e_1 ← (x+1)² - a // beides nichtnegativ
- Wenn e_0 < e_1 dann x sonst x + 1 ist Lösung

Graphik:
- Aufruf: `./sqrt_naive_table 1000 > sqrt_table.dat`
- Gnuplot: `sudo apt install gnuplot-x11`
- Aufruf:
  - `gnuplot`
  - `plot 'sqrt_table.dat' with lines, sqrt(x)`
  
Zeitmessung:
- Aufruf `time ./sqrt_naive_table 40000 >/dev/null`

Aufwand:
a      Schritte
10        4
100      11
1000     33
1 Mio  1001

## Bisektionsverfahren

Verfahren: Nullstelle von f(x) suchen
- f(x) muss stetig sein
- zwei Werte x_l, x_r, f(x_l) ≤ 0 < f(x_r)

Bei uns:
- x = √a | Quadrieren
- x² = a | -a
- x² - a = 0

Beispiel a=10 → Gnuplot-Animation `gnuplot bisect.gnuplot`

Beispiel a=80

~~~
x_l   x_r  x_m  f(x_m)
  0    81
  0    81   40   1520 (> 0)   x_r ← x_m
  0    40   20    320 (> 0)   x_r ← x_m
  0    20   10     20 (> 0)   x_r ← x_m
  0    10    5    -55 (< 0)   x_l ← x_m
  5    10    7    -21 (< 0)   x_l ← x_m
  7    10    8    -16 (< 0)   x_l ← x_m
  8    10    9      1 (> 0)   x_r ← x_m
  8     9   (8) -> Stop
~~~

Algorithmus:

1. Wählen x_l < x_r so, dass f(x_l) ≤ 0 < f(x_r)
2. x_m ← (x_l + x_r)/2
3. wenn f(x_m) > 0
   - dann ersetze x_r ← x_m und gehe zu 2.
   - sonst ersetze x_l ← x_m und gehe zu 2.
