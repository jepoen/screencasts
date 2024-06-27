package config

import (
	"fmt"
	"image"
	"path/filepath"

	"github.com/jung-kurt/gofpdf"
	"github.com/llgcode/draw2d"
	"github.com/llgcode/draw2d/draw2dimg"
	"github.com/llgcode/draw2d/draw2dpdf"
	"github.com/llgcode/draw2d/draw2dsvg"
)

type Canvas interface {
	Ctx() draw2d.GraphicContext
	Save()
}

type Png struct {
	fileName string
	canvas   *image.RGBA
	ctx      *draw2dimg.GraphicContext
}

func NewPngCanvas(fileName string, width, height float64) Canvas {
	pngCanvas := &Png{
		fileName: fileName,
		canvas: image.NewRGBA(image.Rect(
			0, 0, int(width), int(height),
		)),
	}
	pngCanvas.ctx = draw2dimg.NewGraphicContext(pngCanvas.canvas)
	return pngCanvas
}

func (c *Png) Ctx() draw2d.GraphicContext {
	return c.ctx
}

func (c *Png) Save() {
	draw2dimg.SaveToPngFile(c.fileName, c.canvas)
}

type Pdf struct {
	fileName string
	pdf      *gofpdf.Fpdf
	ctx      *draw2dpdf.GraphicContext
}

func NewPdfCanvas(fileName string, width, height float64) Canvas {
	init := &gofpdf.InitType{
		OrientationStr: "P",
		UnitStr:        "mm",
		Size:           gofpdf.SizeType{Wd: width, Ht: height},
		FontDirStr:     draw2d.GetFontFolder(),
	}
	pdf := gofpdf.NewCustom(init)
	pdf.AddPage()
	return &Pdf{
		fileName: fileName,
		pdf:      pdf,
		ctx:      draw2dpdf.NewGraphicContext(pdf),
	}
}

func (c *Pdf) Ctx() draw2d.GraphicContext {
	return c.ctx
}

func (c *Pdf) Save() {
	draw2dpdf.SaveToPdfFile(c.fileName, c.pdf)
}

type Svg struct {
	fileName string
	canvas   *draw2dsvg.Svg
	ctx      *draw2dsvg.GraphicContext
}

func NewSvgCanvas(fileName string, width, height float64) Canvas {
	svg := draw2dsvg.NewSvg()
	svg.ViewBox = fmt.Sprintf("0 0 %d %d", int(width), int(height))
	return &Svg{
		fileName: fileName,
		canvas:   svg,
		ctx:      draw2dsvg.NewGraphicContext(svg),
	}
}

func (c *Svg) Ctx() draw2d.GraphicContext {
	return c.ctx
}

func (c *Svg) Save() {
	draw2dsvg.SaveToSvgFile(c.fileName, c.canvas)
}

func GetCanvas(fileName string, width, height float64) Canvas {
	ext := filepath.Ext(fileName)
	switch ext {
	case ".png":
		return NewPngCanvas(fileName, width, height)
	case ".pdf":
		return NewPdfCanvas(fileName, width, height)
	case ".svg":
		return NewSvgCanvas(fileName, width, height)
	}
	return nil
}
