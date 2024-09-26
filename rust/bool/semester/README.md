# Bestimmen, ob Datum im Semester liegt

Gegeben:

Datum: Monat Tag

Gesucht:

Im Semester? (ja oder nein)

Zusatzinformation (als Eingabegröße): Dauer Semester

- Wintersemester: 1. Oktober bis 15. Februar des Folgejahrs
- Sommersemester: 1. April bis 15. Juli

Aufruf: `./is_semester1 month day`

## Test abgeschlossenes Intervall

~~~
Test: 2 <= a <= 5 // mathematisch korrekt, aber in Rust falsch!
Zerlegen in 2 <= a UND a <= 5 // Rust: &&

       |--------| gesucht
       |--------------> 2 <= a
    <-----------|       a <= 5
  --+--+--+--+--+--+--
    1  2  3  4  5  6
~~~

Skizze Semester

~~~
  |--|                       |-----| WS
           |--------|                SS
  1  2  3  4  5  6  7  8  9 10 11 12
~~~

Oder-Verknüpfung: `||`

