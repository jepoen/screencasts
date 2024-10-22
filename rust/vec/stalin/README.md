# Typ Vec (Stalin-Sort)

Kennen bereits Felder `[Typ;Größe]`, z. B. `[i32; 10]`

Nachteile:

- Größe beschränkt
- muss mit maximaler Größe angelegt werden
- wenn nur teilweise genutzt, muss Größe manuell verwaltet werden

Rust: `Vec`

- Feld dynamischer Größe
- tatsächliche Größe und Kapazität wird automatisch verwaltet

## Beispiel: [Stalin-Sort](https://viviandai.hashnode.dev/esoteric-sorting-algorithms)

Gegegeben:

- Folge von Werten (Zahlen)
- nur positive ganzzahlige Werte, Abschluss mit -1

Gesucht:

- Sortierung der Zahlen
- Sortierte Teilfolge
- Rest

Algorithmus:

1. Nimm 1. Wert
2. Ab dem 2. Wert:
   - Falls Wert ≥ Vorgänger, dann Wert übernehmen (= neuer Vorgänger)
   - sonst → Gulag
3. Wiederhole ab 2.

Beispiel:

~~~
3 1 2 7 4 9 11 2 8
3
  -
    -
3     7
        -
3     7   9
3     7   9 11
               -
                  -
~~~
   
## Anmerkungen

- Anlegen: `Vec::new()` (Variable muss `mut` sein)
- Länge: `vec_var.len()`
- Indexzugriff: `vec_var[idx]`

Beispielprogramme: `stalin0.rs`, `stalin1.rs`

## Iteratoren

- Vector verschieben in *Iterator*: `into_iter()`
- Iterator-Methode: `iterator.next()`
- `for val in iterator {…}`, alternativ: `for val in vector {…}`

Beispielprogramm: `stalin2.rs`
