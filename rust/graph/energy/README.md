# Bipartite Graphen / Data Frame

Beispiel: Energiemix: [Destatis](https://www.destatis.de/DE/Themen/Branchen-Unternehmen/Energie/Erzeugung/Tabellen/bruttostromerzeugung.html)

Gegeben:
- 2 Sorten Objekte: Energieart, Jahr
- Beziehungen zwischen Art und Jahr, aber nicht zwischen den Arten oder den
  Jahren

Graphisch:

~~~mermaid
flowchart LR

subgraph kinds
  wind
  gas
  braunk
end

subgraph years
  2020
  2021
  2022
  2023
end

wind -->|126| 2020
wind -->|138| 2023
braunk -->|114| 2020
braunk -->|87| 2023
gas -->|79| 2022
gas -->|80| 2023
~~~

Data Frame:
- Zeilen Bebachtungen
- Spalten Komponenten

Darstellung im Code:

- Matrix
  - Zeilen: Energiearten
  - Spalten: Jahre
  - Zellen: Energiemenge

Zuordnung zwischen Objekt und Kante:
- Suche Zeilenindex über Objektliste der Zeilen
  - Verbesserung: Iterator-Adapter: `position()`
- Suche Spaltenindex über Objektliste der Spalten
- Kante(Gewicht): Matrixeintrag\[Zeile][Spalte]
