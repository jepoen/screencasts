# Prüfen Korrektheit Gartenplans

Gegeben: Gartenplan als 2D-Feld mit Zellen `Wall` und anderen

Gesucht:
- Ist die Mauer geschlossen?
- Hat die Mauer Abzweige (nicht zulässig)?

~~~
offen
#####
#   #
## ##
  ##
  
Abzweig
#######
#     #
#######
  #
~~~

Algorithmus:
- Suche beliebige Mauerposition
- Starten hier
- Wiederhole, solange wie möglich
  - Suche Mauer-Nachbar, den wir noch nicht besucht haben \
    ansonsten Abbruch
  - markieren Position als besucht
  - Wechseln auf Nachbar
- geschlossen, wenn Manhattan-Distanz zwischen Anfang und End = 1
- keine Abzweige, wenn alle Mauerelemente besucht wurden

## Zusammenfasssung

- vereinfachte Wegsuche in Graph
- genauer: geschlossener Weg
- Verfahren: *Breitensuche*
