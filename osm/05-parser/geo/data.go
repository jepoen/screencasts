package geo

import (
	"fmt"
	"math"
)

const RADIUS_EARTH = 6366

type Coord struct {
	Lon float64
	Lat float64
}

type CoordList []Coord

type Bbox struct {
	Coord0 Coord
	Coord1 Coord
}

type Point struct {
	X float64
	Y float64
}

type PointList []Point

func NewBbox(lon0, lat0, lon1, lat1 float64) Bbox {
	return Bbox{Coord{lon0, lat0}, Coord{lon1, lat1}}
}

func (bbox Bbox) ToOverpassStr() string {
	return fmt.Sprintf("%.6f,%.6f,%.6f,%.6f",
		bbox.Coord0.Lat, bbox.Coord0.Lon,
		bbox.Coord1.Lat, bbox.Coord1.Lon,
	)
}

func Radians(deg float64) float64 {
	return deg * math.Pi / 180.0
}

func GeoDist(p0, p1 Coord) float64 {
	r0 := math.Cos(Radians(p0.Lat))
	z0 := math.Sin(Radians(p0.Lat))
	x0 := r0 * math.Cos(Radians(p0.Lon))
	y0 := r0 * math.Sin(Radians(p0.Lon))
	r1 := math.Cos(Radians(p1.Lat))
	z1 := math.Sin(Radians(p1.Lat))
	x1 := r1 * math.Cos(Radians(p1.Lon))
	y1 := r1 * math.Sin(Radians(p1.Lon))
	innerProd := x0*x1 + y0*y1 + z0*z1
	return RADIUS_EARTH * math.Acos(innerProd)
}
