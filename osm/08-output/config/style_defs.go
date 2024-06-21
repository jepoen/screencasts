package config

type LineCapType int

const (
	CAP_ROUND LineCapType = iota
	CAP_BUTT
	CAP_SQUARE
)

func (s LineCapType) String() string {
	names := map[LineCapType]string{
		CAP_ROUND:  "capRound",
		CAP_BUTT:   "capButt",
		CAP_SQUARE: "capSquare",
	}
	if str, ok := names[s]; ok {
		return str
	} else {
		return "capUnkown"
	}
}

var String2Cap = map[string]LineCapType{
	"round":  CAP_ROUND,
	"butt":   CAP_BUTT,
	"square": CAP_SQUARE,
}

type LineJoinType int

const (
	JOIN_ROUND LineJoinType = iota
	JOIN_MITER
	JOIN_BEVEL
)

func (s LineJoinType) String() string {
	names := map[LineJoinType]string{
		JOIN_ROUND: "joinRound",
		JOIN_MITER: "joinMiter",
		JOIN_BEVEL: "joinBevel",
	}
	if str, ok := names[s]; ok {
		return str
	} else {
		return "joinUnkown"
	}
}

var String2Join = map[string]LineJoinType{
	"round": JOIN_ROUND,
	"miter": JOIN_MITER,
	"bevel": JOIN_BEVEL,
}
