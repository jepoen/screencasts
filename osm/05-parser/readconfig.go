package main

import (
	"05-parser/config"
	"05-parser/parser"
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
	//log.Println(ast)
	conf := config.EvalConfig(ast)
	log.Println(conf)
}
