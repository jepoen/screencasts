# 2D-Feld: Zellen mit Bedeutung versehen, Ausgabe verbessern

Gegeben:
- Eingabe: 0: leer, 1: Mauer, andere Werte: unbekannt

Gesucht:
- Sinnvoller Datentyp für Zelle
- schönere Darstellung

Datentyp: `enum Cell`
- 0 ⇒ `Free`
- 1 ⇒ `Wall`

Beobachtungen:
- Ausgabe mit `match` Variante ⇒ Zeichen
- Vergleich von `enum` nicht direkt möglich hier: \
  `if let Variante = variable`
