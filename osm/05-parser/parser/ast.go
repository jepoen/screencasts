package parser

import (
	"05-parser/token"
	"fmt"
	"strings"
)

// ...Node: Interface

type AstNode interface {
	fmt.Stringer
	astNode()
}

type ConfigNode interface {
	AstNode
	configNode()
}

type StepNode interface {
	AstNode
	stepNode()
}

type ValueNode interface {
	AstNode
	valueNode()
}

type Ast struct {
	ConfigList []ConfigNode
	StepList   []StepNode
}

func (n *Ast) astNode() {}

func (n *Ast) String() string {
	return fmt.Sprintln(n.ConfigList, n.StepList)
}

func NewAst() *Ast {
	return &Ast{
		ConfigList: []ConfigNode{},
		StepList:   []StepNode{},
	}
}

func (n *Ast) addSubAst(sub *Ast) {
	n.ConfigList = append(n.ConfigList, sub.ConfigList...)
	n.StepList = append(n.StepList, sub.StepList...)
}

type ConfigEntry struct {
	Key    string
	Values []ValueNode
}

func (n *ConfigEntry) astNode()    {}
func (n *ConfigEntry) configNode() {}

func (n *ConfigEntry) String() string {
	sValues := []string{}
	for _, v := range n.Values {
		sValues = append(sValues, v.String())
	}
	return fmt.Sprintf("(config %s (%s))", n.Key, strings.Join(sValues, ", "))
}

type NumType int

const (
	NUM_INVALID NumType = iota
	NUM_INT
	NUM_FLOAT
	NUM_HEX
)

var numStr = map[NumType]string{
	NUM_INVALID: "invalid number",
	NUM_INT:     "int",
	NUM_FLOAT:   "float",
	NUM_HEX:     "hex",
}

func (nt NumType) String() string {
	return numStr[nt]
}

type NumValue struct {
	NumType NumType
	Value   string
}

func (n *NumValue) astNode()   {}
func (n *NumValue) valueNode() {}

// Nach Screencast ergänzt:
func (n *NumValue) String() string {
	switch n.NumType {
	case NUM_INT, NUM_FLOAT:
		return n.Value
	case NUM_HEX:
		return "#" + n.Value
	default:
		return "?" + n.Value
	}
	return n.Value
}

func NewNumValue(tok token.Token) *NumValue {
	ty := NUM_INVALID
	switch tok.Type {
	case token.INT:
		ty = NUM_INT
	case token.FLOAT:
		ty = NUM_FLOAT
	case token.HEX:
		ty = NUM_HEX
	}
	return &NumValue{NumType: ty, Value: tok.Value}
}

type StrValue struct {
	Value string
}

func (n *StrValue) astNode()   {}
func (n *StrValue) valueNode() {}

func (n *StrValue) String() string {
	return fmt.Sprintf("\"%s\"", n.Value)
}

type QueryStep struct {
	Key   string
	Query *StrValue
}

func (n *QueryStep) astNode()  {}
func (n *QueryStep) stepNode() {}

func (n *QueryStep) String() string {
	return fmt.Sprintf("(query %s %s)", n.Key, n.Query)
}
