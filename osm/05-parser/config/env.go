package config

import (
	"05-parser/data"
	"05-parser/geo"
	"05-parser/parser"
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
	Query  map[string]string
	Data   *data.OsmData
}

func NewEnvironment(ast *parser.Ast) *Environment {
	env := &Environment{
		Config: EvalConfig(ast),
		Query:  map[string]string{},
	}
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
	return env
}

func (env *Environment) String() string {
	return fmt.Sprintf("Config: %s", env.Config)
}
