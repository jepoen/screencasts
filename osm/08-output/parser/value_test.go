package parser

import (
	"errors"
	"image/color"
	"log"
	"testing"
)

func TestRgbVal(t *testing.T) {
	type _Result struct {
		val color.RGBA
		err error
	}
	testCases := []struct {
		inp ValueNode
		res _Result
	}{
		{
			&NumValue{NUM_HEX, "00aaff"},
			_Result{color.RGBA{0x0, 0xaa, 0xff, 0xff}, nil},
		},
		{
			&StrValue{"red"},
			_Result{color.RGBA{0xff, 0x0, 0x0, 0xff}, nil},
		},
		{
			&NumValue{NUM_INT, "255"},
			_Result{color.RGBA{}, errors.New("invalid")},
		},
		{
			&StrValue{"foobar"},
			_Result{color.RGBA{0xff, 0x0, 0x0, 0x0}, errors.New("invalid")},
		},
	}
	for _, c := range testCases {
		val, err := c.inp.RgbVal()
		if (err != nil) != (c.res.err != nil) {
			t.Errorf("expected %s, %s, got %s, %s",
				MyRgb(c.res.val), c.res.err, MyRgb(val), err,
			)
		}
		if err == nil && val != c.res.val {
			t.Errorf("expected %s, %s, got %s, %s",
				MyRgb(c.res.val), c.res.err, MyRgb(val), err,
			)
		}
	}
}
func TestBoolVal(t *testing.T) {
	type _Result struct {
		val bool
		err error
	}
	testCases := []struct {
		inp ValueNode
		res _Result
	}{
		{
			&NumValue{NUM_HEX, "00aaff"},
			_Result{true, nil},
		},
		{
			&StrValue{"true"},
			_Result{true, nil},
		},
		{
			&NumValue{NUM_INT, "0"},
			_Result{false, nil},
		},
		{
			&StrValue{"foobar"},
			_Result{false, errors.New("invalid")},
		},
	}
	for _, c := range testCases {
		val, err := c.inp.BoolVal()
		if (err != nil) != (c.res.err != nil) {
			t.Errorf("expected %t, %s, got %t, %s",
				c.res.val, c.res.err, val, err,
			)
		}
		if err == nil && val != c.res.val {
			t.Errorf("expected %t, %s, got %t, %s",
				c.res.val, c.res.err, val, err,
			)
		}
	}
}
func TestFloatVal(t *testing.T) {
	type _Result struct {
		val float64
		err error
	}
	testCases := []struct {
		inp ValueNode
		res _Result
	}{
		{
			&NumValue{NUM_HEX, "00aaff"},
			_Result{0.0, errors.New("invalid")},
		},
		{
			&StrValue{"3.5"},
			_Result{3.5, nil},
		},
		{
			&NumValue{NUM_INT, "0"},
			_Result{0.0, nil},
		},
		{
			&StrValue{"foobar"},
			_Result{0.0, errors.New("invalid")},
		},
	}
	for _, c := range testCases {
		val, err := c.inp.FloatVal()
		if (err != nil) != (c.res.err != nil) {
			t.Errorf("expected %f, %s, got %f, %s",
				c.res.val, c.res.err, val, err,
			)
		}
		if err == nil && val != c.res.val {
			t.Errorf("expected %f, %s, got %f, %s",
				c.res.val, c.res.err, val, err,
			)
		}
	}
}

func TestIntVal(t *testing.T) {
	type _Result struct {
		val int64
		err error
	}
	testCases := []struct {
		inp ValueNode
		res _Result
	}{
		{
			&NumValue{NUM_HEX, "00aaff"},
			_Result{0xaaff, nil},
		},
		{
			&NumValue{NUM_FLOAT, "3.5"},
			_Result{3, nil},
		},
		{
			&StrValue{"3.5"},
			_Result{0, errors.New("invalid")},
		},
		{
			&NumValue{NUM_INT, "0"},
			_Result{0, nil},
		},
		{
			&StrValue{"foobar"},
			_Result{0, errors.New("invalid")},
		},
	}
	for _, c := range testCases {
		val, err := c.inp.IntVal()
		if (err != nil) != (c.res.err != nil) {
			t.Errorf("expected %d, %s, got %d, %s",
				c.res.val, c.res.err, val, err,
			)
		}
		if err == nil && val != c.res.val {
			t.Errorf("expected %d, %s, got %d, %s",
				c.res.val, c.res.err, val, err,
			)
		}
	}
}

func TestCast(t *testing.T) {
	a := 3.5
	log.Printf("%d", int64(a))
}
