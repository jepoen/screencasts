# Struct – Wetterdaten

- Quelle: [DWD](https://www.dwd.de/DE/leistungen/zeitreihen/zeitreihen.html)
- CSV-Datei wurde manuell erzeugt

Gegeben: CSV-Datei der Wetterdaten (year, temp, rain, sun)

Gesucht:
- interne Darstellung, die Daten pro Jahr zusammenfasst
- Anzeige
- Auswertung über alle Daten (z. B. Mittelwert eines Messwerts)

Vorgehen:
- Verbundtyp (struct)

Beobachtung:
- Debug-Ausgabe mit `#[derive(Debug)]`
- Komponentenzugriff: `struct_var.field`
- Structs werden bei Zuweisung verschoben,
  ansonsten `#[derive(Clone, Copy)]`, wenn das für die Komponenten
  erfüllt ist
- Funktionen (*Getter*) als Parameter übergeben
