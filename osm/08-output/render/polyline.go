package render

import (
	"08-output/config"
	"08-output/data"
	"08-output/geo"
	"fmt"
	"log"

	"github.com/llgcode/draw2d"
)

func PointsToPath(
	ctx draw2d.GraphicContext,
	points geo.PointList,
	isClosed bool,
) {
	ctx.MoveTo(points[0].X, points[0].Y)
	for _, p := range points[1:] {
		ctx.LineTo(p.X, p.Y)
	}
	if isClosed {
		ctx.Close()
	}
}

func CoordsToPoints(
	tr *geo.Transformer,
	coords geo.CoordList,
) geo.PointList {
	points := geo.PointList{}
	for _, c := range coords {
		p := tr.Transform(c)
		points = append(points, p)
	}
	return points
}

func DrawPolyline(env *config.Environment, nodeIds data.IdList, closeWay bool) {
	isClosed := nodeIds.IsClosed()
	idx0 := 0
	if isClosed && closeWay {
		// zeichnen ab 2. Punkt
		idx0 = 1
	}
	if idx0 >= len(nodeIds)-1 {
		// TODO Fehlermeldung?
		return
	}
	if coords, ok := env.Data.WayCoords(nodeIds[idx0:]); ok {
		points := CoordsToPoints(env.Tr, coords)
		PointsToPath(env.Ctx, points, isClosed)
		env.Ctx.Stroke()
	}
	// TODO Fehlermeldung bei fehlerhafter coordList?
}

func drawPolygonPath(env *config.Environment, nodeIds data.IdList, errPrefix string) bool {
	isClosed := nodeIds.IsClosed()
	if !isClosed {
		log.Printf("%s: is not closed\n", errPrefix)
		return false
	}
	if len(nodeIds) < 4 {
		log.Printf("%s: has only %d vertices",
			errPrefix, len(nodeIds),
		)
		return false
	}
	if coords, ok := env.Data.WayCoords(nodeIds); ok {
		points := CoordsToPoints(env.Tr, coords)
		PointsToPath(env.Ctx, points, true)
		return true
	}
	// TODO Fehlermeldung unvollständige coordList
	return false
}

func DrawPolygon(env *config.Environment, wId int64, nodeIds data.IdList) {
	if drawPolygonPath(env, nodeIds, fmt.Sprintf("polygon %d", wId)) {
		env.Ctx.Fill()
	}
}

func DrawMultipolygon(
	env *config.Environment,
	rId int64,
	outer, inner []data.IdList,
) {
	for i, idList := range outer {
		errPrefix := fmt.Sprintf("multipolygon %d, outer way %d", rId, i)
		if !drawPolygonPath(env, idList, errPrefix) {
			return
		}
	}
	for i, idList := range inner {
		errPrefix := fmt.Sprintf("multipolygon %d, inner way %d", rId, i)
		if !drawPolygonPath(env, idList, errPrefix) {
			return
		}
	}
	env.Ctx.Fill()
}
