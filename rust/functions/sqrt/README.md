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
