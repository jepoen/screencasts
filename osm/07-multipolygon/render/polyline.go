package render

import (
	"07-multipolygon/config"
	"07-multipolygon/data"
	"fmt"
	"log"
)

func DrawPolyline(env *config.Environment, nodeIds data.IdList, closeWay bool) {
	start := true
	isClosed := nodeIds.IsClosed()
	idx0 := 0
	if isClosed && closeWay {
		// zeichnen ab 2. Punkt
		idx0 = 1
	}
	if idx0 >= len(nodeIds)-1 {
		return
	}
	for _, nId := range nodeIds[idx0:] {
		p := env.Tr.Transform(env.Data.Nodes[nId])
		if start {
			env.Ctx.MoveTo(p.X, p.Y)
			start = false
		} else {
			env.Ctx.LineTo(p.X, p.Y)
		}
	}
	if isClosed && closeWay {
		env.Ctx.Close()
	}
	env.Ctx.Stroke()
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
	start := true
	for _, nId := range nodeIds[1:] {
		p := env.Tr.Transform(env.Data.Nodes[nId])
		if start {
			env.Ctx.MoveTo(p.X, p.Y)
			start = false
		} else {
			env.Ctx.LineTo(p.X, p.Y)
		}
	}
	env.Ctx.Close()
	return true
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
