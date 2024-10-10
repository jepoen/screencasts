# Potenzberechnung

a^b = a * a * ... * a (b-mal)

Gegeben:
- a, b natürliche Zahlen

Gesucht:
- a^b

Beispiel: 2^4

~~~
                   Produkt
0                  1 Startwert
1 * 2              2
2     * 2          4
3         * 2      8
4             * 2 16 
~~~

Algorithmus:

- product ← 1
- for row in 1, 2, ..., b
  - product ← product * a
  
## Nur die letzten Stellen

Rechnen statt product * a nun: (product * a) % module in jedem Schritt

## Schnelles Potenzieren

Beispiel: 2^13 = 8192

~~~
2^13 = 2^(8 + 4 + 1)
     = 2⁸ * 2⁴  * 2¹
     = 256 * 16 * 2
     
8 + 4 + 1 = 1101₂

2⁸ = (2⁴)², 2⁴ = (2²)², 2² = (2¹)²
~~~
2^13

Exp  %2 Quad      Faktor   Prod
                       1      1
 13   1    2 (2¹)      2      2
  6   0    4 (2²)      -
  3   1   16 (2⁴)     16     32
  1   1  256 (2⁸)    256   8192
  0   -------------------------
  
3^10

Exp %2 Quad        Faktor    Prod
                        1       1
 10  0    3 (3¹)        -       1
  5  1    9 (3²)        9       9
  2  0   81 (3⁴)        81	  729
  1  1 6561 (3⁸)      6561  59049
  0
~~~

Algorithmus

- prod ← 1
- while exp > 0:
  - m ← exp % 2
  - if m == 1: prod ← prod * a
  - a ← a²
  - exp ← exp/2
