# Funktion: Polynomberechnung

Funktion:

~~~
y = f(x, a₃, a₂, a₁, a₀) = a₃ x³ + a₂ x² + a₁ x + a₀
~~~

Funktion: Abbildung Definitionsbereich in den Wertebereich

x aus D -> f(x) aus W

Signatur:

- Name der Funktion
- Typ des Definitionsbereichs
- Typ des Wertebereichs

Zusätzlich (Kommunikation mit dem Betriebssystem):

- Funktion mit leeren Wertebereich
- Funktion mit leeren Definitionsbereich

## Neues Problem (Rekursion)

Ziel: nicht nur einen Funktionswert berechnen, sondern Tabelle von 0 bis x=n

Algorithmus polynom_table(x_max, A) (A = a₃, a₂, a₁, a₀)

Wir wissen, wir wir polynom(x_max, A) berechnen.

Vorgehen:
- Delegieren: polynom_table(x_max-1, A)
- Erhalten Tabelle 0 ... x_max-1
- Berechnen und Ergänzen: polynom3(x_max, A)
  
- Delegieren fortsetzen: polynom_table(9, A), polynom_table(8, A),
  ... polynom_table(0, A)
- polynom_table(0, A) delegiert polynom_table(-1, A) → nichts mehr zu tun

~~~
polynom_table(x_max, A):
  wenn x_max < 0:
    fertig
  sonst:
    delegiere polynom_table(x_max-1, A)
    berechne und gib aus: polynom3(x_max, A)
~~~

Allgemein:

- Abbruch: Problem ist leer
- Zerlege Problem in einfacheres Problem und Trivialproblem
- Delegiere einfaches Problem nach gleichem Verfahren
- Löse Trivialproblem

Programmaufruf zur Ausgabe der Tabelle:

~~~
./polynom-table 10 2 -25 89 -84 >polynom-table.out
~~~

Anzeige mit `gnuplot` (zum Vergleich exakte Funktion)
~~~
plot 'polynom-table.out' with lines, 2*x**3 - 25*x**2 + 89*x - 84
