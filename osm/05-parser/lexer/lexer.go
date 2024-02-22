package lexer

import (
	"05-parser/token"
	"log"
	"os"
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
	if tok, ok := token.Digraph(lx.ch, lx.peekChar()); ok {
		lx.readChar()
		lx.readChar()
		return tok
	} else if tok, ok := token.Char(lx.ch); ok {
		lx.readChar()
		return tok
	}
	// Fortsetzung
	tok := token.Token{Type: token.INVALID, Value: string(lx.ch)}
	lx.readChar()
	return tok
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
