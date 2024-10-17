# Stein-Schere-Papier

Gegeben: Spielstrategie

Bedingungen:
- mindestens 1 Zug
- niemals 2 gleiche Züge direkt hintereinander

Gesucht: Prüfung der Strategie

Codierung als 1, 2, 3 für Stein, Schere, Papier, 0 für Ende

Beispiele
~~~
gültig:   1 2 1 3 1 0
ungültig: 0
ungültig: 1 2 2 1 0
~~~

Ablauf (Endlicher Automat)

~~~mermaid
flowchart LR
Start{Start} -->|1|S1
Start-->|2|S2
Start-->|3|S3
S1-->|0|Stop(((Stop)))
S2-->|0|Stop
S3-->|0|Stop
S1-->|2|S2
S1-->|3|S3
S2-->|1|S1
S2-->|3|S3
S3-->|1|S1
S3-->|2|S2
Err{{Error}}
~~~
