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


## Methoden, Constructor

- mit `impl StructName {}` assoziiere Funktionen und Methoden definiert
- diese sind meist `pub`
- Assoziierte Funktion: oft als Constructor zum Erzeugen Objekt
- Methode: Parameter `&self`, `&mut self`, `self` für Aktionen auf dem Objekt
- Zugriff von außen:
  - assoziierte Funktion: `StructName::function(...)`
  - Methode: `object.methode()`

## Implementieren von Traits

Ziel:
- Prozesse sortieren: Name, Dauer
- Gesamtzeit bestimmen
- Dauer einfacher anzeigen

Sortieren:
- Traits: `Ord`, `PartialOrd`, `Eq`, `PartialEq`

Implementieren Standard-Traits:
- `Ord`
- `Display`
- `Add`
