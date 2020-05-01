package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func getDecimalValue(head *ListNode) int {
	accum := 0
	p := head
	for p != nil {
		accum = accum*2 + p.Val
		p = p.Next
	}
	return accum
}

func main() {
	fmt.Println("a")
}
