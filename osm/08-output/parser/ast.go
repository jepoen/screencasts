package parser

import (
	"08-output/token"
	"errors"
	"fmt"
	"image/color"
	"strconv"
	"strings"
	"unicode/utf8"

	"golang.org/x/image/colornames"
)

// ...Node: Interface

type AstNode interface {
	fmt.Stringer
	astNode()
}

type SettingsNode interface {
	AstNode
	settingsNode()
}

type StepNode interface {
	AstNode
	stepNode()
}

type ValueNode interface {
	AstNode
	valueNode()
	StrVal() string
	IntVal() (int64, error)
	BoolVal() (bool, error)
	FloatVal() (float64, error)
	RgbVal() (color.RGBA, error)
}

type Ast struct {
	ConfigList []SettingsNode
	StepList   []StepNode
}

func (n *Ast) astNode() {}

func (n *Ast) String() string {
	return fmt.Sprintln(n.ConfigList, n.StepList)
}

func NewAst() *Ast {
	return &Ast{
		ConfigList: []SettingsNode{},
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

func (n *ConfigEntry) astNode()      {}
func (n *ConfigEntry) settingsNode() {}

func (n *ConfigEntry) String() string {
	sValues := []string{}
	for _, v := range n.Values {
		sValues = append(sValues, v.String())
	}
	return fmt.Sprintf("(config %s (%s))", n.Key, strings.Join(sValues, ", "))
}

// Style
type StyleEntry struct {
	Key     string
	Options []*StyleOption
}

func (n *StyleEntry) astNode()      {}
func (n *StyleEntry) settingsNode() {}

func (n *StyleEntry) String() string {
	return fmt.Sprintf("(style %s %v)", n.Key, n.Options)
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
}

func (n *NumValue) StrVal() string {
	return n.String()
}

func (n *NumValue) BoolVal() (bool, error) {
	switch n.NumType {
	case NUM_FLOAT:
		val, err := strconv.ParseFloat(n.Value, 64)
		if err == nil {
			return val != 0.0, nil
		}
	case NUM_HEX:
		val, err := strconv.ParseInt(n.Value, 16, 64)
		if err == nil {
			return val != 0, nil
		}
	case NUM_INT:
		val, err := strconv.ParseInt(n.Value, 10, 64)
		if err == nil {
			return val != 0, nil
		}
	}
	errMsg := fmt.Sprintf("cannot convert %s to bool", n.Value)
	return false, errors.New(errMsg)
}

func (n *NumValue) IntVal() (int64, error) {
	switch n.NumType {
	case NUM_FLOAT:
		val, err := strconv.ParseFloat(n.Value, 64)
		if err == nil {
			return int64(val), nil
		}
	case NUM_HEX:
		val, err := strconv.ParseInt(n.Value, 16, 64)
		if err == nil {
			return val, nil
		}
	case NUM_INT:
		val, err := strconv.ParseInt(n.Value, 10, 64)
		if err == nil {
			return val, nil
		}
	}
	errMsg := fmt.Sprintf("cannot convert %s to int", n.Value)
	return 0, errors.New(errMsg)
}

func (n *NumValue) FloatVal() (float64, error) {
	if n.NumType == NUM_FLOAT || n.NumType == NUM_INT {
		val, err := strconv.ParseFloat(n.Value, 64)
		if err == nil {
			return val, nil
		}
	}
	errMsg := fmt.Sprintf("cannot convert %s to float", n.Value)
	return 0.0, errors.New(errMsg)
}

func hexStr2rgb(s string) (color.RGBA, error) {
	if len(s) == 6 {
		val, err := strconv.ParseUint(s, 16, 32)
		if err == nil {
			b := val % 256
			val /= 256
			g := val % 256
			r := val / 256
			return color.RGBA{uint8(r), uint8(g), uint8(b), 0xff}, nil
		}
	}
	errMsg := fmt.Sprintf("cannot convert %s to RGB value", s)
	return color.RGBA{}, errors.New(errMsg)
}

func (n *NumValue) RgbVal() (color.RGBA, error) {
	if n.NumType == NUM_HEX {
		return hexStr2rgb(n.Value)
	}
	errMsg := fmt.Sprintf("cannot convert %s to RGB value", n.Value)
	return color.RGBA{}, errors.New(errMsg)
}

type MyRgb color.RGBA

func (c MyRgb) String() string {
	return fmt.Sprintf("%02x%02x%02x[%02x]", c.R, c.G, c.B, c.A)
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

func (n *StrValue) StrVal() string {
	return n.Value
}

func (n *StrValue) BoolVal() (bool, error) {
	boolStrings := map[string]bool{
		"true":  true,
		"false": false,
		"T":     true,
		"F":     false,
		"yes":   true,
		"no":    false,
	}
	if val, ok := boolStrings[n.Value]; ok {
		return val, nil
	}
	errMsg := fmt.Sprintf("cannot convert %s to bool", n.Value)
	return false, errors.New(errMsg)
}

func (n *StrValue) IntVal() (int64, error) {
	firstChar, pos := utf8.DecodeRuneInString(n.Value)
	var val int64
	var err error
	if firstChar == '#' {
		val, err = strconv.ParseInt(n.Value[pos:], 16, 64)
	} else {
		val, err = strconv.ParseInt(n.Value[pos:], 10, 64)
	}
	if err == nil {
		return val, nil
	}
	errMsg := fmt.Sprintf("cannot convert %s to int", n.Value)
	return 0, errors.New(errMsg)
}

func (n *StrValue) FloatVal() (float64, error) {
	val, err := strconv.ParseFloat(n.Value, 64)
	if err == nil {
		return val, nil
	}
	errMsg := fmt.Sprintf("cannot convert %s to float", n.Value)
	return 0.0, errors.New(errMsg)
}

func (n *StrValue) RgbVal() (color.RGBA, error) {
	firstChar, pos := utf8.DecodeRuneInString(n.Value)
	if firstChar == '#' {
		return hexStr2rgb(n.Value[pos:])
	} else {
		val, ok := colornames.Map[n.Value]
		if ok {
			return val, nil
		}
	}
	errMsg := fmt.Sprintf("cannot convert '%s' to RGB value", n.Value)
	return color.RGBA{}, errors.New(errMsg)
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
