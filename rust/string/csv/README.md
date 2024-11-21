# Einlesen/Verarbeiten CSV

Gegeben: CSV-Datei

Gesucht: Liste der enthaltenen Daten

## Aufbau:

- Tabelle
- Kopfzeile
- Datenzeilen
  - Spalten durch Komma getrennt
  - Dezimalpunkt
  - Textspalte (Strings) wird in " eingeschlossen
  - Einschränkung: Trennzeichen darf nicht im Text auftauchen
  - jede Zeile hat gleiche Anzahl Spalten

zusätzlich: Kommentarzeilen, die mit # beginnen

Beispiel: [sort1.csv](sort1.csv)

## Zusammenfassung

- Einlesen mit `read_line()` oder mit `lines()`
- String anlegen: `String::new()`, Inhalt löschen: `clear()`
- String zerlegen: `split()`
- String am Anfang und Ende bereinigen: `trim()`, `trim_matches()`
- Kein Indexzugriff auf Einzelzeichen, aber: Slices
- Typen: `String`, `&str`
