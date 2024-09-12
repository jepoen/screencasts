# Advent of Code 2020 Tag 5

## Teil 1

### Codierung der Spalte

~~~
 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7
       L       |       R
   LL  |   LR  #   RL  |   RR
LLL|LLR#LRL|LRR#RLL|RLR#RRL|RRR
000|001|010|011|100|101|110|111
~~~
### Umrechnung Binärdarstellung in Zahlenwert

Dezimal:

431 = 4·10² + 3·10¹ + 1·10⁰

Binär:

~~~
101 = 1·2² + 0·2¹ + 1·2⁰ 
    = 1·4  + 0·2 + 1·1
    = 5
~~~

7-stellige Binärzahl:

~~~
x =      b₆ 2⁶ + b₅ 2⁵ + b₄ 2⁴ + b₃ 2³ + b₂ 2² + b₁ 2 + b₀
  =     (b₆ 2⁵ + b₅ 2⁴ + b₄ 2³ + b₃ 2² + b₂ 2 + b₁) 2 + b₀
  =    ((b₆ 2⁴ + b₅ 2³ + b₄ 2² + b₃ 2 + b₂) 2 + b₁) 2 + b₀
  =   (((b₆ 2³ + b₅ 2² + b₄ 2 + b₃) 2 + b₂) 2 + b₁) 2 + b₀
  =  ((((b₆ 2² + b₅ 2 + b₄) 2 + b₃) 2 + b₂) 2 + b₁) 2 + b₀
  = (((((b₆ 2 + b₅) 2 + b₄) 2 + b₃) 2 + b₂) 2 + b₁) 2 + b₀  
  = (((((([0·2] +b₆) 2 + b₅) 2 + b₄) 2 + b₃) 2 + b₂) 2 + b₁) 2 + b₀  
~~~

Algorithmus (*Horner-Schema*)
~~~
akku ← 0
für alle Binärziffern b von links nach rechts {
    akku ← akku · 2 + b
}
Result: akku
~~~

Beispiel: 101
~~~
akku   b       akku_neu
  0    (init)
  0    1         1
  1    0         2
  2    1         5
  5    --
  ~~~

### ID-Berechnung: Reihe · 8 + Spalte

  - Binär 8 = 1000
  - Platznummer wird (binär) an Reihennummer (binär) »angehängt«

## Teil 2

*Korrektur zum Video*: es sind nicht 1000 oder 1023, sondern
genau 1024 (ID: 0..1023) mögliche Platze.

### Umrechung ID in Zeile/Spalte

- Reihe = ID / 8 (ganzzahlig ohne Rest)
- Spalte = ID % 8  (Divisionrest)