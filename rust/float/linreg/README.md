# Lineare Regression

Gegeben: n Beobachtungen (x, y), z. B. (Strecke, Fahrzeit), (Jahr, Temperatur)

Vermutung: Linearer Zusammenhang: y = a x + b

Gesucht: a, b, so dass der »Fehler« möglichst klein wird.

Modellvorhersage:
$$
\hat{y} = ax + b
$$
Fehler soll möglichst klein sein
(absoluter Fehler):
$$
\sum_{i=1}^n |\hat{y}_i - y_i|
$$
Abwandlung (quadratischer Fehler):
$$
\sum_{i=1}^n (\hat{y}_i - y_i)^2 \rightarrow \text{min}
$$
also:
$$
\sum_{i=1}^n (ax_i + b - y_i)^2 \rightarrow_{a,b} \text{min}
$$
Vorgehen:
- Partielle Ableitung nach a und b
- setzen diese mit 0 gleich

Ergebnis:
$$
\begin{align*}
a &= \frac{n[XY] - [X][Y]}{n[X²] - [X][X]} \\
b &= \frac{[Y]- a[X]}{n}
\end{align*}
$$
Dabei bedeutet:
$$
[X] = \sum_{i=1}^n x_i\\
[XY] = \sum_{i=1}^n x_i y_i
$$

Komplette Herleitung [PDF](linreg.pdf)

## Beispiel

Durchschnittstemperaturen 1881–2023

[DWD](https://www.dwd.de/DE/leistungen/zeitreihen/zeitreihen.html)