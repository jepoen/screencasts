package config

import (
	"06-simpleshapes/parser"
	"errors"
	"fmt"
	"image/color"
	"log"
)

type applyFunc func(env *Environment)
type evalFunc func(opt *parser.StyleOption)

type styleFunc struct {
	apply applyFunc
	eval  evalFunc
}

// new Attribute: add field
type Style struct {
	functions   map[string]styleFunc
	LineWidth   float64
	StrokeColor color.RGBA
	// added after video
	FillColor color.RGBA
	CloseWays bool
}

func NewStyle(parent *Style) *Style {
	style := &Style{
		functions: map[string]styleFunc{},
	}
	if parent != nil {
		*style = *parent
	}
	// new Attribute: register
	style.registerStyle("lineWidth", style.evalLineWidth, style.applyLineWidth)
	style.registerStyle("strokeColor", style.evalStrokeColor, style.applyStrokeColor)
	// added after video
	style.registerStyle("fillColor", style.evalFillColor, style.applyFillColor)
	style.registerStyle("closeWays", style.evalCloseWays, nil)
	return style
}

func (s *Style) registerStyle(key string, es evalFunc, as applyFunc) {
	s.functions[key] = styleFunc{
		apply: as,
		eval:  es,
	}
}

func (s *Style) String() string {
	return fmt.Sprintf("lw %f stroke %v",
		s.LineWidth, s.StrokeColor,
	)
}

func CreateBaseStyle() *Style {
	style := NewStyle(nil)
	style.LineWidth = 0.0
	style.StrokeColor = color.RGBA{}
	return style
}

func EvalStyle(node *parser.DrawStyle, env *Environment) *Style {
	sParent := "_"
	if node != nil {
		sParent = node.BaseStyle
	}
	parent := env.Styles[sParent]
	style := NewStyle(parent)
	if node == nil {
		return style
	}
	for _, opt := range node.Options {
		if fu, ok := style.functions[opt.Key]; ok {
			fu.eval(opt)
		} else {
			log.Printf("Style option %s not implemented", opt)
		}
	}
	return style
}

func (s *Style) SetContext(env *Environment) {
	for _, fu := range s.functions {
		if fu.apply != nil {
			fu.apply(env)
		}
	}
}

func getSingleValue(v parser.StyleExprNode) (parser.ValueNode, error) {
	errMsg := ""
	if val, ok := v.(*parser.StyleValue); ok {
		if len(val.Values) == 1 {
			return val.Values[0], nil
		} else {
			errMsg = fmt.Sprintf("expected exactly one value, got %s",
				v,
			)
		}
	} else {
		errMsg = fmt.Sprintf("expected value, go id %s", v)
	}
	return nil, errors.New(errMsg)
}

func (s *Style) evalLineWidth(opt *parser.StyleOption) {
	valNode, err := getSingleValue(opt.Value)
	var val float64
	if err == nil {
		if val, err = valNode.FloatVal(); err == nil {
			s.LineWidth = val
		} else {
			log.Fatalf("lineWidth: expected float value, got %s",
				valNode,
			)
		}
	} else {
		log.Fatalf("lineWidth: %s", err)
	}
}

func (s *Style) applyLineWidth(env *Environment) {
	env.Ctx.SetLineWidth(s.LineWidth)
}

func (s *Style) evalStrokeColor(opt *parser.StyleOption) {
	valNode, err := getSingleValue(opt.Value)
	var val color.RGBA
	if err == nil {
		if val, err = valNode.RgbVal(); err == nil {
			s.StrokeColor = val
		} else {
			log.Fatalf("strokeColor: expected RGB value, got %s",
				valNode,
			)
		}
	} else {
		log.Fatalf("strokeColor: %s", err)
	}
}

func (s *Style) applyStrokeColor(env *Environment) {
	env.Ctx.SetStrokeColor(s.StrokeColor)
}

// added after video
func (s *Style) evalFillColor(opt *parser.StyleOption) {
	valNode, err := getSingleValue(opt.Value)
	var val color.RGBA
	if err == nil {
		if val, err = valNode.RgbVal(); err == nil {
			s.FillColor = val
		} else {
			log.Fatalf("fillColor: expected RGB value, got %s",
				valNode,
			)
		}
	} else {
		log.Fatalf("fillColor: %s", err)
	}
}

func (s *Style) applyFillColor(env *Environment) {
	env.Ctx.SetFillColor(s.FillColor)
}

func (s *Style) evalCloseWays(opt *parser.StyleOption) {
	valNode, err := getSingleValue(opt.Value)
	var val bool
	if err == nil {
		if val, err = valNode.BoolVal(); err == nil {
			s.CloseWays = val
		} else {
			log.Fatalf("closeWay: expected bool value, got %s",
				valNode,
			)
		}
	} else {
		log.Fatalf("closeWay: %s", err)
	}
}

// no applyCloseWay()
