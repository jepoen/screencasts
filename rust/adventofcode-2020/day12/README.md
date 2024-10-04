# Advent of Code 2020, Tag 12 Teil 1

Gegeben:

- Position/Richtung des Schiffs: (0, 0), EAST
- Folge von Instruktionen Befehl/Wert

Gesucht:

- Abstand von Startposition (Manhattan-Distanz)
- = Endposition des Schiffes

Algorithmus

- Zustandsraum: Position/Richtung des Schiffs
- Aktionen
  - N/S/E/W: entsprechende x- bzw. y-Koordinate verändern
  - F: entsprechend dir die Koordinate ändern (O: x+, W: x-, N: y+, S: y-)
  - Rotation: dir verändert
  
Rotation:
- dir als Zahl (0, 1, 2, 3) im positiven Umlaufsinn.
- Linksrotation um 90°: 1 addieren mod 4
  ~~~
  dir ← (dir + angle/90) % 4
  ~~~
- Rechtsrotation: `angle ← 360-angle` und Linksrotation

Instruktion:
- Enum: Variante mit Wert = Instruktion

Position des Schiffs:
- struct: x, y, dir

# Teil 2

Position (State) des Schiffs:
- struct x, y, wx, wy

Rotation:

~~~
         | y
         +
         |
    90*  +2
         |
         +1    * (x, y)
         |
 --+--+--+--+--+--+-- x
     -1  |     2
180*     +
         |
         +  *270
         |
         +
         |      (2, 1) (rot 90) → (-1, 2) → (-2, -1) → (1, -2)
~~~

Allgemein Linksrotation (x, y):

- 90°: (-y, x)
- 180°: (-x, -y)
- 270°: (y, -x)
