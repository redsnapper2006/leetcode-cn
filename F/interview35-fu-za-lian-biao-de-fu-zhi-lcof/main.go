package main

import "fmt"

type Node struct {
	Val    int
	Next   *Node
	Random *Node
}

func copyRandomList(head *Node) *Node {
	M := map[*Node]*Node{}

	p := head
	NH := Node{Val: 0}
	np := &NH
	for p != nil {
		np.Next = &Node{Val: p.Val}
		M[p] = np.Next
		p = p.Next
		np = np.Next
	}

	p = head
	np = NH.Next
	for p != nil {
		if p.Random != nil {
			np.Random = M[p.Random]
		}
		p = p.Next
		np = np.Next
	}

	return NH.Next
}

func main() {
	fmt.Println("a")
}
