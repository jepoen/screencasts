# Advent of 2020 Tag 8 Teil 1

## Aufbau CPU

~~~
CPU                        Program  Nr.

accumulator     (ac)
program counter (pc) ----> nop +0    0
				           acc +1    1
                           jmp -2    2
                           acc +3    3
~~~

Erweiterung auf echte CPU:

- Datenspeicher
- weitere Befehle
- bedingte Befehle (bedingte Sprünge)

Gegeben:
- CPU mit acc=0, pc=0
- Programm mit 3 verschiedenen Befehlen, jeweils mit Wert

Gesucht:
- Wert des Accus, bevor der erste Befehl ein zweites Mal aufgerufen wird

Lösung:
- Liste von Enums mit Wert
- HashSet (Alternative Histogramm)

## Teil 2

- Tausche genau eine Jmp/Nop-Anweisung so, dass der pc den Wert
  `program.len()` erreicht.
  
Lösung:

- Sequentiell Anweisung austauschen
- Programm ausführen und Ergebnis analysieren
- Anweisung zurücktauschen


