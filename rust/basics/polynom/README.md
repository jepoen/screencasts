# Polynomberechnung

Polynom 2. Grades

~~~
y = f(x) = a x^2 + b x + c
         = a₂ x² + a₁ x + a₀
~~~

Eingabe:

*x a₂ a₁ a₀* ganzzahlig

Ausgabe:

*y*

Aufruf: `./polynom1 x a b c`

Beispiel: `./polynom1 2 2 -3 5`

~~~
y = 2 * 2^2 -3 * 2 + 5 = 7
~~~

## Horner-Schema

~~~
y = a₃ x³ + a₂ x² + a₁ x + a₀
  = (a₃ x² + a₂ x + a₁) x + a₀
  = (a₃ x + a₂) x + a₁) x + a₀
~~~

Berechnungsschema:

~~~
a₃
   * x
       + a₂
            * x
                + a₁
                     * x
                         + a₀
~~~

Algorithmus:
~~~
y <- 0
y <- y + a_3
y <- y * x
y <- y + a_2
y <- y * x
y <- y + a_1
y <- y * x
y <- y + a_0
~~~
