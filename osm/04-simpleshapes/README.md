# Zeichnen einfacher Formen

- Linenzüge
- Polygone

## Screencast

[Youtube](https://youtu.be/XXXX)

## Anmerkungen

- Das Zeichnen von Wasserflächen wurde nachträglich ergänzt
- Abfragetest: [overpass turbo](https://overpass-turbo.eu/)
- Overpass-Abfrage aller Elemente einer Region (Achtung: sehr viele Daten):
  ~~~~
  [bbox: {{bbox}}];
  (
    rel;
    way;
    node;
  );
  (
    ._;
    >;
  );
  out;
  ~~~~
- Map Features: https://wiki.openstreetmap.org/wiki/DE:Map_Features bzw.
  https://wiki.openstreetmap.org/wiki/Map_Features
