package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func recoverFromPreorder(S string) *TreeNode {
	sb := []byte(S)
	num := 0
	idx := -1
	for i := 0; i < len(sb); i++ {
		if sb[i] == '-' {
			idx = i
			break
		}
		num = num*10 + int(sb[i]-'0')
	}

	NH := TreeNode{Val: num}
	if idx == -1 {
		return &NH
	}
	sb = sb[idx:]
	var recur func(sb *[]byte, depth int, p *TreeNode)
	recur = func(sb *[]byte, depth int, p *TreeNode) {

		if len(*sb) == 0 {
			return
		}
		idx := -1
		for i := 0; i < len(*sb); i++ {
			if (*sb)[i] != '-' {
				idx = i
				break
			}
		}
		if idx != depth {
			return
		}
		*sb = (*sb)[idx:]
		num := 0
		idx = -1
		for i := 0; i < len(*sb); i++ {
			if (*sb)[i] == '-' {
				idx = i
				break
			}
			num = num*10 + int((*sb)[i]-'0')
		}
		p.Left = &TreeNode{Val: num}
		if idx == -1 {
			return
		}
		*sb = (*sb)[idx:]
		recur(sb, depth+1, p.Left)
		idx = -1
		for i := 0; i < len(*sb); i++ {
			if (*sb)[i] != '-' {
				idx = i
				break
			}
		}
		if idx != depth {
			return
		}
		*sb = (*sb)[idx:]
		num = 0
		idx = -1
		for i := 0; i < len(*sb); i++ {
			if (*sb)[i] == '-' {
				idx = i
				break
			}
			num = num*10 + int((*sb)[i]-'0')
		}
		p.Right = &TreeNode{Val: num}
		if idx == -1 {
			return
		}
		*sb = (*sb)[idx:]
		recur(sb, depth+1, p.Right)
	}
	recur(&sb, 1, &NH)
	return &NH
}

func main() {
	fmt.Println(recoverFromPreorder("1-2--3--4-5--6--7"))
}
