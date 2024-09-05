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

~~~
flowchart LR;
S[[S]];
Text(((Text)));

S -->|"[0..9]"| Na;
Na -->|"[0..9]"| Na;
Na -->|"[-]"| Dash;
Dash -->|"[0..9]"| Nb;
Nb -->|"[0..9]"| Nb;
Nb -->|"[ ]"| Bla;
Bla -->|"[a..z]"| Letter;
Letter -->|"[:]"| Colon;
Colon -->|"[ ]"| Blb;
Blb -->|"[a..z]"| Text;
Text -->|"[a..z]"| Text;
~~~

~~~mermaid
flowchart LR

S-->A
A-->A
A-->B
~~~

## Teil 2

Text ist korrekt wenn text[pos1] == letter ex-oder text[pos2] == letter
