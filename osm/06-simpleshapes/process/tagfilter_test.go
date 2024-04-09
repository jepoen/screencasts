package process

import (
	"06-simpleshapes/data"
	"06-simpleshapes/lexer"
	"06-simpleshapes/parser"
	"sort"
	"testing"
)

func TestTagFilter(t *testing.T) {
	cases := []struct {
		filter string
		tags   map[int64]data.TagMap
		expect data.IdList
	}{
		{
			filter: "draw[x] (filter (k=='v'))",
			tags: map[int64]data.TagMap{
				1: {"k": "v"},
				2: {"k": "x"},
				3: {"kk": "v"},
			},
			expect: data.IdList{1},
		},
		{
			filter: "draw[x] (filter ((k=='v' or k=='x') and m=='y'))",
			tags: map[int64]data.TagMap{
				1: {"k": "v", "m": "y"},
				2: {"k": "x", "m": "y", "x": "3"},
				3: {"k": "v"},
			},
			expect: data.IdList{1, 2},
		},
		{
			filter: "draw[x] (filter (k|'v' !='v'))",
			tags: map[int64]data.TagMap{
				1: {"k": "v", "m": "y"},
				2: {"m": "y", "x": "3"},
				3: {"k": "x", "l": "w"},
				4: {"l": "x"},
			},
			expect: data.IdList{3},
		},
		{
			filter: "draw[x] (filter (k or m))",
			tags: map[int64]data.TagMap{
				1: {"k": "v", "m": "y"},
				2: {"m": "y", "x": "3"},
				3: {"k": "x", "l": "w"},
				4: {"l": "x"},
			},
			expect: data.IdList{1, 2, 3},
		},
		{
			filter: "draw[x] (filter (not k and m=='v'))",
			tags: map[int64]data.TagMap{
				1: {"k": "v", "m": "v"},
				2: {"m": "v", "x": "3"},
				3: {"k": "x", "l": "w"},
				4: {"l": "x", "m": "3"},
			},
			expect: data.IdList{2},
		},
		{
			filter: "draw[x] (filter (not (k=='v' and m=='w')))",
			tags: map[int64]data.TagMap{
				1: {"k": "v", "m": "w"},
				2: {"m": "v", "x": "3"},
				3: {"k": "x", "l": "w"},
				4: {"l": "x", "m": "3"},
			},
			expect: data.IdList{2, 3, 4},
		},
		// next case ...
	}
	for _, c := range cases {
		lx := lexer.NewLexer("", c.filter)
		ast := parser.NewParser(lx).ParseAll()
		//log.Println(ast)
		drawStep := ast.StepList[0].(*parser.DrawStep)
		filter := drawStep.Filter
		//log.Println(filter)
		res := processFilter(filter, c.tags)
		//log.Println(res)
		if !eqIdLists(c.expect, res) {
			t.Errorf("expected %v got %v", c.expect, res)
		}
	}
}

func eqIdLists(expect data.IdList, res data.IdList) bool {
	if len(expect) != len(res) {
		return false
	}
	sort.Sort(res)
	for i := 0; i < len(res); i++ {
		if res[i] != expect[i] {
			return false
		}
	}
	return true
}
