# Abstand und Projektion

## Screencast

1. [Youtube](https://youtu.be/nnEyLYrEw6Y)
1. [Youtube](https://youtu.be/qZdPPydKPSQ)

## Abstand auf der Kugel

Orthodrome: https://de.wikipedia.org/wiki/Orthodrome

Herleitung:

![Abstand](angledist.png)

x = R α

<u, v> = |u||v|cos α

α = acos <u, v>/(R²)

## Umrechnung in Kartesische Koordinaten

![Kartesisch](polar-cartesian.png)

<u, v> = x₀x₁ + y₀y₁ + z₀z₁

r' = R cos φ
z = R sin φ
y = r' sin λ = R cos φ sin λ
x = r' cos λ = R cos φ cos λ

## Projektion

Mercator-Projektion: https://en.wikipedia.org/wiki/Mercator_projection

## Nachträgliche Ergänzung

`transform_test.go`: Testen der Transformationen