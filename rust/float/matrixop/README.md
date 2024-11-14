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
\end{pmatrix} =
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

## Lösen Lineares Gleichungssystem

Gegeben:

$$
\begin{pmatrix}
  a_{0,0} & a_{0,1} & a_{0,2} \\
  a_{1,0} & a_{1,1} & a_{1,2} \\
  a_{2,0} & a_{2,1} & a_{2,2}
\end{pmatrix} \cdot
\begin{pmatrix}
x_0 \\ x_1 \\ x_2
\end{pmatrix} =
\begin{pmatrix}
r_0 \\ r_1 \\ r_2
\end{pmatrix}
$$

Gesucht: $(x_1, x_2, x_3)^T$

Algorithmus: *Gauß-Algorithmus*

~~~
  5  7 -8  4 -1 |  81 || : 5
  4  1 -8  5  6 |  53 
 -2 -9 -6  3  2 | -71
  3  0  4 -2 -4 |  21
  7 -5  7 -4 -2 |  31

  1  a  a  a  a |  a
  4  1 -8  5  6 |  53 || -4·(1. Z.) 
 -2 -9 -6  3  2 | -71 || +2·(1. Z.)
  3  0  4 -2 -4 |  21 || -3·(1. Z.)
  7 -5  7 -4 -2 |  31 || -7·(1. Z.)

  1  a  a  a  a |   a
  0  b  b  b  b |   b
  0  c  c  c  c |   c
  0  d  d  d  d |   d
  0  e  e  e  e |   e

2. Zeile:
  1  a  a  a  a |   a
  0  b  b  b  b |   b || :b
  0  c  c  c  c |   c
  0  d  d  d  d |   d
  0  e  e  e  e |   e

  1  a  a  a  a |   a
  0  1  f  f  f |   f
  0  c  c  c  c |   c || - c·(2. Z.)
  0  d  d  d  d |   d || - d·(2. Z.)
  0  e  e  e  e |   e || - e·(2. Z.)

  1  a  a  a  a |   a
  0  1  f  f  f |   f
  0  0  g  g  g |   g
  0  0  h  h  h |   h
  0  0  i  i  i |

...

  1  a  a  a  a |  r_a
  0  1  b  b  b |  r_b
  0  0  1  c  c |  r_c
  0  0  0  1  d |  r_d
  0  0  0  0  1 |  r_e

x₄ = r_e
x₃ = r_d - d·x₄
~~~

Problem:
- Auf der Diagonale entsteht eine Null (müssten dann durch 0 teilen!)

## Gauß-Verfahren mit Pivotzeile

Lösung:
- Vertauschen diese Zeile mit einer anderen Folgezeile, die hier keine Null
enthält.
- Wählen die Zeilen mit betragsmäßig größtem Element
- dann normales Gauß-Verfahren
