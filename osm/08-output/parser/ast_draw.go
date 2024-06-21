package parser

import (
	"fmt"
	"strings"
)

type BoolOperator int

const (
	OP_NOOP BoolOperator = iota
	OP_AND
	OP_OR
	OP_XOR
	OP_NOT
)

func (op BoolOperator) String() string {
	boolOpStr := map[BoolOperator]string{
		OP_AND: "and",
		OP_OR:  "or",
		OP_XOR: "xor",
		OP_NOT: "not",
	}
	return boolOpStr[op]
}

type CmpOperator int

const (
	OP_EXISTS CmpOperator = iota
	OP_EQ
	OP_NE
	OP_SIMILAR
)

func (op CmpOperator) String() string {
	cmpOpStr := map[CmpOperator]string{
		OP_EXISTS:  "exists",
		OP_EQ:      "==",
		OP_NE:      "!=",
		OP_SIMILAR: "~",
	}
	return cmpOpStr[op]
}

type DrawStep struct {
	DrawOp string
	Style  *DrawStyle
	Filter TagFilterNode
}

func (n *DrawStep) astNode()  {}
func (n *DrawStep) stepNode() {}

func (n *DrawStep) String() string {
	sFilter := "<nil>"
	if n.Filter != nil {
		sFilter = n.Filter.String()
	}
	return fmt.Sprintf("(draw %s %s %s)", n.DrawOp, n.Style, sFilter)
}

// implemented by BoolOp, CmpOp
type TagFilterNode interface {
	AstNode
	tagFilterNode()
}

type BoolOp struct {
	Op    BoolOperator
	Left  TagFilterNode
	Right TagFilterNode
}

func (n *BoolOp) astNode()       {}
func (n *BoolOp) tagFilterNode() {}

func (n *BoolOp) String() string {
	if n.Right != nil {
		return fmt.Sprintf("(%s %s %s)", n.Op, n.Left, n.Right)
	} else {
		return fmt.Sprintf("(%s %s)", n.Op, n.Left)
	}
}

// evaluates to string value
type TagOrDefault struct {
	Tag     string
	Default ValueNode // or nil
}

func (n *TagOrDefault) astNode() {}
func (n *TagOrDefault) String() string {
	if n.Default == nil {
		return n.Tag
	} else {
		return fmt.Sprintf("(keyOrDefault %s %s)", n.Tag, n.Default)
	}
}

// evaluates to bool
type CmpOp struct {
	Op    CmpOperator
	Left  *TagOrDefault
	Right ValueNode
}

func (n *CmpOp) astNode()       {}
func (n *CmpOp) tagFilterNode() {}
func (n *CmpOp) String() string {
	if n.Op == OP_EXISTS {
		return fmt.Sprintf("(%s %s)", n.Op, n.Left)
	} else {
		return fmt.Sprintf("(%s %s %s)", n.Op, n.Left, n.Right)
	}
}

// Style part
type DrawStyle struct {
	BaseStyle string // never empty
	Options   []*StyleOption
}

func (n *DrawStyle) astNode() {}
func (n *DrawStyle) String() string {
	options := []string{}
	for _, v := range n.Options {
		options = append(options, v.String())
	}
	return fmt.Sprintf("(style %s (%s))", n.BaseStyle, strings.Join(options, ", "))
}

type StyleOption struct {
	Key   string
	Value StyleExprNode // IdRef or []StyleValue
}

func (n *StyleOption) astNode() {}
func (n *StyleOption) String() string {
	return fmt.Sprintf("(option %s %s)", n.Key, n.Value)
}

// used for Value(List) or IdRef
type StyleExprNode interface {
	AstNode
	styleExprNode()
}

type StyleValue struct {
	Values []ValueNode
}

func (n *StyleValue) astNode()       {}
func (n *StyleValue) styleExprNode() {}

func (n *StyleValue) String() string {
	if len(n.Values) == 1 {
		return n.Values[0].String()
	}
	values := []string{}
	for _, v := range n.Values {
		values = append(values, v.String())
	}
	return fmt.Sprintf("(%s)", strings.Join(values, ", "))
}

type IdRef struct {
	Ref string
}

func (n *IdRef) astNode()       {}
func (n *IdRef) styleExprNode() {}
func (n *IdRef) String() string {
	return n.Ref
}
