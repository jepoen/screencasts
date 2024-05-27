# Verbinden von Linien, Multipolygon

## Vorbereitungen

Falls ein neues Projekt erstellt werden soll, sind folgende Vorbereiungen
notwendig:

- Erstellen des neuen Projektverzeichnisses `07-multipolygon`
- Initialisieren des Projekts
  ~~~
  go mod init 07-multipolygon
  ~~~
- Anpassen aller Importe in den Quelldateien von `06-simpleshapes`
  auf `07-multipolygon`

## Wege verbinden

Teil 1: Ablauf, Hilfsfunktionen und Tests

**Screencast** [Youtube](https://youtu.be/4rAPhQv60ZI)

Teil 2: Verbinden der Wege, Auswertung Ergebnis

**Screencast** [Youtube](https://youtu.be/GZDkJaBnXzY)

*TODO*: mehrere Styles pro Zeichenschritt (einzeln auf jedes Element anwenden)

## Multipolygon

![OSM-Multipolygon](multipoly.png)

Dokumentation: [OSM-Wiki](https://wiki.openstreetmap.org/wiki/Relation:multipolygon)

Abfrage:

~~~
(
    rel[type='multipolygon'];
    way;
);
(
    ._;
    >;
);
out qt;
~~~

**Screencast**: [Youtube]