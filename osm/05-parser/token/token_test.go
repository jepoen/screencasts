package token

import "testing"

func TestTokens(t *testing.T) {
	testCases := []struct {
		inp      Token
		expected string
	}{
		{Token{EOF, ""}, "[EOF:]"},
		{Token{INVALID, "?"}, "[INV:?]"},
	}
	for _, c := range testCases {
		res := c.inp.String()
		if res != c.expected {
			t.Errorf("expected %s got %s", res, c.expected)
		}
	}
}

func TestKeywords(t *testing.T) {
	testCases := []struct {
		s   string
		tok Token
		ok  bool
	}{
		{"config", Token{CONFIG, "config"}, true},
		{"foo", Token{INVALID, "foo"}, false},
	}
	for _, c := range testCases {
		tok, ok := Keyword(c.s)
		if ok != c.ok {
			t.Errorf("%s: expected %T got %T", c.s, c.ok, ok)
		}
		if tok.String() != c.tok.String() {
			t.Errorf("%s: expected %s got %s", c.s, c.tok, tok)
		}
	}
}

func TestDigraphs(t *testing.T) {
	testCases := []struct {
		r0  rune
		r1  rune
		tok Token
		ok  bool
	}{
		{'=', '=', Token{EQ, "=="}, true},
		{'=', '!', Token{INVALID, "=!"}, false},
	}
	for _, c := range testCases {
		tok, ok := Digram(c.r0, c.r1)
		if ok != c.ok {
			t.Errorf("%c%c: expected %T got %T", c.r0, c.r1, c.ok, ok)
		}
		if tok.String() != c.tok.String() {
			t.Errorf("%c%c: expected %s got %s", c.r0, c.r1, c.tok, tok)
		}
	}
}
