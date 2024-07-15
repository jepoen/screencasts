# Andere Ausgabeformate

## Vorbereitungen

Falls ein neues Projekt erstellt werden soll, sind folgende Vorbereitungen
notwendig:

- Erstellen des neuen Projektverzeichnisses `08-output`
- Initialisieren des Projekts
  ~~~
  go mod init 08-output
  ~~~
- Anpassen aller Importe in den Quelldateien von `07-multipolygon`
  auf `08-output`

## Refactoring Polyline

Aufspalten der Ausgabe einer Linie oder eines Polygons in:

- Knoten-IDs in Koordinatenliste
- Koordinaten in Canvas-Koordinaten
- Canvas-Koordinaten in GraphicContext-Pfad

**Screencast**: [Youtube](https://youtu.be/22GpLnlW9lo)

## Andere Ausgabeformate

`Canvas` ist nun Interface, das die Daten für das jeweilige Ausgabeformat
speichert.

**Screencast**: [Youtube](https://youtu.be/Uc_rFCia19s)

## Mehrere Styles pro Zeichenobjekt

Fehler in `NewStyle()` korrigiert

**Screencast** [Youtube](https://youtu.be/qkBJbVxNzSI)
