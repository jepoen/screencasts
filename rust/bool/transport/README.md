# Auswahl Verkehrsmittel für Uniweg

Gegeben:
- Aufstehzeit (wake_up), Bsp. 730
- Startzeit der ersten Veranstaltung (start_lesson) 915

Gesucht:
- »Bestes« Verkehrsmittel:
  - zu Fuß (walk) bei > 30 min Zeit
  - Tram bei > 20 min
  - Fahrrad bei > 10 min
  - Online > 0 min
  - sonst verpasst

Teilproblem: Zeitdifferenz in min
- naiv: 915-730 = 185
- korrekt: 30 + 60 + 15 = 105

Algorithmus: Berechnen Minuten seit 0 Uhr
- (timestamp / 100) * 60 + timestamp % 100

Algorithmus

~~~mermaid
flowchart TB

st1[delta = start_lesson-wake_up] --> q_walk
q_walk{delta > 30} -->|ja|o_walk[[Walk]]
q_walk-->|nein|q_tram
q_tram{delta > 20} -->|ja|o_tram[[Tram]]
q_tram-->|nein|q_bike
q_bike{delta > 10}-->|ja|o_bike[[Bicycle]]
q_bike-->|nein|q_online
q_online{delta > 0}-->|ja|o_online[[Online]]
q_online-->|nein|o_none[[Too Late]]
o_walk-->st2
o_tram-->st2
o_bike-->st2
o_online-->st2
o_none-->st2[next statement]
~~~

Programmstruktur:

~~~
if test_1 {
  Alternative_1;
} else if test_2 {
  Alternative_2;
} else if test_3 {
  Alternative_3;
} else {
  Alternative_sonst; // keiner der obigen Fälle
}
~~~

## 2. Variante

alle möglichen Verkehrsmittel

~~~mermaid
flowchart TB

st1[delta = start_lesson-wake_up] --> q_walk
q_walk{delta > 30} -->|ja|o_walk[[Walk]]
o_walk-->q_tram
q_walk-->|nein|q_tram
q_tram{delta > 20} -->|ja|o_tram[[Tram]]-->q_bike
q_tram-->|nein|q_bike
q_bike{delta > 10}-->|ja|o_bike[[Bicycle]]-->q_online
q_bike-->|nein|q_online
q_online{delta > 0}-->|ja|o_online[[Online]]
q_online-->|nein|o_none[[Too Late]]
o_online-->st2
o_none-->st2[next statement]
~~~

Programmstruktur:

~~~
if test_1 {
  Möglichkeit_1;
}
if test_2 {
  Möglichkeit_2;
}
if test_3 {
  Möglichkeit_3;
} else {
  Alternative_3;
}
~~~