package process

import (
	"06-simpleshapes/config"
	"06-simpleshapes/data"
	"06-simpleshapes/parser"
	"06-simpleshapes/render"
	"image/color"
	"log"

	"golang.org/x/image/colornames"
)

type DrawFunction func(step *parser.DrawStep, env *config.Environment)

var drawFunctions = map[string]DrawFunction{
	"way_line":    drawWayLine,
	"way_polygon": drawWayPolygon,
}

func evalDraw(step *parser.DrawStep, env *config.Environment) {
	if fu, ok := drawFunctions[step.DrawOp]; ok {
		fu(step, env)
	} else {
		log.Printf("draw function %s is not implemented\n", step.DrawOp)
	}
}

func drawWayLine(step *parser.DrawStep, env *config.Environment) {
	// TODO: set style
	env.Ctx.SetStrokeColor(color.Black)
	env.Ctx.SetLineWidth(0.5 * env.Config.PtPerMm)
	wayIds := processFilter(step.Filter, env.Data.WayTags)
	ways := []data.IdList{}
	for _, wId := range wayIds {
		ways = append(ways, env.Data.Ways[wId])
	}
	// TODO: maybe connect ways?
	for _, wayPointIds := range ways {
		render.DrawPolyline(env, wayPointIds, true /*closeWay*/)
	}
}

func drawWayPolygon(step *parser.DrawStep, env *config.Environment) {
	// TODO: set style
	env.Ctx.SetFillColor(colornames.Lawngreen)
	wayIds := processFilter(step.Filter, env.Data.WayTags)
	for _, wId := range wayIds {
		render.DrawPolygon(env, wId, env.Data.Ways[wId])
	}
}
