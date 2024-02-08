package main

// Zeichnen der Testkarte
import (
	"02-projection/geo"
	"encoding/xml"
	"errors"
	"fmt"
	"image"
	"image/color"
	"io"
	"log"
	"math"
	"net/http"
	"strconv"
	"strings"

	"github.com/llgcode/draw2d/draw2dimg"
)

// Geographische Koordinaten eines Punktes
/*
type Coord struct {
	Lon float64
	Lat float64
}
*/

type IdList []int64

// Overpass API
const URL = "https://overpass-api.de/api/interpreter"

// Testabfrage
const QUERY_TEMPLATE = `
[bbox: {{bbox}}];
(
	way[railway="rail"];
    >;
 );
out qt;
`

func main() {
	// Abfrage
	bbox := geo.NewBbox(13.3, 50.8, 13.5, 51.0)
	query := strings.ReplaceAll(QUERY_TEMPLATE, "{{bbox}}", bbox.ToOverpassStr())
	stream, err := httpQuery(URL, query)
	if err != nil {
		log.Fatal(err)
	}
	/* Test Abfrage
	scanner := bufio.NewScanner(stream)
	for scanner.Scan() {
		fmt.Println(scanner.Text())
	}
	*/
	// XML-Verarbeitung
	nodes, ways := parseXml(stream)
	fmt.Println("Nodes:")
	fmt.Println(nodes)
	fmt.Println("Ways:")
	fmt.Println(ways)
	// Darstellung
	draw(ways, nodes, bbox)
}

func httpQuery(url string, query string) (io.ReadCloser, error) {
	resp, err := http.Post(url, "text/plain", strings.NewReader(query))
	return resp.Body, err
}

// Erzeugt ein Dictionary der Node-Koordinaten
// sowie ein Dictionary der Wege mit den Node-Referenzen
func parseXml(stream io.Reader) (map[int64]geo.Coord, map[int64]IdList) {
	d := xml.NewDecoder(stream)
	nodes := map[int64]geo.Coord{}
	ways := map[int64]IdList{}
	var currWay IdList
	var currWayId int64
	for {
		tok, err := d.Token()
		if tok == nil || err == io.EOF {
			break
		}
		switch ty := tok.(type) {
		case xml.StartElement:
			//fmt.Printf("%s %v\n", ty.Name.Local, ty.Attr)
			switch ty.Name.Local {
			case "node":
				id, lon, lat, err := getNodeAttr(ty.Attr)
				if err != nil {
					log.Fatal(err)
				}
				nodes[id] = geo.Coord{lon, lat}
			case "way":
				id, err := getId(ty.Attr, "id")
				if err != nil {
					log.Fatal(err)
				}
				currWayId = id
				currWay = IdList{}
			case "nd":
				ref, err := getId(ty.Attr, "ref")
				if err != nil {
					log.Fatal(err)
				}
				currWay = append(currWay, ref)
			}
		case xml.EndElement:
			switch ty.Name.Local {
			case "way":
				ways[currWayId] = currWay
			}
		}
	}
	return nodes, ways
}

// Extrahieren der ID und Koordinaten des Knotens
func getNodeAttr(attrs []xml.Attr) (int64, float64, float64, error) {
	var id int64 = -1
	var lon float64 = math.NaN()
	var lat float64 = math.NaN()
	var err error = nil
	for _, attr := range attrs {
		switch attr.Name.Local {
		case "id":
			id, err = strconv.ParseInt(attr.Value, 10, 64)

		case "lon":
			lon, err = strconv.ParseFloat(attr.Value, 64)
		case "lat":
			lat, err = strconv.ParseFloat(attr.Value, 64)
		}
		if err != nil {
			return id, lon, lat, err
		}
	}
	if id < 0 || math.IsNaN(lon) || math.IsNaN(lat) {
		err = errors.New("missing attribute value")
	}
	return id, lon, lat, err
}

// Extrahieren einer ID
func getId(attrs []xml.Attr, idName string) (int64, error) {
	for _, attr := range attrs {
		if attr.Name.Local == idName {
			return strconv.ParseInt(attr.Value, 10, 64)
		}
	}
	return -1, errors.New("missing id in attributes")
}

func draw(ways map[int64]IdList, nodes map[int64]geo.Coord, bbox geo.Bbox) {
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
	for _, way := range ways {
		isStart := true
		for _, ref := range way {
			c := nodes[ref]
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
