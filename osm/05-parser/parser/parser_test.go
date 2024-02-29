package parser

import (
	"05-parser/lexer"
	"log"
	"os"
	"strings"
	"testing"
)

/*
Beendet Programm mit exit code 1, Test nur mit Subprocess möglich

	func TestInvalidEntry(t *testing.T) {
		inp := "123"
		lx := lexer.NewLexer("-", inp)
		p := NewParser(lx)
		p.ParseAll()
	}
*/
func TestConfig(t *testing.T) {
	inp := `
config[bbox] = (1, 2.3, 4.5, #6)
config[seq] = "albers" 50 70.5
query[xyz] = "out qt"
`
	expected := &Ast{
		ConfigList: []ConfigNode{
			&ConfigEntry{Key: "bbox", Values: []ValueNode{
				&NumValue{NUM_INT, "1"},
				&NumValue{NUM_FLOAT, "2.3"},
				&NumValue{NUM_FLOAT, "4.5"},
				&NumValue{NUM_HEX, "6"},
			}},
			&ConfigEntry{Key: "seq", Values: []ValueNode{
				&StrValue{"albers"},
				&NumValue{NUM_INT, "50"},
				&NumValue{NUM_FLOAT, "70.5"},
			}},
		},
		StepList: []StepNode{
			&QueryStep{Key: "xyz", Query: &StrValue{"out qt"}},
		},
	}
	lx := lexer.NewLexer("-", inp)
	p := NewParser(lx)
	ast := p.ParseAll()
	log.Println(ast)
	log.Println(expected)
	if ast.String() != expected.String() {
		t.Errorf("\nexpected %s\ngot      %s",
			expected, ast,
		)
	}
}

func TestInc(t *testing.T) {
	inpTemplate := `
config[a] = 1
include "{{incFile}}"
config[c] = "s"
	`
	incInp := `config[b] = (1)
query[q]
`
	expected := &Ast{
		ConfigList: []ConfigNode{
			&ConfigEntry{"a", []ValueNode{
				&NumValue{NUM_INT, "1"},
			}},
			&ConfigEntry{"b", []ValueNode{
				&NumValue{NUM_INT, "1"},
			}},
			&ConfigEntry{"c", []ValueNode{
				&StrValue{"s"},
			}},
		},
		StepList: []StepNode{
			&QueryStep{"q", nil},
		},
	}
	fi, err := os.CreateTemp("", "tst")
	if err != nil {
		t.Error(err)
	}
	defer fi.Close()
	fi.WriteString(incInp)
	inp := strings.ReplaceAll(inpTemplate, "{{incFile}}", fi.Name())
	lx := lexer.NewLexer("-", inp)
	p := NewParser(lx)
	ast := p.ParseAll()
	log.Println(ast)
	if ast.String() != expected.String() {
		t.Errorf("\nexpected %s\ngot      %s",
			expected, ast,
		)
	}
}
