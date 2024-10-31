# Longest Increasing Subsequence

[Wikipedia](https://en.wikipedia.org/wiki/Longest_increasing_subsequence)

Gegeben:

Folge von Zahlen (natürliche Zahlen)

Gesucht:

Längste Teilfolge, die schwach monoton steigt

Beispiel:

~~~
2 1 7 3 5 8 6 4 9
~~~

Mögliche monoton wachsende Teilfolgen:

~~~
2 7 8 9
1 3 5 4 9
7 8 9
~~~

## Algorithmus

### Naives Vorgehen

- ermittle alle Teilfolgen
- prüfe für jede, ob sie monoton ist
- wähle die längste aus

Auswahl

~~~
Idx: 0 1 2 3 4 5 6 7 8
Val: 2 1 7 3 5 8 6 4 9
Ind: 0 1 0 0 0 1 1 0 0

Übernehmen:
       1       8 6
~~~

- Indikator für jeden Index: 0 = Wert verwerfen, 1 = Wert übernehmen
- Interpretation als Binärzahl mit n Stellen
- 2ⁿ solche Werte
- Aufwand: 2ⁿ, n = 10: 1024, n = 20: ≈ 1 Mio, n = 30: ≈ 1 Mrd.

## Dynamische Programmierung

[Wikipedia](https://en.wikipedia.org/wiki/Dynamic_programming)

Grundidee:

- Betrachten alle Teilfolgen vom 1. bis zum (k-1)-ten Element
- Setzen voraus, dass wir die längste monotone Teilfolge aller aller
  dieser Teilfolgen kennen
- Längste monotone Teilfolge bis k-ten Element:
  - Betrachten alle Teilfolgen von 1 ... (k-1)
  - Wählen aus diesen die längste aus, aber nur, wenn a[j] ≤ a[k]
  
*Bellman-Gleichung*

Beispiel:

~~~
Schritt:  Folge
          2 1 7 3 5 8 6 4 9 2
1         1
2           1
3         ^---2    
4         ^-----2
5               ^-3
6                 ^-4
7                 ^---4
8               ^-------3
9                   ^-----5     <- längste Teilfolge
10        ^-----------------2
~~~~                       
  
Rekonstruktion von hinten:
- Start mit 9
- Vorgänger 8
- Vorgänger 5
- Vorgänger 3
- Vorgänger 2

⇒ Folge (9, 8, 5, 3, 2)
⇒ Umkehren (2, 3, 5, 8, 9)

## Beispiel

Jahresmitteltemperaturen seit 1881: [DWD](https://www.dwd.de/DE/leistungen/zeitreihen/zeitreihen.html)

- Temperaturreihe Jahresmittel ab 1881
- alle Werte mit 10 multipliziert
- Eingabe mit -1 abgeschlossen
