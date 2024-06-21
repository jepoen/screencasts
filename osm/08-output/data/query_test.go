package data

import (
	"08-output/geo"
	"strings"
	"testing"
)

func TestQuery(t *testing.T) {
	xml := `<osm>
<node id="1" lat="50" lon="10"/>
<node id="2" lat="51" lon="11">
<tag k="n2" v="v2"/>
</node>
<node id="3" lat="52" lon="12">
<tag k="n31" v="v31"/>
<tag k="n32" v="v32"/>
</node>
<way id="10">
<nd ref="1"/>
<nd ref="2"/>
</way>
<way id="11">
<nd ref="2"/>
<nd ref="3"/>
<tag k="w" v="waytag"/>
</way>
<relation id="100">
<member ref="1" type="node" role="noderef"/>
<member ref="11" type="way" role="wayref"/>
<tag k="r" v="reltag"/>
</relation>
</osm>
`
	expected := OsmData{
		Nodes: map[int64]geo.Coord{
			1: {Lon: 10, Lat: 50},
			2: {Lon: 11, Lat: 51},
			3: {Lon: 12, Lat: 52},
		},
		NodeTags: map[int64]TagMap{
			2: {"n2": "v2"},
			3: {"n31": "v31", "n32": "v32"},
		},
		Ways: map[int64]IdList{
			10: {1, 2},
			11: {2, 3},
		},
		WayTags: map[int64]TagMap{
			11: {"w": "waytag"},
		},
		Relations: map[int64]RelMemberList{
			100: {
				RelMember{1, MEMBER_NODE, "noderef"},
				RelMember{11, MEMBER_WAY, "wayref"},
			},
		},
		RelTags: map[int64]TagMap{
			100: {"r": "reltag"},
		},
	}
	osm, err := parseXml(strings.NewReader(xml))
	if err != nil {
		t.Error(err)
	}
	// Debug-Ausgabe
	//fmt.Println(osm)
	if !osmEq(expected, *osm) {
		t.Errorf("\nexpected %v\ngot      %v", expected, osm)
	}
}

func osmEq(expected, got OsmData) bool {
	// Vergleich Nodes
	if len(expected.Nodes) != len(got.Nodes) {
		return false
	}
	for k, v := range expected.Nodes {
		if !eqCoord(got.Nodes[k], v) {
			return false
		}
	}
	// Vergleich NodeTags
	if len(expected.NodeTags) != len(got.NodeTags) {
		return false
	}
	for k, v := range expected.NodeTags {
		if !eqTagMap(got.NodeTags[k], v) {
			return false
		}
	}
	// Vergleich Ways
	if len(expected.Ways) != len(got.Ways) {
		return false
	}
	for k, v := range expected.Ways {
		if !eqIdList(got.Ways[k], v) {
			return false
		}
	}
	// Vergleich WayTags
	if len(expected.WayTags) != len(got.WayTags) {
		return false
	}
	for k, v := range expected.WayTags {
		if !eqTagMap(got.WayTags[k], v) {
			return false
		}
	}
	// Vergleich Relations
	if len(expected.Relations) != len(got.Relations) {
		return false
	}
	for k, v := range expected.Relations {
		if !eqMemberList(got.Relations[k], v) {
			return false
		}
	}
	// Vergleich RelTags
	if len(expected.RelTags) != len(got.RelTags) {
		return false
	}
	for k, v := range expected.RelTags {
		if !eqTagMap(got.RelTags[k], v) {
			return false
		}
	}
	return true
}

func eqCoord(c0, c1 geo.Coord) bool {
	return c0.Lon == c1.Lon && c0.Lat == c1.Lat
}

func eqTagMap(m0, m1 TagMap) bool {
	if len(m0) != len(m1) {
		return false
	}
	for k, v := range m0 {
		if v != m1[k] {
			return false
		}
	}
	return true
}

func eqIdList(l0, l1 IdList) bool {
	if len(l0) != len(l1) {
		return false
	}
	for i := range l0 {
		if l0[i] != l1[i] {
			return false
		}
	}
	return true
}

func eqMemberList(ml0, ml1 RelMemberList) bool {
	if len(ml0) != len(ml1) {
		return false
	}
	for i := range ml0 {
		if !eqMember(ml0[i], ml1[i]) {
			return false
		}
	}
	return true
}

func eqMember(m0, m1 RelMember) bool {
	return m0.Ref == m1.Ref && m0.Type == m1.Type && m0.Role == m1.Role
}
