package geo

import (
	"math"
)

type Orientation int

const (
	ORIENT_POSITIVE Orientation = iota
	ORIENT_NEGATIVE
)

type Transformer struct {
	proj    Projection
	bbox    Bbox
	mmPkm   float64
	ptPmm   float64
	orient  Orientation
	scaleKm float64
	scale   float64
	p0      Point
	p1      Point
}

func NewTransformer(proj Projection, bbox Bbox, mmPkm, ptPmm float64,
	orient Orientation,
) Transformer {
	tr := Transformer{
		proj:   proj,
		bbox:   bbox,
		mmPkm:  mmPkm,
		ptPmm:  ptPmm,
		orient: orient,
	}
	tr.computeScale()
	return tr
}

func (tr *Transformer) computeScale() {
	p0 := tr.proj.Project(tr.bbox.Coord0)
	p1 := tr.proj.Project(tr.bbox.Coord1)
	geoDist := GeoDist(tr.bbox.Coord0, tr.bbox.Coord1)
	projDist := math.Hypot(p1.X-p0.X, p1.Y-p0.Y)
	tr.scaleKm = geoDist / projDist
	tr.scale = tr.scaleKm * tr.mmPkm * tr.ptPmm
	tr.p0 = Point{p0.X * tr.scale, p0.Y * tr.scale}
	tr.p1 = Point{p1.X * tr.scale, p1.Y * tr.scale}
}

func (tr Transformer) TransformToKm(coord Coord) Point {
	p := tr.proj.Project(coord)
	return Point{p.X * tr.scaleKm, p.Y * tr.scaleKm}
}

func (tr Transformer) Transform(coord Coord) Point {
	p := tr.proj.Project(coord)
	x := p.X*tr.scale - tr.p0.X
	y := p.Y * tr.scale
	if tr.orient == ORIENT_POSITIVE {
		y -= tr.p0.Y
	} else {
		y = tr.p1.Y - y
	}
	return Point{x, y}
}
