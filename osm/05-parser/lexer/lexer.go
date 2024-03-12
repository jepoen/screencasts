package lexer

import (
	"05-parser/token"
	"log"
	"os"
	"regexp"
	"strings"
	"unicode"
	"unicode/utf8"
)

const _EOF_CH = '\x00'

type Lexer struct {
	fileName string
	content  string
	curLine  int
	tokLine  int
	pos      int
	ch       rune
	readPos  int
}

func (lx *Lexer) IsEof() bool {
	return lx.ch == _EOF_CH
}

func (lx *Lexer) readChar() {
	rLen := 0
	if lx.ch == '\n' {
		lx.curLine++
	}
	if lx.readPos >= len(lx.content) {
		lx.ch = _EOF_CH
	} else {
		lx.ch, rLen = utf8.DecodeRuneInString(lx.content[lx.readPos:])
	}
	lx.pos = lx.readPos
	lx.readPos += rLen
}

func (lx *Lexer) peekChar() rune {
	if lx.readPos >= len(lx.content) {
		return _EOF_CH
	}
	ch, _ := utf8.DecodeRuneInString(lx.content[lx.readPos:])
	return ch
}

func (lx *Lexer) skipTo(pos int) {
	for lx.pos < pos {
		lx.readChar()
	}
}

func NewLexer(fileName, content string) *Lexer {
	lx := &Lexer{
		fileName: fileName,
		content:  content,
		curLine:  1,
		pos:      0,
		readPos:  0,
	}
	lx.readChar()
	return lx
}

func NewLexerFromFile(fileName string) *Lexer {
	bytes, err := os.ReadFile(fileName)
	if err != nil {
		log.Fatalf("Error reading file %s: %s", fileName, err)
	}
	return NewLexer(fileName, string(bytes))
}

func (lx *Lexer) Next() token.Token {
	lx.skipBlankOrComment()
	lx.tokLine = lx.curLine
	if lx.IsEof() {
		return token.Token{Type: token.EOF, Value: ""}
	}
	if tok, ok := token.Digram(lx.ch, lx.peekChar()); ok {
		lx.readChar()
		lx.readChar()
		return tok
	} else if tok, ok := token.Char(lx.ch); ok {
		lx.readChar()
		return tok
		// Fortsetzung
	} else if isAsciiDigitOrSign(lx.ch) {
		return lx.readNumber()
	} else if lx.ch == '#' {
		return lx.readHexNumber()
	} else if isAsciiLetter(lx.ch) {
		val := lx.readIdentifier()
		if tok, ok := token.Keyword(val); ok {
			return tok
		} else {
			return token.Token{Type: token.ID, Value: val}
		}
	} else if isAsciiStrDelim(lx.ch) {
		return lx.readStr()
	}
	tok := token.Token{Type: token.INVALID, Value: string(lx.ch)}
	lx.readChar()
	return tok
}

func (lx *Lexer) FileName() string {
	return lx.fileName
}

func (lx *Lexer) TokenLine() int {
	return lx.tokLine
}

func (lx *Lexer) skipBlankOrComment() {
	for !lx.IsEof() {
		if unicode.IsSpace(lx.ch) {
			lx.readChar()
		} else if lx.ch == '/' && lx.peekChar() == '/' {
			lx.skipToNl()
		} else if lx.ch == '/' && lx.peekChar() == '*' {
			lx.skipBlockComment()
		} else {
			return
		}
	}
}

func (lx *Lexer) skipToNl() {
	for !lx.IsEof() && lx.ch != '\n' {
		lx.readChar()
	}
}

func (lx *Lexer) skipBlockComment() {
	idx := strings.Index(lx.content[lx.pos:], "*/")
	if idx < 0 {
		log.Fatalf("File %s, line %d: comment not closed",
			lx.fileName, lx.curLine,
		)
	}
	lx.skipTo(lx.pos + idx)
	lx.readChar()
	lx.readChar()
}

