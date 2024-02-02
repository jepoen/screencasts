package main

import (
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
type Coord struct {
	Lon float64
	Lat float64
}

type IdList []int64

// Overpass API
const URL = "https://overpass-api.de/api/interpreter"

// Testabfrage
const QUERY_TEMPLATE = `
[bbox: %f,%f,%f,%f];
(
	way[railway="rail"];
    >;
 );
out qt;
`

func main() {
	// Abfrage
	p0 := Coord{13.3, 50.8}
	p1 := Coord{13.5, 51.0}
	query := fmt.Sprintf(QUERY_TEMPLATE, p0.Lat, p0.Lon, p1.Lat, p1.Lon)
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
	draw(ways, nodes, p0, p1)
}

func httpQuery(url string, query string) (io.ReadCloser, error) {
	resp, err := http.Post(url, "text/plain", strings.NewReader(query))
	return resp.Body, err
}

// Erzeugt ein Dictionary der Node-Koordinaten
// sowie ein Dictionary der Wege mit den Node-Referenzen
func parseXml(stream io.Reader) (map[int64]Coord, map[int64]IdList) {
	d := xml.NewDecoder(stream)
	nodes := map[int64]Coord{}
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
				nodes[id] = Coord{lon, lat}
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

func draw(ways map[int64]IdList, nodes map[int64]Coord, p0 Coord, p1 Coord) {
	scale := 2000.0
	wi := int((p1.Lon - p0.Lon) * scale)
	he := int((p1.Lat - p0.Lat) * scale)
	c := image.NewRGBA(image.Rect(0, 0, wi, he))
	ctx := draw2dimg.NewGraphicContext(c)
	ctx.SetStrokeColor(color.RGBA{0, 0, 0, 0xff})
	ctx.SetLineWidth(2)
	for _, way := range ways {
		isStart := true
		for _, ref := range way {
			p := nodes[ref]
			x, y := (p.Lon-p0.Lon)*scale, (p.Lat-p0.Lat)*scale
			if isStart {
				isStart = false
				ctx.MoveTo(x, y)
			} else {
				ctx.LineTo(x, y)
			}
			// TODO Zeichne p
		}
		ctx.Stroke()
	}
	draw2dimg.SaveToPngFile("map_query.png", c)
}
