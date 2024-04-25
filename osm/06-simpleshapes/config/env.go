package config

import (
	"06-simpleshapes/data"
	"06-simpleshapes/geo"
	"06-simpleshapes/parser"
	"fmt"
	"image"
	"log"

	"github.com/llgcode/draw2d/draw2dimg"
)

type Environment struct {
	Config *Config
	Tr     *geo.Transformer
	Canvas *image.RGBA
	Ctx    *draw2dimg.GraphicContext
	Styles map[string]*Style
	Query  map[string]string
	Data   *data.OsmData
}

func NewEnvironment(ast *parser.Ast) *Environment {
	env := &Environment{
		Styles: map[string]*Style{},
		Query:  map[string]string{},
	}
	env.EvalSettings(ast)
	env.Tr = geo.NewTransformer(
		env.Config.Projection,
		env.Config.Bbox,
		env.Config.MmPerKm,
		env.Config.PtPerMm,
		geo.ORIENT_NEGATIVE,
	)
	log.Println(env.Tr.ImgRect())
	env.Canvas = image.NewRGBA(env.Tr.ImgRect())
	env.Ctx = draw2dimg.NewGraphicContext(env.Canvas)
	env.Styles["_"] = CreateBaseStyle()
	return env
}

func (env *Environment) String() string {
	return fmt.Sprintf("Config: %s", env.Config)
}

func (env *Environment) EvalSettings(ast *parser.Ast) {
	env.Config = NewConfig()
	// TODO: Basic Styles, Paths, Symbols, ...
	for _, entry := range ast.ConfigList {
		switch ty := entry.(type) {
		case *parser.ConfigEntry:
			if cf, ok := env.Config.functions[ty.Key]; ok {
				cf.eval(ty.Values)
			} else {
				log.Printf("config: unknown option %s", ty.Key)
			}
		default:
			log.Printf("unknown setting %s", entry)
		}
	}
}
