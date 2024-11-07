# Lotka-Volterra-Gleichungen

[Wikipedia](https://en.wikipedia.org/wiki/Lotka%E2%80%93Volterra_equations)

Gegeben:
- x: Anzahl Hasen, y: Anzahl Füchse
- a: Geburtsrate Hasen, b: Sterberate Hasen (abh. von Anz. der Füchse)
- c: Sterberate Füchse, d: Geburtsrate Füchse (abh. von Anz. der Hasen)

Gesucht:
- Anzahl Füchse, Hasen zu bestimmten Zeitpunkten t₀, t₁, t₂, …

Modell:

~~~
dx/dt = ax - bxy
dy/dt = -cy + dxy
~~~

*Euler-Vorwärtsverfahren*

Verfahren:
- Teilen Zeit in ganz kleine Schritte
- Berechnen die Ableitungen dx, dy
- Verändern Parameter (x, y) um dx*dt bzw. dy*dt