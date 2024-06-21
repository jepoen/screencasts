package data

import (
	"08-output/geo"
	"encoding/xml"
	"errors"
	"io"
	"log"
	"math"
	"net/http"
	"strconv"
	"strings"
)

func GetData(url string, query string) (*OsmData, error) {
	stream, err := httpQuery(url, query)
	if err != nil {
		return &OsmData{}, err
	}
	return parseXml(stream)
}

func httpQuery(url string, query string) (io.ReadCloser, error) {
	resp, err := http.Post(url, "text/plain", strings.NewReader(query))
	return resp.Body, err
}

// Erzeugt ein Dictionary der Node-Koordinaten
// sowie ein Dictionary der Wege mit den Node-Referenzen
func parseXml(stream io.Reader) (*OsmData, error) {
	d := xml.NewDecoder(stream)
	osm := NewOsmData()
	var currWay IdList
	var currId int64
	var currMemberList RelMemberList
	var currTags TagMap
	for {
		tok, err := d.Token()
		if tok == nil || err == io.EOF {
			break
		}
		if err != nil {
			return osm, err
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
				osm.Nodes[id] = geo.Coord{Lon: lon, Lat: lat}
				currId = id // fehlt im Screencast
				currTags = TagMap{}
			case "way":
				id, err := getId(ty.Attr, "id")
				if err != nil {
					log.Fatal(err)
				}
				currId = id
				currWay = IdList{}
				currTags = TagMap{}
			case "nd":
				ref, err := getId(ty.Attr, "ref")
				if err != nil {
					return osm, err
				}
				currWay = append(currWay, ref)
			case "relation":
				id, err := getId(ty.Attr, "id")
				if err != nil {
					return osm, err
				}
				currId = id
				currMemberList = RelMemberList{}
				currTags = TagMap{}
			case "member":
				member, err := getMemberAttr(ty.Attr)
				if err != nil {
					return osm, err
				}
				currMemberList = append(currMemberList, member)
			case "tag":
				key, val, err := getTag(ty.Attr)
				if err != nil {
					return osm, err
				}
				currTags[key] = val
			}
		case xml.EndElement:
			switch ty.Name.Local {
			case "node":
				if len(currTags) > 0 {
					osm.NodeTags[currId] = currTags
				}
			case "way":
				osm.Ways[currId] = currWay
				if len(currTags) > 0 {
					osm.WayTags[currId] = currTags
				}
			case "relation":
				osm.Relations[currId] = currMemberList
				if len(currTags) > 0 {
					osm.RelTags[currId] = currTags // korrigiert gegenüber Screencast
				}
			}
		}
	}
	return osm, nil
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

func getMemberAttr(attrs []xml.Attr) (RelMember, error) {
	var ref int64 = -1
	var mType MemberType = MEMBER_NIL
	role := ""
	var err error = nil
	for _, attr := range attrs {
		switch attr.Name.Local {
		case "ref":
			ref, err = strconv.ParseInt(attr.Value, 10, 64)
		case "type":
			switch attr.Value {
			case "node":
				mType = MEMBER_NODE
			case "way":
				mType = MEMBER_WAY
			case "relation":
				mType = MEMBER_REL
			}
		case "role":
			role = attr.Value
		}
		if err != nil {
			return RelMember{ref, mType, role}, err
		}
	}
	if ref < 0 || mType == MEMBER_NIL {
		err = errors.New("missing attribute value")
	}
	return RelMember{ref, mType, role}, err
}

func getTag(attrs []xml.Attr) (string, string, error) {
	k := ""
	v := ""
	for _, attr := range attrs {
		switch attr.Name.Local {
		case "k":
			k = attr.Value
		case "v":
			v = attr.Value
		}
	}
	if k == "" {
		return k, v, errors.New("missing k in tag")
	}
	return k, v, nil
}
