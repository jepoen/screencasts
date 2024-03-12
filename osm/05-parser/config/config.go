package config

import (
	"05-parser/geo"
	"05-parser/parser"
	"fmt"
	"log"
	"path/filepath"
	"strconv"
	"strings"
)

type ConfigFuncs struct {
	eval  func([]parser.ValueNode)
	check func() bool
	str   func() string
}

type Config struct {
	OverpassUrl string
	Bbox        geo.Bbox
	OutputFile  string
	MmPerKm     float64
	PtPerMm     float64
	Projection  geo.Projection
	functions   map[string]ConfigFuncs
}

func NewConfig() *Config {
	c := &Config{
		functions: map[string]ConfigFuncs{},
	}
	c.registerFunctions("bbox", c.evalBbox, c.checkBbox, c.strBbox)
	c.registerFunctions("overpass_url", c.evalOverpassUrl, c.checkOverpassUrl, c.strOverpassUrl)
	c.registerFunctions("output_file", c.evalOutputFile, c.checkOutputFile, c.strOutputFile)
	c.registerFunctions("mm_per_km", c.evalMmPerKm, c.checkMmPerKm, c.strMmPerKm)
	c.registerFunctions("pt_per_mm", c.evalPtPerMm, c.checkPtPerMm, c.strPtPerMM)
	c.registerFunctions("projection", c.evalProjection, c.checkProjection, c.strProjection)
	return c
}

func EvalConfig(ast *parser.Ast) *Config {
	c := NewConfig()
	for _, entry := range ast.ConfigList {
		switch ty := entry.(type) {
		case *parser.ConfigEntry:
			if cf, ok := c.functions[ty.Key]; ok {
				cf.eval(ty.Values)
			} else {
				log.Printf("config: unknown option %s", ty.Key)
			}
		default:
			log.Printf("unknown config entry %s", entry)
		}
	}
	return c
}

func (c *Config) String() string {
	res := []string{}
	res = append(res, "Config:")
	for k, v := range c.functions {
		res = append(res, fmt.Sprintf("[%s] %s", k, v.str()))
	}
	return strings.Join(res, "\n")
}

func (c *Config) registerFunctions(
	key string,
	evalFunc func([]parser.ValueNode),
	checkFunc func() bool,
	strFunc func() string,
) {
	c.functions[key] = ConfigFuncs{evalFunc, checkFunc, strFunc}
}

func (c *Config) evalBbox(values []parser.ValueNode) {
	if len(values) != 4 {
		log.Fatalf("bbox: 4 values needed, got %d", len(values))
	}
	boxValues := []float64{}
	for i, v := range values {
		if num, ok := v.(*parser.NumValue); ok {
			if num.NumType == parser.NUM_FLOAT || num.NumType == parser.NUM_INT {
				val, _ := strconv.ParseFloat(num.Value, 64)
				boxValues = append(boxValues, val)
			} else {
				log.Fatalf("bbox: param %d has wrong type, got %s",
					i+1, num.Value,
				)
			}
		} else {
			log.Fatalf("bbox: param %d is not a number, got %s",
				i+1, v,
			)
		}
	}
	c.Bbox = geo.NewBbox(boxValues[0], boxValues[1], boxValues[2], boxValues[3])
}

func (c *Config) checkBbox() bool {
	// TODO: Funktionen besser als Bbox-Methoden realisieren
	if c.Bbox.Coord0.Lon >= c.Bbox.Coord1.Lon || c.Bbox.Coord0.Lat >= c.Bbox.Coord1.Lat {
		log.Fatalf("bbox: wrong size: %s", c.Bbox)
	}
	if !checkLon(c.Bbox.Coord0) || !checkLon(c.Bbox.Coord1) {
		log.Fatalf("bbox: wrong longitude: %s", c.Bbox)
	}
	if !checkLat(c.Bbox.Coord0) || !checkLat(c.Bbox.Coord1) {
		log.Fatalf("bbox: wrong longitude: %s", c.Bbox)
	}
	return true
}

func checkLon(coord geo.Coord) bool {
	return coord.Lon >= -180.0 && coord.Lon <= 180.0
}

func checkLat(coord geo.Coord) bool {
	return coord.Lat >= -90 && coord.Lat <= 90
}

