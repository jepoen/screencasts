# Advent of Code 2020 Tag 13

## Teil 1

Gegeben:
- Zeitstempel
- Busliste (Nr oder x, durch Komma getrennt)

Gesucht:
- Auf welchen Bus müssen wir am wenigsten warten?
- Wie lange warten wir?

Algorithmus:
- Wartezeit zu Zeit t für Bus b schwierig
- Einfach: »Verpasstzeit« (missing time): t % b = m
- Wartezeit w = b - m

~~~
 +----t-----+
 ^ m  ^ w?  ^
 |←   b    →|
~~~

## Teil 2

- Busse p₁, p₂, …
- Rechnen Wartezeit um in missing-time \
  Bsp. Bus 3 zum Zeitpunkt  t + 13 (t+1, t+4, t+7, t+10)
- m₂: w₂ solange p₂ abziehen, bis der Wert ≤ 0 ist, dann negieren
- Busse treffen sich zum Zeitpunkt 0
- Nächster Treff: t = c₁ * p₁ = c₂ * p₂ = c₃ * p₃
- t muss gleichzeitig durch p₁, p₂, p₃ teilbar sein.
- t = p₁ p₂ p₃

~~~
T₁ = c₁ p₁ + m₁ = c₂ p₂ + m₂ = c₃ p₃ + m₃
T₂ = d₁ p₁ + m₁ = d₂ p₂ + m₂ = d₃ p₃ + m₃

T₂ - T₁ = e₁ p₁ = e₂ p₂ = e₃ p₃ = p₁ p₂ p₃
~~~

Algorithmus am Beispiel (Achtung: im Beispiel der Aufgabe ist w₂ = 1, das Vorgehen ist aber identisch)
~~~
t = 0
step 7
p₁ = 7, p₂ = 13, w₂ = 2 ⇒ m₂ = 11

t   m₂(=)
 0  0
 7  7
14  1
21  8
28  2
35  9
42  3
49 10
56  4
63 11*

Neue Schrittweite = p₁ p₂ = 7 * 13 = 91
~~~

Algorithmus:
~~~
t ← 0
delta ← p₁
for i in 2 .. N:
  solange t % p_i != m_i:
    t ← delta
  // t für p_1 ... p..i gefunden
  delta ← delta * p[i]
Rückgabe t
~~~
