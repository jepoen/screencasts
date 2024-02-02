# Workflow Erstellen OSM-Karte mit Go

## OSM-Daten

- Datenzugang: https://wiki.openstreetmap.org/wiki/Databases_and_data_access_APIs
- API: https://wiki.openstreetmap.org/wiki/API_v0.6
- Dump: https://wiki.openstreetmap.org/wiki/Planet.osm
- Geofabrik: https://download.geofabrik.de/
- Overpass API:
  - Beschreibung: https://wiki.openstreetmap.org/wiki/Overpass_API
  - Server-URL: https://overpass-api.de/api/interpreter
  - Abfrage mit Curl:

  ~~~~
  curl -X post https://overpass-api.de/api/interpreter -d @query.dat
  ~~~~
- Overpass Turbo: http://overpass-turbo.eu/

## Go-Projekt anlegen

Modul initialisieren:

~~~~
go mod init 01-workflow
~~~~

## Graphikbibiotheken

- [Übersicht](https://golang.ch/the-ultimate-list-of-golang-gui-graphics-image-related-libraries/)
- Zusätzlich `tdewolff/canvas`: produzierte Abstürze und fehlerhafte Daten
- `gg`: nur Rastergraphiken
- `draw2d`: zunächst verwendet, bindet nicht gewartete Bibliothek `fpdf` ein
- `go-pdf/fpdf`: weiterentwickelter Fork, kann nur PDF-Graphiken

Installation von `draw2d`

~~~~
go get -u github.com/llgcode/draw2d
~~~~
- 

