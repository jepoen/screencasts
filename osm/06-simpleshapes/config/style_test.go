package config

import (
	"06-simpleshapes/lexer"
	"06-simpleshapes/parser"
	"strings"
	"testing"

	"golang.org/x/image/colornames"
)

func TestStyle(t *testing.T) {
	baseConfig := `
config[overpassUrl] = "https://overpass-api.de/api/interpreter"
config[bbox] = (13.9,50.8,14.2,51.0)
config[outputFile] = "map0.png"
config[projection] = "Mercator"
config[mmPerKm] = 20
config[ptPerMm] = 5
`
	testCases := []struct {
		inp      string
		expected *Style
	}{
		{
			inp: "{{conf}}draw[x] (style (lineWidth=3, fillColor='red'))",
			expected: &Style{
				LineWidth: 3,
				FillColor: colornames.Red,
			},
		},
		{
			inp: "{{conf}}draw[y] (style (lineDash=(1,2), dashOffset=2, closeWays=1))",
			expected: &Style{
				LineDash:   []float64{1, 2},
				DashOffset: 2,
				CloseWays:  true,
			},
		},
	}
	style0 := CreateBaseStyle()
	for _, c := range testCases {
		inp := strings.ReplaceAll(c.inp, "{{conf}}", baseConfig)
		p := parser.NewParser(lexer.NewLexer("", inp))
		ast := p.ParseAll()
		env := NewEnvironment(ast)
		drawStep0 := ast.StepList[0].(*parser.DrawStep)
		style := EvalStyle(drawStep0.Style, env)
		//log.Println(style)
		if !eqStyle(c.expected, style) {
			t.Errorf("expected %s, got %s", c.expected, style)
		}
		if !eqStyle(env.Styles["_"], style0) {
			t.Errorf("Base Style changed, expected %s, got %s",
				style0, env.Styles["_"])
		}
	}
}

func eqStyle(s1, s2 *Style) bool {
	if s1.LineWidth != s2.LineWidth {
		return false
	}
	if s1.DashOffset != s2.DashOffset {
		return false
	}
	if s1.CloseWays != s2.CloseWays {
		return false
	}
	if s1.StrokeColor != s2.StrokeColor {
		return false
	}
	if s1.FillColor != s2.FillColor {
		return false
	}
	if !eqFloatList(s1.LineDash, s2.LineDash) {
		return false
	}
	return true
}

func eqFloatList(l1, l2 []float64) bool {
	if len(l1) != len(l2) {
		return false
	}
	for i := 0; i < len(l1); i++ {
		if l1[i] != l2[i] {
			return false
		}
	}
	return true
}
