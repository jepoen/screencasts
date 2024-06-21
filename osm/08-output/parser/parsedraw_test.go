package parser

import (
	"08-output/lexer"
	"testing"
)

// Compare string results to simplify construction the expected result
func TestDraw(t *testing.T) {
	testCases := []struct {
		input    string
		expected string
	}{
		{"draw[xxx] (filter (key=='val'))",
			"[] [(draw xxx <nil> (== key \"val\"))]\n"},
		{"draw[xxx] (filter ('key'|3=='val' and not bar))",
			"[] [(draw xxx <nil> (and (== (keyOrDefault key 3) \"val\") (not (exists bar))))]\n"},
		{"draw[zzz] (filter (key or bar != 's'))",
			"[] [(draw zzz <nil> (or (exists key) (!= bar \"s\")))]\n"},
		{"draw[a] (filter (not key or not bar|'s' ~ 's*'))",
			"[] [(draw a <nil> (or (not (exists key)) (not (~ (keyOrDefault bar \"s\") \"s*\"))))]\n"},
		{"draw[boolseq] (filter (a==3 and b!=2 or c))",
			"[] [(draw boolseq <nil> (or (and (== a 3) (!= b 2)) (exists c)))]\n"},
	}
	for _, c := range testCases {
		lx := lexer.NewLexer("", c.input)
		p := NewParser(lx)
		ast := p.ParseAll()
		if ast.String() != c.expected {
			t.Errorf("\nexpected %sgot      %s", c.expected, ast)
		}
	}
}

func TestStyle(t *testing.T) {
	testCases := []struct {
		input    string
		expected string
	}{
		{"draw[xxx] (filter (key) style (s=3))",
			"[] [(draw xxx (style _ ((option s 3))) (exists key))]\n"},
		{"draw[x2] (style[s] (s=3, t='xyz', a=id, b=(1, '2', #3)))",
			"[] [(draw x2 (style s ((option s 3), (option t \"xyz\"), (option a id), (option b (1, \"2\", #3)))) <nil>)]\n"},
	}
	for _, c := range testCases {
		lx := lexer.NewLexer("", c.input)
		p := NewParser(lx)
		ast := p.ParseAll()
		if ast.String() != c.expected {
			t.Errorf("\nexpected %sgot      %s", c.expected, ast)
		}
	}
}
