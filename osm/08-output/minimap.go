package main

// Zeichnen der Testkarte
import (
	"08-output/config"
	"08-output/parser"
	"08-output/process"
	"log"
	"os"
)

func main() {
	args := os.Args
	if len(args) != 2 {
		log.Fatalf("Usage %s config_file", args[0])
	}
	p := parser.NewParserFromFile(args[1])
	ast := p.ParseAll()
	env := config.NewEnvironment(ast)
	log.Println(env)
	// Process Steps
	process.ProcessAst(env, ast)
	// Darstellung TODO
	//draw(env)
	env.SaveMap()
}

/*
func draw(env *config.Environment) {
	env.Ctx.SetFillColor(color.White)
	env.Ctx.Clear()
	render.RenderLakes(env.Ctx, *env.Data, *env.Tr) // nicht im Video
	render.RenderWoods(env.Ctx, *env.Data, *env.Tr)
	render.RenderRivers(env.Ctx, *env.Data, *env.Tr)
	render.RenderRailways(env.Ctx, *env.Data, *env.Tr)
	render.RenderHighways(env.Ctx, *env.Data, *env.Tr)
}
*/
