package config

import (
	"08-output/parser"
	"errors"
	"fmt"
	"image/color"
	"log"

	"github.com/llgcode/draw2d"
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
	FillColor   color.RGBA
	LineDash    []float64
	DashOffset  float64
	LineCap     LineCapType
	LineJoin    LineJoinType
	CloseWays   bool
	ConnectWays bool
}

func NewStyle(parent *Style) *Style {
	style := &Style{}
	if parent != nil {
		*style = *parent
	}
	style.functions = map[string]styleFunc{}
	style.LineDash = []float64{}
	// new Attribute: register
	style.registerStyle("lineWidth", style.evalLineWidth, style.applyLineWidth)
	style.registerStyle("strokeColor", style.evalStrokeColor, style.applyStrokeColor)
	// added after video
	style.registerStyle("fillColor", style.evalFillColor, style.applyFillColor)
	style.registerStyle("lineDash", style.evalLineDash, style.applyLineDash)
	style.registerStyle("dashOffset", style.evalDashOffset, nil)
	style.registerStyle("lineCap", style.evalLineCap, style.applyLineCap)
	style.registerStyle("lineJoin", style.evalLineJoin, style.applyLineJoin)
	style.registerStyle("closeWays", style.evalCloseWays, nil)
	style.registerStyle("connectWays", style.evalConnectWays, nil)
	return style
}

func (s *Style) registerStyle(key string, es evalFunc, as applyFunc) {
	s.functions[key] = styleFunc{
		apply: as,
		eval:  es,
	}
}

func (s *Style) String() string {
	return fmt.Sprintf("lw %f stroke %v fill %v dash %v/%v",
		s.LineWidth, s.StrokeColor, s.FillColor,
		s.LineDash, s.DashOffset,
	)
}

func CreateBaseStyle() *Style {
	style := NewStyle(nil)
	style.LineWidth = 0.0
	style.StrokeColor = color.RGBA{}
	return style
}

func EvalStyle(nodes []*parser.DrawStyle, env *Environment) []*Style {
	sParent := "_"
	res := []*Style{}
	for _, node := range nodes {
		if node != nil {
			sParent = node.BaseStyle
		}
		parent := env.Styles[sParent]
		style := NewStyle(parent)
		if node != nil {
			for _, opt := range node.Options {
				if fu, ok := style.functions[opt.Key]; ok {
					fu.eval(opt)
				} else {
					log.Printf("Style option %s not implemented", opt)
				}
			}
		}
		res = append(res, style)
	}
	return res
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
	env.Ctx.SetLineWidth(s.LineWidth * env.Config.PtPerMm)
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

func (s *Style) evalConnectWays(opt *parser.StyleOption) {
	valNode, err := getSingleValue(opt.Value)
	var val bool
	if err == nil {
		if val, err = valNode.BoolVal(); err == nil {
			s.ConnectWays = val
		} else {
			log.Fatalf("connectWays: expected bool value, got %s",
				valNode,
			)
		}
	} else {
		log.Fatalf("connectWays: %s", err)
	}
}

func (s *Style) evalDashOffset(opt *parser.StyleOption) {
	valNode, err := getSingleValue(opt.Value)
	var val float64
	if err == nil {
		if val, err = valNode.FloatVal(); err == nil {
			s.DashOffset = val
		} else {
			log.Fatalf("dashOffset: expected float value, got %s",
				valNode,
			)
		}
	} else {
		log.Fatalf("dashOffset: %s", err)
	}
}

func getValueList(v parser.StyleExprNode) ([]parser.ValueNode, error) {
	errMsg := ""
	if val, ok := v.(*parser.StyleValue); ok {
		return val.Values, nil
	} else {
		errMsg = fmt.Sprintf("expected value list, go id %s", v)
	}
	return nil, errors.New(errMsg)
}

func (s *Style) evalLineDash(opt *parser.StyleOption) {
	valNodes, err := getValueList(opt.Value)
	if err != nil {
		log.Fatalf("lineDash: %s", err)
	}
	values := []float64{}
	for i, n := range valNodes {
		if v, err := n.FloatVal(); err == nil {
			values = append(values, v)
		} else {
			log.Fatalf("lineDash: value %d is not float, got %s",
				i+1, n,
			)
		}
	}
	s.LineDash = values
}

func (s *Style) applyLineDash(env *Environment) {
	dashes := []float64{}
	for _, v := range s.LineDash {
		dashes = append(dashes, v*env.Config.PtPerMm)
	}
	env.Ctx.SetLineDash(dashes, s.DashOffset*env.Config.PtPerMm)
}

func (s *Style) evalLineCap(opt *parser.StyleOption) {
	valNode, err := getSingleValue(opt.Value)
	if err == nil {
		capStr := valNode.StrVal()
		cap, ok := String2Cap[capStr]
		if !ok {
			log.Fatalf("LineCap type %s unknown", capStr)
		}
		s.LineCap = cap
	}
}

func (s *Style) applyLineCap(env *Environment) {
	caps := map[LineCapType]draw2d.LineCap{
		CAP_ROUND:  draw2d.RoundCap,
		CAP_BUTT:   draw2d.ButtCap,
		CAP_SQUARE: draw2d.SquareCap,
	}
	if c, ok := caps[s.LineCap]; ok {
		env.Ctx.SetLineCap(c)
	} else {
		log.Fatalf("applyLineCap: cap type %s not available", s.LineCap)
	}
}

func (s *Style) evalLineJoin(opt *parser.StyleOption) {
	valNode, err := getSingleValue(opt.Value)
	if err == nil {
		joinStr := valNode.StrVal()
		join, ok := String2Join[joinStr]
		if !ok {
			log.Fatalf("LineJoin type %s unknown", joinStr)
		}
		s.LineJoin = join
	}
}

func (s *Style) applyLineJoin(env *Environment) {
	joins := map[LineJoinType]draw2d.LineJoin{
		JOIN_ROUND: draw2d.RoundJoin,
		JOIN_MITER: draw2d.MiterJoin,
		JOIN_BEVEL: draw2d.BevelJoin,
	}
	if j, ok := joins[s.LineJoin]; ok {
		env.Ctx.SetLineJoin(j)
	} else {
		log.Fatalf("applyLineCap: cap type %s not available", s.LineCap)
	}
}