func (c *Config) strBbox() string {
	return c.Bbox.String()
}

func (c *Config) evalOverpassUrl(values []parser.ValueNode) {
	if len(values) != 1 {
		log.Fatalf("overpass_url: 1 value needed, got %d", len(values))
	}
	if strVal, ok := values[0].(*parser.StrValue); ok {
		c.OverpassUrl = strVal.Value
	} else {
		log.Fatalf("overpass_url: need string, got %s", values[0])
	}
}

func (c *Config) checkOverpassUrl() bool {
	// TODO: string nur dann leer, wenn Kommando da
	// sonst URL-Syntax prüfen
	return true
}

func (c *Config) strOverpassUrl() string {
	return c.OverpassUrl
}
func (c *Config) evalOutputFile(values []parser.ValueNode) {
	if len(values) != 1 {
		log.Fatalf("output_file: 1 value needed, got %d", len(values))
	}
	if strVal, ok := values[0].(*parser.StrValue); ok {
		c.OutputFile = strVal.Value
	} else {
		log.Fatalf("output_file: need string, got %s", values[0])
	}
}

func (c *Config) checkOutputFile() bool {
	extensions := map[string]bool{
		".png": true,
		".pdf": true,
	}
	ext := filepath.Ext(c.OutputFile)
	if !extensions[ext] {
		log.Fatalf("output_file: wrong extension, got %s", ext)
	}
	return true
}

func (c *Config) strOutputFile() string {
	return c.OutputFile
}

func (c *Config) evalMmPerKm(values []parser.ValueNode) {
	if len(values) != 1 {
		log.Fatalf("mm_per_km: 1 value needed, got %d", len(values))
	}
	if num, ok := values[0].(*parser.NumValue); ok {
		if num.NumType == parser.NUM_FLOAT || num.NumType == parser.NUM_INT {
			c.MmPerKm, _ = strconv.ParseFloat(num.Value, 64)
		} else {
			log.Fatalf("mm_per_km: param has wrong type, got %s", num.Value)
		}
	} else {
		log.Fatalf("mm_per_km: param is not a number, got %s", values[0])
	}
}

func (c *Config) checkMmPerKm() bool {
	if c.MmPerKm <= 0 {
		log.Fatalf("mm_per_km: must be positive, got %f", c.MmPerKm)
	}
	return true
}

func (c *Config) strMmPerKm() string {
	return fmt.Sprintf("%f", c.MmPerKm)
}

func (c *Config) evalPtPerMm(values []parser.ValueNode) {
	if len(values) != 1 {
		log.Fatalf("pt_per_mm: 1 value needed, got %d", len(values))
	}
	if num, ok := values[0].(*parser.NumValue); ok {
		if num.NumType == parser.NUM_FLOAT || num.NumType == parser.NUM_INT {
			c.PtPerMm, _ = strconv.ParseFloat(num.Value, 64)
		} else {
			log.Fatalf("pt_per_mm: param has wrong type, got %s", num.Value)
		}
	} else {
		log.Fatalf("pt_per_mm: param is not a number, got %s", values[0])
	}
}

func (c *Config) checkPtPerMm() bool {
	if c.PtPerMm <= 0 {
		log.Fatalf("pt_per_mm: must be positive, got %f", c.PtPerMm)
	}
	return true
}

func (c *Config) strPtPerMM() string {
	return fmt.Sprintf("%f", c.PtPerMm)
}

func (c *Config) evalProjection(values []parser.ValueNode) {
	if len(values) < 1 {
		log.Fatalf("overpass_url: at least 1 value needed, got %d", len(values))
	}
	if projType, ok := values[0].(*parser.StrValue); ok {
		switch strings.ToLower(projType.Value) {
		case "mercator":
			c.Projection = &geo.MercatorProjection{}
		default:
			log.Fatalf("projection: unknown %s", projType.Value)
		}
	} else {
		log.Fatalf("overpass_url: need string, got %s", values[0])
	}
}

func (c *Config) checkProjection() bool {
	if c.Projection == nil {
		log.Fatal("Missing projection")
	}
	return true
}

func (c *Config) strProjection() string {
	return c.Projection.String()
}
