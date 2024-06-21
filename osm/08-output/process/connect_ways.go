package process

import (
	"08-output/data"
	"log"
	"slices"
)

type _Endpoint struct {
	wId      int
	opposite int64 // opposite endpoint of way
	isFirst  bool
}

type _EndPointList []_Endpoint

func appendWay(wOld, wNew data.IdList) data.IdList {
	if wOld[len(wOld)-1] == wNew[0] { // same direction
		//log.Println("case 1")
		// pass
	} else if wOld[len(wOld)-1] == wNew[len(wNew)-1] { // new way has wrong dir.
		//log.Println("case 4")
		slices.Reverse(wNew)
	} else if wOld[0] == wNew[0] { // first way has wrong direction
		//log.Println("case 2")
		slices.Reverse(wOld)
	} else if wOld[0] == wNew[len(wNew)-1] { // both ways have wrong dir.
		//log.Println("case 3")
		slices.Reverse(wOld)
		slices.Reverse(wNew)
	} else {
		log.Fatalf("ways not connected: %v %v", wOld, wNew)
	}
	w := append(wOld, wNew[1:]...)
	return w
}

func joinWays(ways []data.IdList, conn []int) data.IdList {
	if len(conn) == 0 {
		log.Fatal("join of zero ways not possible")
		return data.IdList{}
	}
	if len(conn) == 1 {
		return ways[conn[0]]
	}
	res := slices.Clone(ways[conn[0]])
	for _, idx := range conn[1:] {
		res = appendWay(res, ways[idx])
	}
	return res
}

func ConnectWays(ways []data.IdList) []data.IdList {
	res := []data.IdList{}
	//log.Println("ConnectWays")
	// 1. Sammeln Knoten ord=2
	endPoints := map[int64]_EndPointList{}
	for wId, wayPoints := range ways {
		firstId := wayPoints[0]
		lastId := wayPoints[len(wayPoints)-1]
		if firstId != lastId {
			endPoints[firstId] = append(endPoints[firstId],
				_Endpoint{wId, lastId, true})
			endPoints[lastId] = append(endPoints[lastId],
				_Endpoint{wId, firstId, false})
		}
	}
	//showEndPoints(endPoints, "all")
	for k, v := range endPoints {
		if len(v) != 2 {
			delete(endPoints, k)
		}
	}
	//showEndPoints(endPoints, "grade 2")
	usedWays := map[int]bool{} // index: wayIndex
	// solange Knoten ord=2
	// - verbinde Wege
	for len(endPoints) > 0 {
		k := nextNode(endPoints)
		conn := getConnectedWays(endPoints, k)
		connWay := joinWays(ways, conn)
		markUsedWays(usedWays, conn)
		res = append(res, connWay)
	}
	// Sammlung aller verbundenen und nicht verbundenen Wege
	for wId, idList := range ways {
		if !usedWays[wId] {
			res = append(res, idList)
		}
	}
	return res
}

func markUsedWays(usedWays map[int]bool, conn []int) {
	for _, wId := range conn {
		usedWays[wId] = true
	}
}

func getConnectedWays(endPoints map[int64]_EndPointList, k int64) []int {
	// - wiederhole:
	//   - suche Endpunkt nach beiden Seiten
	//   - falls ord=2:
	//     - verbinde Wege
	//       joinWays(ways []data.IdList, conn []int)
	conn := []int{}
	eP := endPoints[k]
	if eP[0].isFirst { // omit reversing directed ways
		eP[1], eP[0] = eP[0], eP[1]
	} // right point ist start of way
	delete(endPoints, k)
	eP0 := eP[0]
	eP1 := eP[1]
	// go to left and reverse list after completion
	conn = append(conn, eP0.wId)
	for ePn, ok := getNextSegment(endPoints, eP0); ok; ePn, ok = getNextSegment(endPoints, eP0) {
		delete(endPoints, eP0.opposite)
		conn = append(conn, ePn.wId)
		eP0 = ePn
	}
	slices.Reverse(conn)
	if eP0.wId == eP1.wId { // closed Way
		return conn
	}
	conn = append(conn, eP1.wId)
	for ePn, ok := getNextSegment(endPoints, eP1); ok; ePn, ok = getNextSegment(endPoints, eP1) {
		delete(endPoints, eP1.opposite)
		conn = append(conn, ePn.wId)
		eP1 = ePn
	}
	return conn
}

func getNextSegment(endPoints map[int64]_EndPointList, eP0 _Endpoint) (_Endpoint, bool) {
	if ePair, ok := endPoints[eP0.opposite]; ok {
		if ePair[0].wId == eP0.wId {
			return ePair[1], true
		} else {
			return ePair[0], true
		}
	}
	return _Endpoint{}, false
}

func nextNode(endPoints map[int64]_EndPointList) int64 {
	for k := range endPoints {
		return k
	}
	log.Fatal("should not be reached")
	return -1
}

/*
func showEndPoints(endpoints map[int64]_EndPointList, msg string) {
	log.Println(msg)
	for k, v := range endpoints {
		log.Printf("Point %d, ways %v", k, v)
	}
}
*/
