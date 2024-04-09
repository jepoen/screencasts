package data

import "06-simpleshapes/geo"

type IdList []int64

func (idL IdList) Len() int {
	return len(idL)
}

func (iL IdList) Less(i, k int) bool {
	return iL[i] < iL[k]
}

func (iL IdList) Swap(i, k int) {
	iL[i], iL[k] = iL[k], iL[i]
}

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
