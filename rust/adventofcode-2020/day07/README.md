# Advent of Code 2020 Tag 7

## Vorbereitungen

Verarbeitung der Eingabe

Paket A enthält n Pakete B, m Pakete C, ...

~~~mermaid
flowchart LR

A -->|n| B
A -->|m| C
~~~

Alternative: Graphviz 
- Paket `graphviz`
- Homepage: [graphviz.org](https://graphviz.org/)
- Dokumentation: [Dot](https://graphviz.org/pdf/dotguide.pdf)

Aufruf: `dot -T png -o demo.png demo.dot`

Aufbau Eingabe:

~~~
vibrant plum bags contain 5  faded blue bags, 6 dotted black bags.
faded   blue bags contain no other bags.
  0      1    2    3      4   5     6    7    8   9     10    11
  ^      ^                ^   ^     ^       | ^   ^      ^
~~~
- Parent: Farbe an Pos. 0-1
- Füllwörter
- Kind: Anzahl oder "no" an Pos. 4
- Kind: Farbe an Pos. 6-7
- Füllwort
- Kind: Anzahl Pos 8
- Kind: Farbe  Pos 9-10
- ...

## Aufbau Graph

- DAG (Directed Acyclic Graph)
- Kantengewichte
- Datenstruktur: Adjazenzliste
- Realisierung über [HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html)

## Teil 1

- Jeden Knoten als Start betrachtet
- (vereinfachte) Tiefensuche
- Rekursion

## Teil 2

- Target ist Startknoten s
- rekursiv für s
  - für alle Kinder von s
    - Summiere (Anzahl von s) * (1 + Inhalt von s)
    - Rückgabe Summe
