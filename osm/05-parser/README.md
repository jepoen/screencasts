# Konfiguration

Das Parsen der Konfiguration ist in mehrere Abschnitte unterteilt.

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

## Tokens (1. Teil)

**Screencast** [Youtoube](https://youtu.be/CE_gtv5Mjow)

- Einzelzeichen: `[ ] ( ) = , |`
- Digraph: `== !=`
- Keywords: `config query draw filter style include and or xor not`
- Identifier: Buchstaben mit _, Zahlen, 1. Zeichen Buchstabe
- Zahlen: int, float mit Dezimalpunkt, Hexadezimalzahlen mit #

Definition im Paket `tokens`

## Lexer (2. und 3. Teil)

**Screencast**: [Youtube](https://youtu.be/c17gWghwUJs)

- Entfernen von Blanks und Kommentarten
- Einfache Zeichen und Digraphs
- Identifier, Schlüsselwörter, Zahlen, Strings

Definition im Paket `lexer`

## Erkennen von Zahlen, Identifier, Strings

**Screencast**: [Youtube](https://youtu.be/wtwZ2rnk3a0)

- endliche Automaten
- hier reguläre Ausdrücke:
  - Zahl: `[+-]?[0-9]+(\.[0-9]+)?`
  - Bezeichner: `[a-zA-Z_][a-zA-Z0-9_]*`

DFA für Zahlen:

![DFA für Zahlen](dfa_number.png)
