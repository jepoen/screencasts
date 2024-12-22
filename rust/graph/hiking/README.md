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