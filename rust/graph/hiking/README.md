# Wanderplanung

Gegeben:
- Orte (Startpunkt, PoI = Point of Interest)
- Wanderverbindungen

Gesucht:
- Datenstruktur
- »Routenplanung«
  - Vom Startpunkt erreichbare Ziele
  - Startpunkt mit max. Routen
  - Verbindung zwischen 2 Startpunkten mit Sehenwürdigkeit dazwischen

## Vorgehen

- Verwenden Graph
- Ortsliste
- Verbindung: Ortsindex → Graphknoten
- Eigenes Objekt mit Knotenliste und Graph
- Zuordnung Objekt → Index über dieses Objekt

## Ergänzungen

(2. Video)

- Korrektur Modul `places`:
  - `is_connection()` muss Graph nur lesen
  - `Display` für `Place`
- Einlesen aus Datei
- Abschnitte mit Endlichen Automat verarbeitet

## Weitere Auswertungen

(3. Video)

- Idealer Startplatz: mit den meisten Touren
- Startpunkte für jedes Ziel
- Touren zwischen 2 Endpunkten mit Zwischenziel

Vorgehen mit Zwischenstop:

- Für alle Knoten Hop prüfe, ob Kante von Start nach Hop und von Hop nach Ziel

