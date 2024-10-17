# Beschreibung der Bewegung eines Objektes

Gegeben: Folge von Aktionen

- Beschleunigung         1
- Gleichmäßige Bewegung  0
- Bremsen               -1
- Notbremse             -2

Beispiel: 0 1 1 0 -2 0 1 0 -1 99

Festlegungen:

- Aktion wirkt immer 1 sec.
- Beschleunigung: a = 10
- Notbremsung: Beschleunigungsfaktor  f = 3
- Geschwindigkeit: 0 ≤ v ≤ 100

Gesucht:

- Zeitpunkt nach Abarbeiten der Aktionen
- Position
- Geschwindigkeit

Algorithmus:

- Bewegung ist zusammengesetzt aus Zeitabschnitten der Länge 1
- jede Aktion startet zum Zeitpunkt t₀ = 0 und endet bei t₁ = 1
- Formel: v(t) = a t + v₀ ⇒ v₁ = a + v₀
  - Beschleunigung: wie die Formel dasteht
  - Gleichmäßig: v₁ = v₀ = 0 · 1 + v₀
  - Bremsen: v₁ = -a + v₀
  - Notbremse: v₁ = - f·a + v₀
- Formel: x(t) = ½a t² + v₀ t + x₀  ⇒ x₁ = ½a + v₀ + x₀
  - Beschleunigung: wie die Formel dasteht
  - Gleichmäßig: x₁ = 0 · a + v₀ + x₀ 
  - Bremsen: v₁ = -½a + v₀ + x₀
  - Notbremse: v₁ = - ½f·a + v₀ + x₀
- Berechnen aus (x₀, v₀) neu (x₁, v₁)
- Begrenzen Geschwindigkeit nach dem Zeitschritt

## Darstellung Diagramm

- Umlenken der Ausgabe in Datei: `./speed1 < speed1.dat > speed1.out`
- Aufruf Gnuplot: `gnuplot`
- Darstellung: `plot 'speed1.out' using 1:2 wi li title 'pos', 'speed1.out' using 1:3 wi li title 'v'`

## Variante

Nun soll jede Aktion eine Dauer haben

Eingabebeispiel: 0 1 1 2 0 3 -1 1 99 99

Jeder Schritt ist Paar (action, duration)
