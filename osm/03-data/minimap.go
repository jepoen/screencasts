package main

// Zeichnen der Testkarte
import (
	"03-data/data"
	"03-data/geo"
	"fmt"
	"image"
	"image/color"
	"log"
	"strings"

	"github.com/llgcode/draw2d/draw2dimg"
)

// Overpass API
const URL = "https://overpass-api.de/api/interpreter"

// Testabfrage
const QUERY_TEMPLATE = `
[bbox: {{bbox}}];
(
	rel[type="multipolygon"];
	way;
    node;
);
(
	._;
	>;
);
out qt;
`

func main() {
	// Abfrage
	bbox := geo.NewBbox(13.3, 50.8, 13.5, 51.0)
	query := strings.ReplaceAll(QUERY_TEMPLATE, "{{bbox}}", bbox.ToOverpassStr())
	osm, err := data.GetData(URL, query)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Nodes: %d\n", len(osm.Nodes))
	fmt.Printf("Ways: %d\n", len(osm.Ways))
	fmt.Printf("Rels: %d\n", len(osm.Relations))
	// Darstellung
	draw(osm, bbox)
}

func draw(osm data.OsmData, bbox geo.Bbox) {
	c0 := bbox.Coord0
	c1 := bbox.Coord1
	proj := geo.NewMercatorProjection()
	// 20 mm je km = 1:50_000, 5 Punkte je mm
	tr := geo.NewTransformer(proj, bbox, 20, 5, geo.ORIENT_NEGATIVE)
	p0 := tr.Transform(c0)
	p1 := tr.Transform(c1)
	wi := int(p1.X - p0.X)
	he := int(p0.Y - p1.Y)
	log.Printf("wi %d he %d\n", wi, he)
	c := image.NewRGBA(image.Rect(0, 0, wi, he))
	ctx := draw2dimg.NewGraphicContext(c)
	ctx.SetStrokeColor(color.RGBA{0, 0, 0, 0xff})
	ctx.SetLineWidth(2)
	for _, way := range osm.Ways {
		isStart := true
		for _, ref := range way {
			c := osm.Nodes[ref]
			p := tr.Transform(c)
			if isStart {
				isStart = false
				ctx.MoveTo(p.X, p.Y)
			} else {
				ctx.LineTo(p.X, p.Y)
			}
			// TODO Zeichne p
		}
		ctx.Stroke()
	}
	draw2dimg.SaveToPngFile("map_query.png", c)
}
