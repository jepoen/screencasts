# Iterator: LinSpace

Gegeben:
- Intervall [a, b], k Segmente

Gesucht:
- Stützstellen der Segmente

Beispiel:
- Intervall [0, 10], 4 Segmente
- Ergebnis [0, 2.5, 5, 7.5, 10]

## Iterator

- Typ `Item`, bei uns `f64`
- Methode `next() -> Option<Item>`, liefert nächstes Element oder None

`while`-Schleife

~~~
while let Some(val) = it.next() {
    // verarbeite val
}
~~~

`for`-Schleife

~~~
for val in iterableObject {
    verarbeite val
}
~~~

Daraus entsteht:

~~~
let mut it = iterableObject.into_iter();
while let Some(val) = it.next() {
    // verarbeite val
}
~~~

## Consumer

- Methoden, die den Iterator »verbrauchen«
- `collect()`: sammelt alle Werte in Collection ein
- `sum::<typ>()`, `count()`: Summe, Anzahl
- `all(test)`, `any(test)`: Test auf Bedingung

## Adapter

- wandelt Iterator um in neuen Iterator
- `skip(k)`: überspringt k Werte
- `step_by(k)`: nimmt jeden k-ten Wert
- `take(k)`: nimmt max. k Werte und bricht ab
- `filter(test)`: lässt nur getestete Werte durch
- `map(transform)`: wandelt jeden Wert um
- `scan(state, transform)`: wandelt jeden Wert um, merkt sich Vergangenheit
