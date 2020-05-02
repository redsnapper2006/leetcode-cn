package main

import (
	"fmt"
	"sort"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

type TN struct {
	Node *TreeNode
	X    int
	Y    int
}

type TNArr []*TN

func (p TNArr) Len() int {
	return len(p)
}

func (p TNArr) Swap(i, j int) {
	p[i], p[j] = p[j], p[i]
}

func (p TNArr) Less(i, j int) bool {
	if p[i].X != p[j].X {
		return p[i].X < p[j].X
	}

	if p[i].Y != p[j].Y {
		return p[i].Y > p[j].Y
	}
	return p[i].Node.Val < p[j].Node.Val
}

func verticalTraversal(root *TreeNode) [][]int {
	if root == nil {
		return [][]int{}
	}

	var buf []*TN
	newRoot := TN{Node: root, X: 0, Y: 0}
	// buf[newRoot.X] = append(buf[newRoot.X], &newRoot)
	var recur func(tn *TN, B *[]*TN)
	recur = func(tn *TN, B *[]*TN) {
		if tn.Node == nil {
			return
		}
		(*B) = append(*B, tn)
		if tn.Node.Left != nil {
			recur(&TN{Node: tn.Node.Left, X: tn.X - 1, Y: tn.Y - 1}, B)
		}
		if tn.Node.Right != nil {
			recur(&TN{Node: tn.Node.Right, X: tn.X + 1, Y: tn.Y - 1}, B)
		}
	}
	recur(&newRoot, &buf)
	sort.Sort(TNArr(buf))
	var ret [][]int
	px := -10000
	for i := 0; i < len(buf); i++ {
		if buf[i].X != px {
			ret = append(ret, []int{buf[i].Node.Val})
			px = buf[i].X
		} else {
			ret[len(ret)-1] = append(ret[len(ret)-1], buf[i].Node.Val)
		}
	}

	return ret
}

func main() {
	fmt.Println("a")
}
