package geo

import (
	"log"
	"testing"
)

func TestProjection(t *testing.T) {
	bbox := NewBbox(13.0, 50.0, 13.5, 50.5)
	proj := NewMercatorProjection()
	c0 := bbox.Coord0
	c1 := Coord{bbox.Coord1.Lon, bbox.Coord0.Lat}
	c2 := Coord{bbox.Coord0.Lon, bbox.Coord1.Lat}
	dHoriz := GeoDist(c0, c1)
	dVert := GeoDist(c0, c2)
	p0 := proj.Project(c0)
	p1 := proj.Project(c1)
	p2 := proj.Project(c2)
	if p0.Y != p1.Y {
		t.Errorf("not horizontal")
	}
	if p0.X != p2.X {
		t.Errorf("not vertical")
	}
	dProjHoriz := p1.X - p0.X
	dProjVert := p2.Y - p0.Y
	qGeo := dVert / dHoriz
	qProj := dProjVert / dProjHoriz
	log.Printf("qGeo %f qProj %f\n", qGeo, qProj)
	if relError(qGeo, qProj) > 0.01 {
		t.Errorf("projection ratio expected %f got %f", qGeo, qProj)
	}
}
