# Advent of Code 2020, Tag 2

Gegeben: Zeilen der Form

~~~
zahl1-zahl2 buchstabe: text
~~~

- Buchstabe: klein, ASCII
- Text: Kleinbuchstaben, ASCII

Gesucht: Anzahl der Zeilen, bei denen der Text
- mindesten zahl1-mal
- höchsten zahl2-mal
den Buchstabe enthält

Hauptproblem: Zeile umwandeln in Komponenten

Reguläre Ausdrücke:
~~~
grep -E \
'^[0-9]+-[0-9]+ [a-z]: [a-z]+$'
~~~

Alternativ str.`split()`

Automat (die mit . markierten Knoten sind keine Zustände, sondern derzeit erforderlich zur korrekten Anzeige):

~~~mermaid
flowchart LR
S[[S]]
Text(((Text)))
X1((.))
X2((.))
X3((.))

S -->|"[0..9]"| N1
N1 -->|"[0..9]"| X1
X1 --> N1
N1 -->|"[-]"| Dash
Dash -->|"[0..9]"| N2
N2 -->|"[0..9]"| X2
X2 --> N2
N2 -->|"[ ]"| Bl1
Bl1 -->|"[a..z]"| Letter
Letter -->|"[:]"| Colon
Colon -->|"[ ]"| Bl2
Bl2 -->|"[a..z]"| Text
Text -->|"[a..z]"| X3
X3 --> Text
~~~


## Teil 2

Text ist korrekt wenn 
- entweder text[pos1] == letter
- oder text[pos2] == letter (XOR)
