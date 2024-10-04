# Zinsberechnung (Schleifen)

Festverzinste Wertanlage (fester Zinssatz für gesamte Laufzeit)

Gegeben:

- Startguthaben (balance)
- Zinssatz pro Jahr (%) (rate)
- Laufzeit in Jahren  (duration)

Gesucht:

- Guthaben am Ende der Laufzeit
- (Zinsen pro Jahr)

Algorithmus:

pro Jahr

~~~
balance_new = balance + balance * rate /100
~~~

Beispiel: balance = 200, rate = 3

~~~
balance_new = 200 + 200 * 3 / 100
            = 206
~~~

Für duration Jahre: duration-mal wiederholen

~~~mermaid
flowchart TB
input("balance, rate, dur = int_args()")-->init[year = 1]
init-->join(("."))
join-->test{year ≤ duration?}
test-->|ja|compute["balance = f(balance)"]
compute-->reinit[year += 1]
reinit-->join
test-->|nein|out[["print(balance)"]]
out-->stop{{Stop}}
~~~

## Schleifen

- abweisende Schleifen (Test, bevor Schleife betreten wird)
- while-Schleife (init → while test { anweisung; reinit})
- for-Schleife (for var in interval { anweisung })
- Intervall:
  - start .. stop (start, ..., stop-1)
  - start ..= stop (start, ..., stop)

## Neue Frage

Wie lange dauert es, bis sich Guthaben verdoppelt hat?

Eingabe:

- (Guthaben)
- Zinssatz

Ausgabe:

- Dauer (Jahre)

~~~mermaid
flowchart TB
input("rate = int_args()")-->init["year = 0
balance_0 = 10_000
balance = balance_0
"]
init-->join(("."))
join-->test{balance < 2*balance_0?}
test-->|ja|compute["balance = f(balance)"]
compute-->reinit[year += 1]
reinit-->join
test-->|nein|out[["print(year)"]]
out-->stop{{Stop}}
~~~
