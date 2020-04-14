package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
	var v1 []int
	for l1 != nil {
		v1 = append(v1, l1.Val)
		l1 = l1.Next
	}
	var v2 []int
	for l2 != nil {
		v2 = append(v2, l2.Val)
		l2 = l2.Next
	}

	rl := len(v1)
	if rl < len(v2) {
		rl = len(v2)
	}
	r := make([]int, rl+1)
	isPlus := false
	for i := 0; i < len(v2) && i < len(v1); i++ {
		c1 := v1[len(v1)-1-i] + v2[len(v2)-1-i]
		if isPlus {
			c1++
		}
		if c1 >= 10 {
			isPlus = true
			c1 -= 10
		} else {
			isPlus = false
		}
		r[len(r)-1-i] = c1
	}
	start := len(v1)
	p := v2
	if start > len(v2) {
		start = len(v2)
		p = v1
	}
	for i := start; i < rl; i++ {
		c1 := p[len(p)-1-i]
		if isPlus {
			c1++
		}
		if c1 >= 10 {
			isPlus = true
			c1 -= 10
		} else {
			isPlus = false
		}
		r[len(r)-1-i] = c1
	}
	if isPlus {
		r[0] = 1
	}
	sIdx := 1
	if r[0] == 1 {
		sIdx = 0
	}
	var pp *ListNode
	for i := len(r) - 1; i >= sIdx; i-- {
		t := ListNode{Val: r[i], Next: pp}
		pp = &t
	}
	return pp
}

func main() {
	fmt.Println("a")
}
