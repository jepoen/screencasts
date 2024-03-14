package geo

import (
	"fmt"
	"math"
)

type Projection interface {
	fmt.Stringer
	Project(coord Coord) Point
}

type MercatorProjection struct{}

func (p *MercatorProjection) String() string {
	return "MercatorProjection"
}

func NewMercatorProjection() *MercatorProjection {
	return &MercatorProjection{}
}

func (p *MercatorProjection) Project(coord Coord) Point {
	x := Radians(coord.Lon)
	y := math.Log(math.Tan(math.Pi/4.0 + Radians(coord.Lat)/2.0))
	return Point{x, y}
}
