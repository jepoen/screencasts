# Einfache Zeichenoperationen

Dieses Kapitel ist in mehrere Abschnitte unterteilt.

## Offene Punkte

- Tokens ergänzt (~, and, or, xor, not, in)

### Beispielkonfiguration

~~~~
// Konfiguration, Reihenfolge beliebig
config[projection] = "mercator"
// ...
// Schrittbefehle, Reihenfolge wichtig
query[q1] = "..."
draw[polygon] (
    style[basis] (stroke=#ff0000, linewidth=1.3)
    filter (("highway"=="primary" or highway=="secondary")
      and layer|0==0
    )
    id (1, 3)
)
// ...
~~~~

### Grammatik der DrawSteps

Meta-Ausdrücke:

- `( )?`: wahlweise
- `( )*`: beliebig oft
- `//`: Kommentar bis Zeilenende

~~~
ValueTuple = "(" Value ( "," Value )* ")"

Value = STRING
      | INT
      | FLOAT
      | HEX

DrawStep = "draw" "[" ID "]" "(" ( DrawParam )* ")"

DrawParam = DrawStyle
          | TagFilter
          | IdFilter

DrawStyle = "style" ( "[" ID "]" )? "(" KV ("," KV)* ")"

KV = ID "=" ValueOrTuple

ValueOrTuple = Value
             | ValueTuple

TagFilter = "filter" "(" FilterExpr ")"

FilterExpr = TagTerm ( BinOp TagTerm )*  // liefert bool-Wert

TagTerm = TagCmp                         // liefert bool-Wert
        | "not" TagTerm
        | "(" FilterExpr ")"

TagCmp = TagOrDefault ( CmpValue )?      // liefert bool-Wert

TagOrDefault = ID ( "|" Value )?         // liefert string-Wert

CmpValue = CmpOp Value
         | "in" ValueTuple

CmpOp = "=="
      | "!="

BinOp = "and"
      | "or"

IdFilter = "osmId" ValueTuple // nur int-Werte erlaubt
~~~

### Reihenfolge der Verarbeitung

~~~
a and b or c
~~~

Falsch:
~~~mermaid
flowchart TB

and --> a
and --> or
or --> b
or --> c
~~~
Richtig:
~~~mermaid
flowchart TB

or --> and
and --> a
and --> b
or --> c
~~~

## Parsen des Draw-Steps (Teil 1 und 2)

- AST-Definition: *Screencast* [Youtube](https://youtu.be/I9wie_7KWCg)
- Parser: *Screencast* [Youtube](https://youtu.be/O_pRFMpPn4c)

Korrekturen:

- Leere Styleoptionen (Schlüssel ohne Wert) lassen wir vorläufig nicht zu.
- in `parseDraw()` fehlt der Fehlerfall im Video
