# Elemente der Objektorientierten Programmierung

(OOP)

Gegeben:
- Liste von Prozessen
- Prozess:
  - Name
  - Dauer (h:min)
  - Priorität

Gesucht:
- Datenstrukturen
- Anzeige
- Analysen

## OOP

Drei Konzepte:
- Kapselung √
- Vererbung —
- Polymorphie (über Traits)

## Modul

- Sichtbarkeit: 
  - innerhalb des Moduls: alles was "global" definiert ist
  - außerhalb: nur, was explizit mit `pub` exportiert wird

- Erstellen:
  - inline: `mod name { ... }`
  - in Datei `name.rs` im gleichen Verzeichnis, im Programm: `mod name;`
  - in Verzeichnis `name` Datei `mod.rs`, im Programm: `mod name;`

  Zusammenfassung:
  - nach außen sichtbar mit `pub`
  - Pfad verkürzen mit `use modul_name::object`
  - 3 Varianten der Module