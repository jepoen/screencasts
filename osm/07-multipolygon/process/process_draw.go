package process

import (
	"07-multipolygon/config"
	"07-multipolygon/data"
	"07-multipolygon/parser"
	"07-multipolygon/render"
	"log"
)

type DrawFunction func(step *parser.DrawStep, style *config.Style, env *config.Environment)

var drawFunctions = map[string]DrawFunction{
	"wayLine":    drawWayLine,
	"wayPolygon": drawWayPolygon,
}

func evalDraw(step *parser.DrawStep, env *config.Environment) {
	if fu, ok := drawFunctions[step.DrawOp]; ok {
		style := config.EvalStyle(step.Style, env)
		fu(step, style, env)
	} else {
		log.Printf("draw function %s is not implemented\n", step.DrawOp)
	}
}

func drawWayLine(step *parser.DrawStep, style *config.Style, env *config.Environment) {
	// TODO: set style
	//env.Ctx.SetStrokeColor(color.Black)
	//env.Ctx.SetLineWidth(0.5 * env.Config.PtPerMm)
	style.SetContext(env)
	wayIds := processFilter(step.Filter, env.Data.WayTags)
	ways := []data.IdList{}
	for _, wId := range wayIds {
		ways = append(ways, env.Data.Ways[wId])
	}
	if style.ConnectWays {
		// TODO: maybe connect ways?
		len0 := len(ways)
		ways = ConnectWays(ways)
		len1 := len(ways)
		log.Printf("connectWays old: %d new %d", len0, len1)
	}
	for _, wayPointIds := range ways {
		render.DrawPolyline(env, wayPointIds, style.CloseWays)
	}
}

func drawWayPolygon(step *parser.DrawStep, style *config.Style, env *config.Environment) {
	// TODO: set style
	//env.Ctx.SetFillColor(colornames.Lawngreen)
	style.SetContext(env)
	wayIds := processFilter(step.Filter, env.Data.WayTags)
	for _, wId := range wayIds {
		render.DrawPolygon(env, wId, env.Data.Ways[wId])
	}
}
