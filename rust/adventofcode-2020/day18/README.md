# Advent of Code 2020, Tag 18

## Vorbereitungen

Gegeben:
- »Rechenaufgaben«, genauer arithmetische Ausdrücke, pro Zeile
- Zahlen sind alle einstellig
- Operationen: `*` `+`
- Rechenregeln:
  - Klammervorrang
  - von links nach rechts

Gesucht:
- Ergebnis pro Ausdruck

## Grammatik

~~~
S = Expr

Expr = Term { Op Term }

Term = Num
     | "(" Expr ")"

Op = "+"
   | "*"

Num = "0" | "1" | ... | "9"
~~~

LL(1)

## Lexer

erster Schritt: Lexer
- liefert Token (Spaces werden ignoriert)
- `lah()`: nächstes Token anschauen
- `consume()`: nächstes Token verbrauchen
  (meist `match()`, aber `match` ist 
  Schlüsselwort in *Rust*)

## Parser

- jede Variable bekommt eigene Funktion
- starten mit `expr()`
- ruft `term()` und speicher Ergebnis als lhs
- solange nächstes Zeichen `+` oder `*`:
  - Operation merken
  - Term auswerten
  - Mit Operation auf lhs anwenden
- lhs zurückgeben

`term()`
- wenn `lah()` eine Ziffer liefert, dann return Ziffer
- sonst:
  - verbrauche `(`
  - rufe `expr()`
  - verbrauche `)`
  - geben expr-Result zurück