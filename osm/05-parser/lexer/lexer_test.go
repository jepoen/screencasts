package lexer

import (
	"log"
	"os"
	"testing"
)

func TestBlank(t *testing.T) {
	content := `äü
2 nd // comment
  /* äöüß */ (4, 5)
  // comment
`
	expect := []string{
		"[INV:ä]", "[INV:ü]", "[INT:2]", "[ID:nd]", "[(:(]", "[INT:4]",
		"[,:,]", "[INT:5]", "[):)]",
	}
	lexer := NewLexer("main", content)
	res := []string{}
	for tok := lexer.Next(); !tok.IsEof(); tok = lexer.Next() {
		res = append(res, tok.String())
	}
	log.Println(res)
	if !eqStrList(expect, res) {
		t.Errorf("expected %s got %s", expect, res)
	}
}

// Test Lesen aus Datei
func TestLexerFromFile(t *testing.T) {
	content := `
	a , // xxx
	b config,abc(draw)include
	`
	expect := []string{
		"[ID:a]", "[,:,]", "[ID:b]", "[CONF:config]",
		"[,:,]", "[ID:abc]", "[(:(]", "[DRAW:draw]",
		"[):)]", "[INC:include]",
	}
	tmpFile, err := os.CreateTemp("", "test")
	if err != nil {
		t.Errorf("Cannot create temp file %s: %s", tmpFile.Name(), err)
	}
	defer tmpFile.Close()
	tmpFile.WriteString(content)
	lexer := NewLexerFromFile(tmpFile.Name())
	os.Remove(tmpFile.Name())
	res := []string{}
	for tok := lexer.Next(); !tok.IsEof(); tok = lexer.Next() {
		res = append(res, tok.String())
	}
	log.Println(res)
	if !eqStrList(expect, res) {
		t.Errorf("expected %s got %s", expect, res)
	}
}

func eqStrList(s0, s1 []string) bool {
	if len(s0) != len(s1) {
		return false
	}
	for i := 0; i < len(s0); i++ {
		if s0[i] != s1[i] {
			return false
		}
	}
	return true
}