// Fortsetzung
func (lx *Lexer) readNumber() token.Token {
	rex := regexp.MustCompile(`^[+-]?[0-9]+(\.[0-9]+)?`)
	m := rex.FindStringSubmatchIndex(lx.content[lx.pos:])
	if m == nil {
		ch := lx.ch
		lx.readChar()
		return token.Token{Type: token.INVALID, Value: string(ch)}
	}
	val := lx.content[lx.pos : lx.pos+m[1]]
	tt := token.INT
	if m[2] >= 0 {
		tt = token.FLOAT
	}
	lx.skipTo(lx.pos + m[1])
	return token.Token{Type: tt, Value: val}
}

func (lx *Lexer) readIdentifier() string {
	rex := regexp.MustCompile(`^[a-zA-Z_][a-zA-Z0-9_]*`)
	m := rex.FindStringIndex(lx.content[lx.pos:])
	// m != nil
	val := lx.content[lx.pos : lx.pos+m[1]]
	lx.skipTo(lx.pos + m[1])
	return val
}

func (lx *Lexer) readStr() token.Token {
	delim := lx.ch
	delimCnt := 0
	for lx.ch == delim {
		delimCnt++
		lx.readChar()
	}
	fullDelim := strings.Repeat(string(delim), delimCnt)
	idx := strings.Index(lx.content[lx.pos:], fullDelim)
	if idx < 0 {
		// im Screencast:
		//lx.readChar()
		//return token.Token{Type: token.INVALID, Value: string(delim)}
		// besser für Fehlersuche: bis zum Ende lesen
		errStr := lx.content[lx.pos : lx.pos+3]
		lx.skipTo(len(lx.content))
		return token.Token{Type: token.INVALID, Value: errStr + "...unterminated"}
	}
	val := lx.content[lx.pos : lx.pos+idx]
	lx.skipTo(lx.pos + idx)
	for i := 0; i < delimCnt; i++ {
		lx.readChar()
	}
	return token.Token{Type: token.STR, Value: val}
}

func (lx *Lexer) readHexNumber() token.Token {
	// skip #
	lx.readChar()
	rex := regexp.MustCompile(`^[0-9a-fA-F]+`)
	m := rex.FindStringIndex(lx.content[lx.pos:])
	if m == nil {
		ch := lx.ch
		lx.readChar()
		return token.Token{Type: token.INVALID, Value: string(ch)}
	}
	val := lx.content[lx.pos : lx.pos+m[1]]
	lx.skipTo(lx.pos + m[1])
	return token.Token{Type: token.HEX, Value: val}
}

// alternativ: Automat selbst geschrieben
func (lx *Lexer) readNumberDfa() token.Token {
	const ( // Zustände
		s_start = iota
		s_sign
		s_int
		s_dec_sep
		s_dec
	)
	state := s_start
	res := []rune{}
	for {
		switch state {
		case s_start:
			if isAsciiSign(lx.ch) {
				res = append(res, lx.ch)
				state = s_sign
			} else if isAsciiDigit(lx.ch) {
				res = append(res, lx.ch)
				state = s_int
			} else {
				ch := lx.ch
				lx.readChar()
				return token.Token{Type: token.INVALID, Value: string(ch)}
			}
		case s_sign:
			res = append(res, lx.ch)
			if isAsciiDigit(lx.ch) {
				state = s_int
			} else {
				ch := lx.ch
				lx.readChar()
				return token.Token{Type: token.INVALID, Value: string(ch)}
			}
		case s_int:
			if isAsciiDigit(lx.ch) {
				res = append(res, lx.ch)
			} else if lx.ch == '.' {
				res = append(res, lx.ch)
				state = s_dec_sep
			} else { // end of int number
				return token.Token{Type: token.INT, Value: string(res)}
			}
		case s_dec_sep:
			res = append(res, lx.ch)
			if isAsciiDigit(lx.ch) {
				state = s_dec
			} else {
				lx.readChar()
				return token.Token{Type: token.INVALID, Value: string(res)}
			}
		case s_dec:
			if isAsciiDigit(lx.ch) {
				res = append(res, lx.ch)
			} else {
				return token.Token{Type: token.FLOAT, Value: string(res)}
			}
		}
		lx.readChar()
	}
}
