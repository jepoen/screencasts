# Advent of Code 2020, Tag 18

## Vorbereitungen

Gegeben:
- 쨩Rechenaufgaben짬, genauer arithmetische Ausdr체cke, pro Zeile
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
- `lah()`: n채chstes Token anschauen
- `consume()`: n채chstes Token verbrauchen
  (meist `match()`, aber `match` ist 
  Schl체sselwort in *Rust*)

## Parser

- jede Variable bekommt eigene Funktion
- starten mit `expr()`
- ruft `term()` und speicher Ergebnis als lhs
- solange n채chstes Zeichen `+` oder `*`:
  - Operation merken
  - Term auswerten
  - Mit Operation auf lhs anwenden
- lhs zur체ckgeben

`term()`
- wenn `lah()` eine Ziffer liefert, dann return Ziffer
- sonst:
  - verbrauche `(`
  - rufe `expr()`
  - verbrauche `)`
  - geben expr-Result zur체ck

## Teil 2

Neue Grammatik (Op entf홯lt nun)

~~~
S = Expr

Expr = Factor { "*"  Factor }

Factor = Term { "+" Term }

Term = Num
     | "(" Expr ")"

Num = "0" | "1" | ... | "9"
~~~

- `Token` anpassen
- `expr()` anpassen
- neue Funktion `factor()` im Parser
