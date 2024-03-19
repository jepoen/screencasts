package parser

import (
	"06-simpleshapes/token"
	"fmt"
)

func (p *Parser) parseDraw() *DrawStep {
	p.nextToken() // skip "draw"
	p.match(token.LBRACKET)
	tok := p.match(token.ID)
	p.match(token.RBRACKET)
	p.match(token.LPAREN)
	res := &DrawStep{DrawOp: tok.Value}
L:
	for {
		switch p.curToken.Type {
		case token.STYLE:
			res.Style = p.parseStyle()
		case token.FILTER:
			res.Filter = p.parseFilter()
		case token.RPAREN:
			break L
		}
	}
	p.match(token.RPAREN)
	return res
}

// Parse Filter
func (p *Parser) parseFilter() TagFilterNode {
	p.nextToken() // skip "filter"
	p.match(token.LPAREN)
	res := p.parseFilterExpr()
	p.match(token.RPAREN)
	return res
}

func (p *Parser) parseFilterExpr() TagFilterNode {
	left := p.parseTerm()
L:
	for {
		var op BoolOperator
		switch p.curToken.Type {
		case token.AND:
			op = OP_AND
			p.nextToken()
		case token.OR:
			op = OP_OR
			p.nextToken()
		case token.XOR:
			op = OP_XOR
			p.nextToken()
		default:
			break L
		}
		right := p.parseTerm()
		left = &BoolOp{
			Op:    op,
			Left:  left,
			Right: right,
		}
	}
	return left
}

func (p *Parser) parseTerm() TagFilterNode {
	switch p.curToken.Type {
	case token.NOT:
		p.nextToken()
		left := p.parseTerm()
		return &BoolOp{
			Op:    OP_NOT,
			Left:  left,
			Right: nil,
		}
	case token.LPAREN:
		p.nextToken()
		left := p.parseFilterExpr()
		p.match(token.RPAREN)
		return left
	case token.ID, token.STR:
		return p.parseTagCmp()
	default:
		p.error(fmt.Sprintf("Invalid token, expected 'not', '(...)' or tag, got %s",
			p.curToken,
		))
		return nil
	}
}

func (p *Parser) parseTagCmp() TagFilterNode {
	cmpTokens := map[token.TokenType]CmpOperator{
		token.EQ:      OP_EQ,
		token.NE:      OP_NE,
		token.SIMILAR: OP_SIMILAR,
	}
	left := p.parseTagOrDefault()
	op := OP_EXISTS
	var right ValueNode = nil
	ok := false
	if op, ok = cmpTokens[p.curToken.Type]; ok {
		p.nextToken()
		right = p.parseValue()
	} // TODO "in ValueList"
	return &CmpOp{
		Op:    op,
		Left:  left,
		Right: right,
	}
}

func (p *Parser) parseTagOrDefault() *KeyOrDefault {
	// p.curToken is ID or String
	key := p.curToken.Value
	p.nextToken()
	var defaultVal ValueNode = nil
	if p.curToken.Type == token.DEFAULT {
		p.nextToken()
		defaultVal = p.parseValue()
	}
	return &KeyOrDefault{
		Key:     key,
		Default: defaultVal,
	}
}

// Parse Style
func (p *Parser) parseStyle() *DrawStyle {
	p.nextToken() // skip "style"
	baseStyle := "_"
	if p.curToken.Type == token.LBRACKET {
		p.nextToken()
		tok := p.match(token.ID)
		p.match(token.RBRACKET)
		baseStyle = tok.Value
	}
	return &DrawStyle{
		BaseStyle: baseStyle,
		Options:   p.parseStyleList(),
	}
}

func (p *Parser) parseStyleList() []*StyleOption {
	p.match(token.LPAREN)
	res := []*StyleOption{}
	for {
		id := ""
		switch p.curToken.Type {
		case token.ID, token.STR:
			id = p.curToken.Value
			p.nextToken()
			/*
				case token.RPAREN: // position option without value not allowed
					break L
			*/
		}
		// TODO: key without value?
		p.match(token.ASSIGN)
		switch p.curToken.Type {
		case token.LPAREN:
			values := p.parseValueTuple()
			res = append(res, &StyleOption{
				Key:   id,
				Value: &StyleValue{Values: values},
			})
		case token.INT, token.FLOAT, token.HEX:
			values := []ValueNode{NewNumValue(p.curToken)}
			res = append(res, &StyleOption{
				Key:   id,
				Value: &StyleValue{Values: values},
			})
			p.nextToken()
		case token.STR:
			values := []ValueNode{&StrValue{Value: p.curToken.Value}}
			res = append(res, &StyleOption{
				Key:   id,
				Value: &StyleValue{Values: values},
			})
			p.nextToken()
		case token.ID:
			values := &IdRef{Ref: p.curToken.Value}
			res = append(res, &StyleOption{
				Key:   id,
				Value: values,
			})
			p.nextToken()
		}
		if p.curToken.Type == token.RPAREN {
			break
		}
		p.match(token.COMMA)
	}
	p.match(token.RPAREN)
	return res
}
