# Playfair Chiffre

Gegeben: Text, Schlüssel *oder* Chiffre, Schüssel

Gesucht: Verschlüsselter Text *oder* Klartext

Algorithmus:

Schlüssel: Großbuchstaben ohne Wiederholung, ohne J (25 verschiedene Buchstaben)

z. B. LINUX

Anordnung im 5×5-Quadrat

~~~
LINUX
ABCDE
FGHKM
OPQRS
TVWYZ
~~~

Text verschlüsseln:

Vorbereitung:
- nur Großbuchstaben
- Ersetzen J, Y durch I
- zwischen wiederholte Buchstaben Y einschieben
- Länge auf gerade Anzahl ergänzen

Beispiel: Babysitter → BABISITTER → BA BI SI TY TE RY

Verschlüsselung:

→ BA IB XP TY AZ YR

Entschlüsselung:

  BA BI SI TY TE RY

Verfahren:
- Buchstabenpaar spannt Rechteck auf
- Verwenden die beiden anderen Ecken (Spalte bleibt gleich)

Aufruf:
~~~
playfair enc|dec|prep key text
~~~
