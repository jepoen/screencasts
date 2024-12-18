# Graph - Grundlagen

Definition:

G = (V, E)

- V: Knotenmenge (Vertex, Node), endliche viele
- E: Kantenmenge (Edge), Kante verbindet zwei verschiedene Knoten (v₁, v₂)
- Knoten: Nummern
- Kante: Paar aus 2 Knotennummern

Beispiel:
~~~
v0 -- v1 -- v3
|     |
v2 ---+

v4 -- v5
~~~

## Modellierung:

Adjazenzmatrix (Adjacency matrix)

~~~
       nach
         0  1  2  3  4  5
von 0    -  x  x
    1    x  -  x
    2    x  x  -
    3             ...
    4
    5
~~~

## Graph visualisieren

- Mermaid (Plug-in für VS Codium, Github)
- [Graphviz](https://graphviz.org/)

## Statistik

- Anzahl Nachbarn: Zeile der Adjazenzmatrix: true-Werte zählen
- Nachbarn: Zeile der Adjazenzmatrix: Spalten-Indices der true-Werte
- Anzahl Kanten: Anzahl der true-Werte im oberen (bzw. unteren) Dreieck