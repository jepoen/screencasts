package process

import (
	"08-output/config"
	"08-output/data"
	"08-output/parser"
	"log"
	"strings"

	"github.com/llgcode/draw2d/draw2dimg"
)

func ProcessAst(env *config.Environment, ast *parser.Ast) {
	for _, stepNode := range ast.StepList {
		switch ty := stepNode.(type) {
		case *parser.QueryStep:
			evalQuery(ty, env)
		case *parser.DrawStep: // vorher auskommentiert
			evalDraw(ty, env)
		default:
			log.Printf("unknown step: %s", ty)
		}
	}
}

func Save(env *config.Environment) {
	draw2dimg.SaveToPngFile(env.Config.OutputFile, env.Canvas)
}

func evalQuery(qs *parser.QueryStep, env *config.Environment) {
	if env.Query[qs.Key] == "" {
		if qs.Query == nil || qs.Query.Value == "" {
			log.Fatalf("query[%s]: missing query", qs.Key)
		}
		// TODO: qs.Query not empty
		env.Query[qs.Key] = qs.Query.Value
	} else {
		log.Fatalf("query: duplicate key %s", qs.Key)
	}
	qTempl := env.Query[qs.Key]
	query := strings.ReplaceAll(qTempl, "{{bbox}}", env.Config.Bbox.ToOverpassStr())
	log.Printf("query[%s] = %s\n", qs.Key, query)
	osm, err := data.GetData(env.Config.OverpassUrl, query)
	if err != nil {
		log.Fatal(err)
	}
	env.Data = osm
	log.Printf("Nodes %d Ways %d Rels: %d\n",
		len(env.Data.Nodes),
		len(env.Data.Ways),
		len(env.Data.Relations),
	)
}
