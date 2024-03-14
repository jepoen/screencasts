package data

import "06-simpleshapes/geo"

type IdList []int64

type MemberType int

const (
	MEMBER_NIL MemberType = iota
	MEMBER_NODE
	MEMBER_WAY
	MEMBER_REL
)

type RelMember struct {
	Ref  int64
	Type MemberType
	Role string
}

type RelMemberList []RelMember

type TagMap map[string]string

type OsmData struct {
	Nodes     map[int64]geo.Coord
	Ways      map[int64]IdList
	Relations map[int64]RelMemberList
	NodeTags  map[int64]TagMap
	WayTags   map[int64]TagMap
	RelTags   map[int64]TagMap
}

func NewOsmData() *OsmData {
	return &OsmData{
		Nodes:     map[int64]geo.Coord{},
		Ways:      map[int64]IdList{},
		Relations: map[int64]RelMemberList{},
		NodeTags:  map[int64]TagMap{},
		WayTags:   map[int64]TagMap{},
		RelTags:   map[int64]TagMap{},
	}
}
