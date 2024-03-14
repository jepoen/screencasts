package token

import "fmt"

// Ergänzen von Tokens:
// - TokenType-Konstante definieren
// - tokenStr-Konstante definieren
// - Zuordnung Eingabe → Token
//   - Einzelzeichen: in charTokens
//   - Digrams: in digrams
//   - Schlüsselwörter: in keywords

type TokenType int

const (
	EOF TokenType = iota
	INVALID
	// char tokens
	COMMA
	LPAREN
	RPAREN
	LBRACKET
	RBRACKET
	ASSIGN
	DEFAULT
	// digraphs
	EQ
	NE
	// identifier
	ID
	// keywords
	CONFIG
	INC
	QUERY
	DRAW
	FILTER
	STYLE
	// numbers
	INT
	FLOAT
	HEX
	// string
	STR
)

var tokenStr = map[TokenType]string{
	EOF:      "EOF",
	INVALID:  "INV",
	COMMA:    ",",
	LPAREN:   "(",
	RPAREN:   ")",
	LBRACKET: "[",
	RBRACKET: "]",
	ASSIGN:   "=",
	DEFAULT:  "|",
	EQ:       "==",
	NE:       "!=",
	ID:       "ID",
	CONFIG:   "CONF",
	INC:      "INC",
	DRAW:     "DRAW",
	FILTER:   "FILT",
	STYLE:    "STY",
	INT:      "INT",
	FLOAT:    "FLOAT",
	HEX:      "HEX",
	STR:      "STR",
}

func (tt TokenType) String() string {
	if s, ok := tokenStr[tt]; ok {
		return s
	} else {
		return "???"
	}
}

type Token struct {
	Type  TokenType
	Value string
}

func (tok Token) String() string {
	return fmt.Sprintf("[%s:%s]", tok.Type, tok.Value)
}

func (tok *Token) IsEof() bool {
	return tok.Type == EOF
}

func (tok *Token) IsInvalid() bool {
	return tok.Type == INVALID
}

// Ist das Token ein Wert (Zahl oder String)?
func (tok *Token) IsValue() bool {
	valTokens := map[TokenType]bool{
		INT:   true,
		FLOAT: true,
		HEX:   true,
		STR:   true,
	}
	_, ok := valTokens[tok.Type]
	return ok
}

var charTokens = map[rune]TokenType{
	',': COMMA,
	'(': LPAREN,
	')': RPAREN,
	'[': LBRACKET,
	']': RBRACKET,
	'=': ASSIGN,
	'|': DEFAULT,
}

func Char(r rune) (Token, bool) {
	if tt, ok := charTokens[r]; ok {
		return Token{tt, string(r)}, true
	} else {
		return Token{INVALID, string(r)}, false
	}
}

var digrams = map[string]TokenType{
	"==": EQ,
	"!=": NE,
}

func Digram(r0, r1 rune) (Token, bool) {
	s := string([]rune{r0, r1})
	if tt, ok := digrams[s]; ok {
		return Token{tt, s}, true
	} else {
		return Token{INVALID, s}, false
	}
}

var keywords = map[string]TokenType{
	"config":  CONFIG,
	"include": INC,
	"query":   QUERY,
	"draw":    DRAW,
	"filter":  FILTER,
	"style":   STYLE,
}

func Keyword(s string) (Token, bool) {
	if tt, ok := keywords[s]; ok {
		return Token{tt, s}, true
	} else {
		return Token{INVALID, s}, false
	}
}
