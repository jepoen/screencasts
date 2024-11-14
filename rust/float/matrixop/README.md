# Matrix- und Vektoroperationen

## Einlesen, Anzeige, Matrix-Vektor-Multiplikation

Gegeben:
- Matrix (n×n)
- Vektor (n)

Gesucht:
- Anzeige
- Produkt Matrix · Vektor

Eingabe:
- simple_input:
  - Dimension n (ganzzahlig)
  - Matrix zeilenweise, n·n Werte (float)
  - Vektor n Werte (float)
  - Für Probe: Vektor mit n Werten (float)

Multiplikation Matrix und Vektor:

$$
\begin{pmatrix}
  a_{0,0} & a_{0,1} & a_{0,2} \\
  a_{1,0} & a_{1,1} & a_{1,2} \\
  a_{2,0} & a_{2,1} & a_{2,2}
\end{pmatrix} \cdot
\begin{pmatrix}
v_0 \\ v_1 \\ v_2
\end{pmatrix}
=
\begin{pmatrix}
r_0 \\ r_1 \\ r_2
\end{pmatrix}
$$

Berechnung $r_{i,j}$:

$$
r_i = \sum_{k=0}^{n-1} a_{i,k} \cdot v_k
$$

Zusammenfassung:

- Matrix/Vektor einlesen
- Anzeige Matrix/Vektor
- Matrix-Vektor-Multiplikation
- Differenz zwischen Vektoren