package render

import (
	"06-simpleshapes/config"
	"06-simpleshapes/data"
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
func DrawPolygon(env *config.Environment, wId int64, nodeIds data.IdList) {
	isClosed := nodeIds.IsClosed()
	if !isClosed {
		log.Printf("polygon %d is not closed\n", wId)
		return
	}
	if len(nodeIds) < 4 {
		log.Printf("polygon %d has only %d vertices",
			wId, len(nodeIds),
		)
		return
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
	env.Ctx.Fill()
}
