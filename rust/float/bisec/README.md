# Bisectionsverfahren

Gegeben: Funktion
- stetig
- muss Nullstelle haben

Gesucht: Nullstelle

Algorithmus:
1. benötigen Parameter 2-er Funktionswerte, die unterschiedliches
   Vorzeichen haben x_left, x_right
2. Bestimme x_m = (x_left + x_right)/2
3. Bestimme y_m = f(x_m)
4. Wenn signum(f(x_l) == signum(f(x_m) dann x_l ← x_m, sonst x_r ← x_m
5. Wenn |x_r - x_l| klein genug, dann fertig, sonst gehe zu 2. 

Demonstration: Polynom 3-ten Grades: a₃ x³ + a₂ x² + a₁ x + a₀

Anmerkung: Float-Zahlen niemals auf Gleichheit testen, sondern
auf Differenz < EPS
