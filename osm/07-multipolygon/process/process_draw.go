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
	"wayLine":         drawWayLine,
	"wayPolygon":      drawWayPolygon,
	"relMultipolygon": drawRelMultipolygon,
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

func drawRelMultipolygon(step *parser.DrawStep, style *config.Style, env *config.Environment) {
	style.SetContext(env)
	relIds := processFilter(step.Filter, env.Data.RelTags)
	for _, rId := range relIds {
		outer := []data.IdList{}
		inner := []data.IdList{}
		for _, m := range env.Data.Relations[rId] {
			if m.Type != data.MEMBER_WAY {
				continue
			}
			nodeIds := env.Data.Ways[m.Ref]
			switch m.Role {
			case "outer":
				outer = append(outer, nodeIds)
			case "inner":
				inner = append(inner, nodeIds)
			default:
				log.Fatalf("unknown role %s of multipolygon %d, way %d",
					m.Role, rId, m.Ref,
				)
			}
		}
		outer = ConnectWays(outer)
		if len(outer) == 0 {
			log.Fatalf("multipolygon %d has no outer border", rId)
		}
		inner = ConnectWays(inner)
		render.DrawMultipolygon(env, rId, outer, inner)
	}
}
