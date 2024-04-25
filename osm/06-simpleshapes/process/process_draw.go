package process

import (
	"06-simpleshapes/config"
	"06-simpleshapes/data"
	"06-simpleshapes/parser"
	"06-simpleshapes/render"
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
	// TODO: maybe connect ways?
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
