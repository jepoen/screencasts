# Iterables, Adapter und Consumer

Gegeben:

- Temperaturen [DWD](dwd.de/DE/leistungen/zeitreihen/zeitreihen.html)

Gesucht:

- Einlesen Temp. als Vektor
- Statistische Auswertungen:
  - Mittel
  - wärmstes Jahr
  - Differenz benachbarter Jahre
  - Dekadenmittel
  - Gleitendes Mittel

Iterator und Adapter: [Rust-Dokumentation](https://doc.rust-lang.org/std/iter/trait.Iterator.html)

## Eingabe

- `io::stdin()` mit Methode `lines()`

## Auswertungen

- Mittlere Temperatur
- heißestes und kältestes Jahr: `enumerate()`, `max_by()`
- Differenz zwischen Jahren: `windows()`, `map()`
- Gleitendes Mittel: `windows()`, `sum()`
- Mittel pro Segment: `chunks()`, `sum()`

## Anzeige

- mit Gnuplot
- 2 Spalten
- `plot '{datei}' using {spalte} with lines`

## Methoden für Iterator

- über alle Elemente: `into_iter()`, `iter()`, `iter_mut()`
- Windows: `windows({len})`
- Nichtüberlappende Segmente: `chunks({len})`
