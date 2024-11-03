# Binomialkoeffizient

Anwendung:
- Binomische Formel: z. B. \
  $(a + b)^3 = a^3 + 3a^2b + 3 ab^2 + b^3$
- Anzahl der Möglichkeiten, aus n Elementen k auszuwählen (ohne Wiederholung und Beachtung der Reihenfolge) $n \choose k$

Berechnung:

- Direkte Formel:

$$
{n \choose k} = \frac{n!}{k!(n-k)!}
$$

- Rekursive Formel:

$$
\begin{align*}
{n \choose k} &= {n-1 \choose k-1} + {n-1 \choose k} \\
{n \choose 0} &= 1 \\
{n \choose n} &= 1 \\
\end{align*}
$$

- Wenn $n < 0$, $k < 0$ oder $k > n$ dann:

$$
{n \choose k} = 0
$$

Gegeben: n und k
Gesucht: $n \choose k$ = binomial(n, k)

## Direkte Berechnung:
- Zwischenergebnisse (Zähler) sehr groß
- Verbesserung: (n-k)! aus dem Zähler kürzen

## Rekursive Berechnung
- sehr aufwendig

## Pascalsche Dreieck

Zeilenweise alle Funktionswerte auf (Zeile = n)

~~~
n
0         1
1       1   1
2     1   2   1
3   1   3   3   1
4 1   4   6   4   1
~~~

- Beschleunigt Berechnung.
- Alternative: Zwischenergebnisse »cachen«
