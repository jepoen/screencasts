# Bestimmen der Fläche innerhalb der Mauer

Gegeben:
- Feld mit geschlossener Mauer, freie Zellen innen und außen

Gesucht:
- freie innere Zellen

Verfahren:
- Fluten:
  - Suchen freies Feld am Rand (bei uns (0, 0)
  - Flute dieses Feld
  - Suche freie Nachbarn
  - flute diese
  - Wiederhole, bis nichts mehr geflutet kann
- übrige freie Zellen sind innen und werden gezählt
