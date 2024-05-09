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

## Filterausdrücke

**Screencast** [Youtube](https://youtu.be/1zcn671EVX8)

Ausdruck:
~~~
draw[way_line] (
      filter (higway=='primary' and tunnel|'no' != 'yes')
)
~~~
AST:
~~~mermaid
graph TD
and --> eq1[eq]
eq1 --> key1[key]
key1 --> hw1[highway]
eq1 --> v1["'primary'"]
and --> ne
ne --> key2[keyOrDefault]
key2 --> tunnel
key2 --> v2["'no'"]
ne --> v3["'yes'"]
~~~
Tags:
| tag     | value      |
| ------- | -------    |
| highway | primary    |
| name    | Bergstraße |
| speed:limit| 30 |

## Verarbeiten der Draw Steps

- einfache Linienzüge
- einfache Polygone
- Schließen von »Rundwegen«
- Style wird noch nicht ausgewertet

**Screencast** [Youtube](https://youtu.be/W1sHKmFOi6g)

## Umwandlung der Value Nodes in konkrete Datentypen

- Extrahieren des Zahlen- bzw. Farbwerts aus `ValueNode`-Objekten

**Screencast** [Youtube](https://youtu.be/t5dVhmoy9R0)

## Auswerten und Anwenden der Style-Optionen

**Screencast** [Youtube](https://youtu.be/Vt_7L2Q4zck)

## Styletest

**Screencast** [Youtube](https://youtu.be/-xywMhjM1j0)

Offene Issues:

- Dateiname und Zeilennummer in AST-Knoten ergänzen
- Prüfen der Vollständigkeit der Environment

## Zusammenfassung und Tests

Issues:

- Crash, wenn config-Einstellungen fehlen
- Dateinamen, Zeilennummer in AST-Knoten
- draw: fehlender Filter: alle Elemene wählen
- bessere Fehlermeldungen
- log-Ausgabe steps
- query: Fehler Duplicate keys für wiederholte Abfragen

Erweiterungen:

- Zusammenfügen von Linien
- Multipolygone