package process

import (
	"06-simpleshapes/data"
	"06-simpleshapes/parser"
	"log"
)

func processFilter(
	tagFilter parser.TagFilterNode,
	tagMap map[int64]data.TagMap,
) data.IdList {
	res := data.IdList{}
	for id := range tagMap {
		if applyBoolFilter(tagFilter, tagMap[id]) {
			res = append(res, id)
		}
	}
	return res
}

func applyBoolFilter(tagFilter parser.TagFilterNode, tagMap data.TagMap) bool {
	switch fi := tagFilter.(type) {
	case *parser.CmpOp:
		val, ok := applyKeyOrDefault(fi.Left, tagMap)
		switch fi.Op {
		case parser.OP_EXISTS:
			return ok
		case parser.OP_EQ:
			if !ok {
				return false
			}
			rightVal := fi.Right.StrVal()
			return val == rightVal
		case parser.OP_NE:
			if !ok {
				return true
			}
			rightVal := fi.Right.StrVal()
			return val != rightVal
		case parser.OP_SIMILAR:
			log.Fatal("~ not implemented")
			return false
		default:
			log.Fatalf("Operation %s is not a compare op", fi.Op)
			return false
		}
	case *parser.BoolOp:
		left := applyBoolFilter(fi.Left, tagMap)
		switch fi.Op {
		case parser.OP_NOT:
			return !left
		case parser.OP_AND:
			if !left {
				return false
			}
			return applyBoolFilter(fi.Right, tagMap)
		case parser.OP_OR:
			if left {
				return true
			}
			return applyBoolFilter(fi.Right, tagMap)
		case parser.OP_XOR:
			right := applyBoolFilter(fi.Right, tagMap)
			return left != right
		default:
			log.Fatalf("Operation %s is not a bool op", fi.Op)
			return false
		}
	}
	log.Fatalf("Filter %s not implemented", tagFilter)
	return false
}

func applyKeyOrDefault(
	n *parser.TagOrDefault,
	tagMap data.TagMap,
) (string, bool) {
	if val, ok := tagMap[n.Tag]; ok {
		return val, ok
	}
	if n.Default != nil {
		return n.Default.StrVal(), true
	} else {
		return "", false
	}
}
