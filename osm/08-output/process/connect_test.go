package process

import (
	"08-output/data"
	"log"
	"slices"
	"testing"
)

func TestAppendWay(t *testing.T) {
	// append new way to sequence of connected ways
	// goal: target should not reversed
	testCases := []struct {
		inp0 data.IdList
		inp1 data.IdList
		exp  data.IdList
	}{
		{
			inp0: data.IdList{1, 2},
			inp1: data.IdList{2, 1},
			exp:  data.IdList{1, 2, 1},
		},
		{
			inp0: data.IdList{1, 2, 3},
			inp1: data.IdList{1, 5, 4, 3},
			exp:  data.IdList{1, 2, 3, 4, 5, 1},
		},
		{
			inp0: data.IdList{1, 2, 3},
			inp1: data.IdList{5, 4, 1},
			exp:  data.IdList{3, 2, 1, 4, 5},
		},
		{
			inp0: data.IdList{1, 2, 3},
			inp1: data.IdList{1, 4, 5},
			exp:  data.IdList{3, 2, 1, 4, 5},
		},
	}
	for _, c := range testCases {
		res := appendWay(c.inp0, c.inp1)
		if !slices.Equal(c.exp, res) {
			t.Errorf("expected %v got %v", c.exp, res)
		}
	}
}

func TestJoinWays(t *testing.T) {
	testCases := []struct {
		ways []data.IdList
		conn []int
		exp  data.IdList
	}{
		{
			ways: []data.IdList{{1, 2}, {2, 1}},
			conn: []int{0, 1},
			exp:  data.IdList{1, 2, 1},
		},
		{
			ways: []data.IdList{{1, 2}, {3, 4}, {2, 1}},
			conn: []int{2, 0},
			exp:  data.IdList{2, 1, 2},
		},
		{
			ways: []data.IdList{{1, 2}, {4, 3}, {3, 2}, {11, 12}},
			conn: []int{0, 2, 1},
			exp:  data.IdList{1, 2, 3, 4},
		},
	}
	for _, c := range testCases {
		res := joinWays(c.ways, c.conn)
		if !slices.Equal(c.exp, res) {
			t.Errorf("expected: %v\n got %v", c.exp, res)
		}
	}
}
func TestConnectWaysDirected(t *testing.T) {
	inp := []data.IdList{
		{1, 2, 3},
		{4, 7, 5, 1},
		{10, 11},
		{11, 10},
		{23, 23},
		{31, 32, 33},
		{33, 34, 35},
		{3, 4},
		{21, 22, 23}, // 23 has grade 3
		{23, 24, 25},
		{23, 26, 27},
	}
	expect := []data.IdList{
		{1, 2, 3, 4, 7, 5, 1},
		{10, 11, 10},
		{23, 23},
		{31, 32, 33, 34, 35},
		{21, 22, 23}, // 23 has grade 3
		{23, 24, 25},
		{23, 26, 27},
	}
	res := ConnectWays(inp)
	log.Printf("expect %v", expect)
	log.Printf("got    %v", res)
	if !eqNodeLists(expect, res, true) {
		t.Errorf("expected %v\ngot %v", expect, res)
	}
}

// added after screencast
func TestConnectWaysUndirected(t *testing.T) {
	inp := []data.IdList{
		{3, 2, 1},
		{4, 7, 5, 1},
		{10, 11},
		{11, 10},
		{23, 23},
		{31, 32, 33},
		{35, 34, 33},
		{3, 4},
	}
	expect := []data.IdList{
		{1, 2, 3, 4, 7, 5, 1},
		{10, 11, 10},
		{23, 23},
		{31, 32, 33, 34, 35},
	}
	res := ConnectWays(inp)
	log.Printf("Undir expect %v", expect)
	log.Printf("Undir got    %v", res)
	if !eqNodeLists(expect, res, false) {
		t.Errorf("expected %v\ngot %v", expect, res)
	}
}

func eqNodeLists(wL0, wL1 []data.IdList, directed bool) bool {
	if len(wL0) != len(wL1) {
		return false
	}
	eqLists := map[int]bool{}
	for i := 0; i < len(wL0); i++ {
		for k := 0; k < len(wL1); k++ {
			if eqLists[k] { // wL1[k] is eq to another way
				continue
			}
			if eqLi(wL0[i], wL1[k], directed) {
				eqLists[k] = true
			}
		}
	}
	for k := 0; k < len(wL1); k++ {
		if !eqLists[k] {
			return false
		}
	}
	return true
}

func eqLi(li0, li1 data.IdList, directed bool) bool {
	if len(li0) != len(li1) {
		return false
	}
	ll0 := data.IdList{}
	ll1 := data.IdList{}
	ll0 = append(ll0, li0...)
	ll1 = append(ll1, li1...)
	if ll0.IsClosed() {
		ll0 = ll0[1:]
		ll1 = ll1[1:]
		for i := 0; i < len(ll0) && ll0[0] != ll1[0]; i++ {
			ll0 = append(ll0[1:], ll0[0])
		}
		if ll0[0] != ll1[0] {
			return false
		}
	}
	if slices.Equal(ll0, ll1) {
		return true
	}
	if directed {
		return false
	}
	// fix for cyclic lists (after screencast)
	if li0.IsClosed() { // omit 1st point
		ll0 = ll0[1:]
		ll1 = ll1[1:]
	}
	slices.Reverse(ll0) // first point is last
	return slices.Equal(ll0, ll1)
}
