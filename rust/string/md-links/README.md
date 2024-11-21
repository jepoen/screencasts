# Extrahieren von Links aus Markdown-Dateien

Gegeben: Markdown-Datei

Gesucht: alle enthaltenen Links: Titel/URL

Beispiel: [Rust-Doku](https://rust-lang.org/)

Syntax:

~~~
'[' Titel ']' '(' URL ')'
~~~

[Markdown-Beschreibung \[Original\]](https://daringfireball.net/projects/markdown)

Vorgehen:

- Endlicher Automat
- Formulierung häufig auch als regulärer Ausdruck

~~~mermaid
flowchart LR

S0{S₀} -->|"["|Title
S0 -->|"..."|S0
Title-->|"]"|S1
Title-->|"\"|Q
Title-->|"..."|Title
Q-->|"..."|Title
Title-->|"["|Tnew((T'))
Tnew-->|eps|Title
S1-->|"("|Url
S1-->|"["|Tnew
S1-->|"..."|S0
Url-->|")"|Final[[Final]]
Url-->|"..."|Url
Final-->|eps|S0
~~~

## Beschreibung Automat

[Endlicher Automat](https://en.wikipedia.org/wiki/Deterministic_finite_automaton)

## Tests

[Text [LinkTitle](http://example.org)
[Kein Linktitle](Url aber jetzt [Title](http://example2.org)
[Kein Linktitle] (keine URL)
[LinkTitle1](url1)[LinkTitle2](url2)

## Zusammenfassung

- String in Zeichen zerlegen mit `chars()`
- Kopie eines Strings: `clone()`
- normalerweise wird DFA automatische aus *regular expression* erzeugt