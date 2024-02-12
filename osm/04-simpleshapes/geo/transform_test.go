package geo

// Transform Tests, nach dem Screencast ergänzt

import (
	"math"
	"testing"
)

// Plausibilitätstest: Diagonallänge der KM-Transformation = GeoDist
func TestTransform1(t *testing.T) {
	bbox := NewBbox(13.0, 50.0, 14.0, 51.0)
	pr := NewMercatorProjection()
	tr := NewTransformer(pr, bbox, 10, 10, 1)
	p0 := tr.TransformToKm(bbox.Coord0)
	p1 := tr.TransformToKm(bbox.Coord1)
	dGeo := GeoDist(bbox.Coord0, bbox.Coord1)
	dProj := math.Hypot(p1.X-p0.X, p1.Y-p0.Y)
	if relError(dGeo, dProj) > 0.01 {
		t.Errorf("dist expected %f got %f", dGeo, dProj)
	}
}

// Transformation in Ebene
func TestTransform2(t *testing.T) {
	bbox := NewBbox(13.0, 50.0, 14.0, 51.0)
	mmPkm := 7.0
	ptPmm := 3.0
	dGeo := GeoDist(bbox.Coord0, bbox.Coord1)
	dScaledGeo := dGeo * mmPkm * ptPmm
	pr := NewMercatorProjection()
	tr := NewTransformer(pr, bbox, mmPkm, ptPmm, ORIENT_POSITIVE)
	p0 := tr.Transform(bbox.Coord0)
	p1 := tr.Transform(bbox.Coord1)
	dPlane := math.Hypot(p0.X-p1.X, p0.Y-p1.Y)
	// Test Ursprung
	if p0.X != 0 || p0.Y != 0 {
		t.Errorf("origin (0, 0), got %v", p0)
	}
	// Vergleich Diagonallängen in der Zeichenebene
	if relError(dScaledGeo, dPlane) > 1e-2 {
		t.Errorf("wrong size got %f expected %f", dPlane, dScaledGeo)
	}
	tr = NewTransformer(pr, bbox, mmPkm, ptPmm, ORIENT_NEGATIVE)
	p0 = tr.Transform(bbox.Coord0)
	p1 = tr.Transform(bbox.Coord1)
	dPlane = math.Hypot(p0.X-p1.X, p0.Y-p1.Y)
	// Ursprung Zeichenebene nun links oben
	if p0.X != 0 || p1.Y != 0 {
		t.Errorf("origin (0, 0), got (%f,%f)", p0.X, p1.Y)
	}
	if relError(dScaledGeo, dPlane) > 1e-2 {
		t.Errorf("wrong size got %f expected %f", dPlane, dScaledGeo)
	}
}
