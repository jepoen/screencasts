package parser

import (
	"06-simpleshapes/lexer"
	"06-simpleshapes/token"
	"fmt"
	"log"
)

type Parser struct {
	lx       *lexer.Lexer
	curToken token.Token
}

func (p *Parser) error(msg string) {
	log.Fatalf("Error in file %s line %d: %s",
		p.lx.FileName(), p.lx.TokenLine(), msg,
	)
}

func (p *Parser) nextToken() token.Token {
	p.curToken = p.lx.Next()
	return p.curToken
}

func (p *Parser) match(tokType token.TokenType) token.Token {
	if p.curToken.Type != tokType {
		p.error(fmt.Sprintf("expected %s, got %s",
			tokType, p.curToken.Value),
		)
	}
	res := p.curToken
	p.nextToken()
	return res
}

func NewParser(lx *lexer.Lexer) *Parser {
	p := &Parser{lx: lx}
	p.nextToken()
	return p
}

func NewParserFromFile(fileName string) *Parser {
	lx := lexer.NewLexerFromFile(fileName)
	return NewParser(lx)
}

func (p *Parser) ParseAll() *Ast {
	ast := NewAst()
	for !p.curToken.IsEof() {
		switch p.curToken.Type {
		case token.CONFIG:
			configNode := p.parseConfig()
			ast.ConfigList = append(ast.ConfigList, configNode)
		case token.INC:
			subAst := p.parseInclude()
			ast.addSubAst(subAst)
		case token.QUERY:
			stepNode := p.parseQuery()
			ast.StepList = append(ast.StepList, stepNode)
		case token.DRAW:
			stepNode := p.parseDraw()
			ast.StepList = append(ast.StepList, stepNode)
		default:
			p.error(fmt.Sprintf("Invalid token, expected entry, got %s",
				p.curToken.Value),
			)
		}
	}
	return ast
}

func (p *Parser) parseConfig() ConfigNode {
	// p.curToken.Type == token.CONFIG
	p.nextToken()
	p.match(token.LBRACKET)
	tok := p.match(token.ID)
	id := tok.Value
	p.match(token.RBRACKET)
	p.match(token.ASSIGN)
	var values []ValueNode
	if p.curToken.Type == token.LPAREN {
		values = p.parseValueTuple()
	} else {
		values = p.parseValueSequence()
	}
	return &ConfigEntry{
		Key:    id,
		Values: values,
	}
}

func (p *Parser) parseValueTuple() []ValueNode {
	res := []ValueNode{}
	p.match(token.LPAREN)
	for {
		nVal := p.parseValue()
		res = append(res, nVal)
		if p.curToken.Type == token.RPAREN {
			break
		}
		p.match(token.COMMA)
	}
	p.match(token.RPAREN)
	return res
}

func (p *Parser) parseValueSequence() []ValueNode {
	res := []ValueNode{}
	for p.curToken.IsValue() {
		res = append(res, p.parseValue())
	}
	return res
}

func (p *Parser) parseValue() ValueNode {
	var res ValueNode
	if p.curToken.IsValue() {
		if p.curToken.Type == token.STR {
			res = &StrValue{Value: p.curToken.Value}
		} else {
			res = NewNumValue(p.curToken)
		}
	} else {
		p.error(
			fmt.Sprintf("expected value, got %s", p.curToken.Value),
		)
	}
	p.nextToken()
	return res
}

func (p *Parser) parseInclude() *Ast {
	// skip include token
	p.match(token.INC)
	ast := NewAst()
	for p.curToken.Type == token.STR {
		pSub := NewParserFromFile(p.curToken.Value)
		subAst := pSub.ParseAll()
		ast.addSubAst(subAst)
		p.nextToken()
	}
	return ast
}

func (p *Parser) parseDraw() StepNode {
	panic("not implemented")

}

func (p *Parser) parseQuery() StepNode {
	// skip query keyword
	p.match(token.QUERY)
	p.match(token.LBRACKET)
	tok := p.match(token.ID)
	key := tok.Value
	var value *StrValue
	p.match(token.RBRACKET)
	if p.curToken.Type == token.ASSIGN {
		p.nextToken()
		valTok := p.match(token.STR)
		value = &StrValue{valTok.Value}
	}
	return &QueryStep{
		Key:   key,
		Query: value,
	}
}
