# Konfiguration

## Beispiel

~~~~
// Konfiguration, Reihenfolge beliebig
config[projection] = "mercator"
config[bbox] = (12.8, 50.8, 13.0, 51.0)
config[outfile] = "map0.png"
include /* "datei1.cfg" */
  "datei2.cfg"
// Schrittbefehle, Reihenfolge wichtig
query[q1] = """
[bbox: {{box}}]
(
    way;
    >;
)
out qt
"""
draw[polygon] (
    style[basis] (stroke=#ff0000, linewidth=1.3)
    filter (("highway"=="primary" or highway=="secondary")
      and layer|0==0
    )
    id (1, 3)
)

query[q1]
// ...
~~~~

## Literatur

- [Aho et al: Compiler](https://www.pearson.de/compiler-9783863265748)
- [Nystrom: Crafting Interpreters](https://craftinginterpreters.com/)
- [Ball: Writing an Interpreter in Go](https://interpreterbook.com/)

## Tokens

- Einzelzeichen: `[ ] ( ) = , |`
- Digraph: `== !=`
- Keywords: `config query draw filter style include and or xor not`
- Identifier: Buchstaben mit _, Zahlen, 1. Zeichen Buchstabe
- Zahlen: int, float mit Dezimalpunkt, Hexadezimalzahlen mit #