package render

/* wird nicht mehr benötigt
import (
	"07-multipolygon/data"
	"07-multipolygon/geo"
	"image/color"
	"log"

	"github.com/llgcode/draw2d/draw2dimg"
)

// Wegkoordinaten aus den Referenz-IDs der Wegliste ermitteln
// return:
// - coords CoordList  Koordinatenliste
// - isClosed bool     Weg war geschlossen
// Falls geschlossen, wird der doppelte Punkt entfernt
func getWayCoords(osm data.OsmData, wayId int64) (geo.CoordList, bool) {
	res := geo.CoordList{}
	nodes := osm.Ways[wayId]
	isClosed := false
	if len(nodes) > 0 && nodes[0] == nodes[len(nodes)-1] {
		isClosed = true
		nodes = nodes[1:]
	}
	for _, nodeId := range nodes {
		if coord, ok := osm.Nodes[nodeId]; ok {
			res = append(res, coord)
		} else {
			log.Printf("Way %d: node %d not found\n", wayId, nodeId)
		}
	}
	return res, isClosed
}

// Transformation einer Koordinatenliste in eine Punktliste
// TODO: besser als Transformer-Methode realisieren
func transformCoords(tr geo.Transformer, coords geo.CoordList) geo.PointList {
	res := geo.PointList{}
	for _, coord := range coords {
		res = append(res, tr.Transform(coord))
	}
	return res
}

func RenderRailways(
	ctx *draw2dimg.GraphicContext,
	osm data.OsmData, tr geo.Transformer,
) {
	ctx.SetStrokeColor(color.Black)
	ctx.SetLineWidth(2.0)
	for wId := range osm.Ways {
		if tags, ok := osm.WayTags[wId]; ok {
			if tags["railway"] == "rail" {
				coords, isClosed := getWayCoords(osm, wId)
				points := transformCoords(tr, coords)
				isStart := true
				for _, p := range points {
					if isStart {
						isStart = false
						ctx.MoveTo(p.X, p.Y)
					} else {
						ctx.LineTo(p.X, p.Y)
					}
				}
				if isClosed {
					ctx.Close()
				}
				ctx.Stroke()
			}
		}
	}
}

func RenderRivers(
	ctx *draw2dimg.GraphicContext,
	osm data.OsmData, tr geo.Transformer,
) {
	ctx.SetStrokeColor(color.RGBA{0, 0, 0xff, 0xff})
	ctx.SetLineWidth(2.0)
	for wId := range osm.Ways {
		if tags, ok := osm.WayTags[wId]; ok {
			if _, ok := tags["waterway"]; ok {
				coords, _ := getWayCoords(osm, wId)
				points := transformCoords(tr, coords)
				isStart := true
				for _, p := range points {
					if isStart {
						isStart = false
						ctx.MoveTo(p.X, p.Y)
					} else {
						ctx.LineTo(p.X, p.Y)
					}
				}
				ctx.Stroke()
			}
		}
	}
}

func RenderHighways(
	ctx *draw2dimg.GraphicContext,
	osm data.OsmData, tr geo.Transformer,
) {
	colors := map[string]color.RGBA{
		"primary":   {0xff, 0, 0, 0xff},
		"secondary": {0x7f, 0x7f, 0, 0xff},
		"tertiary":  {0x7f, 0, 0, 0xff},
		"_":         {0x7f, 0x7f, 0x7f, 0xff},
	}
	ctx.SetLineWidth(2.0)
	for wId := range osm.Ways {
		if tags, ok := osm.WayTags[wId]; ok {
			if hTy, ok := tags["highway"]; ok {
				col := colors["_"]
				if co, ok := colors[hTy]; ok {
					col = co
				} else {
					continue
				}
				ctx.SetStrokeColor(col)
				coords, isClosed := getWayCoords(osm, wId)
				points := transformCoords(tr, coords)
				isStart := true
				for _, p := range points {
					if isStart {
						isStart = false
						ctx.MoveTo(p.X, p.Y)
					} else {
						ctx.LineTo(p.X, p.Y)
					}
				}
				if isClosed {
					ctx.Close()
				}
				ctx.Stroke()
			}
		}
	}
}

func RenderWoods(
	ctx *draw2dimg.GraphicContext,
	osm data.OsmData, tr geo.Transformer,
) {
	ctx.SetFillColor(color.RGBA{0, 0xff, 0, 0xff})
	ctx.SetLineWidth(2.0)
	for wId := range osm.Ways {
		if tags, ok := osm.WayTags[wId]; ok {
			if tags["landuse"] == "forest" || tags["natural"] == "wood" {
				coords, isClosed := getWayCoords(osm, wId)
				points := transformCoords(tr, coords)
				isStart := true
				for _, p := range points {
					if isStart {
						isStart = false
						ctx.MoveTo(p.X, p.Y)
					} else {
						ctx.LineTo(p.X, p.Y)
					}
				}
				if !isClosed {
					log.Printf("way %d for wood not closed\n", wId)
				}
				ctx.Close()
				ctx.Fill()
			}
		}
	}
}

// nicht im Video, analog zu RenderWoods
func RenderLakes(
	ctx *draw2dimg.GraphicContext,
	osm data.OsmData, tr geo.Transformer,
) {
	ctx.SetFillColor(color.RGBA{0, 0, 0xff, 0xff})
	ctx.SetLineWidth(2.0)
	for wId := range osm.Ways {
		if tags, ok := osm.WayTags[wId]; ok {
			if tags["natural"] == "water" {
				coords, isClosed := getWayCoords(osm, wId)
				points := transformCoords(tr, coords)
				isStart := true
				for _, p := range points {
					if isStart {
						isStart = false
						ctx.MoveTo(p.X, p.Y)
					} else {
						ctx.LineTo(p.X, p.Y)
					}
				}
				if !isClosed {
					log.Printf("way %d for wood not closed\n", wId)
				}
				ctx.Close()
				ctx.Fill()
			}
		}
	}
}
*/
