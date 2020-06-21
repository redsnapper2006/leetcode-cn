package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func detectCycle(head *ListNode) *ListNode {
	if head == nil {
		return nil
	}

	pFast, pSlow := head, head
	for {
		if pFast.Next != nil && pFast.Next.Next != nil {
			pFast = pFast.Next.Next
		} else {
			return nil
		}
		if pSlow.Next != nil {
			pSlow = pSlow.Next
		}

		if pFast == pSlow {
			break
		}
	}
	pFast = head
	for pFast != pSlow {
		pFast = pFast.Next
		pSlow = pSlow.Next
	}

	return pFast
}

func main() {
	fmt.Println("a")
}
