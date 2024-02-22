package geo

import (
	"math"
	"testing"
)

func TestOverpassStr(t *testing.T) {
	bbox := NewBbox(13.3, 50.8, 13.5, 51.0)
	expectedRes := "50.800000,13.300000,51.000000,13.500000"
	res := bbox.ToOverpassStr()
	if res != expectedRes {
		t.Errorf("overpassStr expected %s got %s", expectedRes, res)
	}
}

func TestDist(t *testing.T) {
	c0 := Coord{13.40, 52.517}
	c1 := Coord{139.767, 35.70}
	expectedDist := 8912.0
	dist := GeoDist(c0, c1)
	if relError(expectedDist, dist) > 0.01 {
		t.Errorf("dist expected %f got %f", expectedDist, dist)
	}
}

func relError(expect, got float64) float64 {
	return math.Abs((got - expect) / expect)
}
