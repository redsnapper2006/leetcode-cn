package main

import (
	"fmt"
)

func PredictTheWinner(nums []int) bool {

	var recur func(ns []int, A, B int, AorB bool) bool
	recur = func(ns []int, A, B int, AorB bool) bool {
		if len(ns) == 0 {
			return A >= B
		}
		head, tail := true, true
		if AorB {
			head = recur(ns[1:], A+ns[0], B, !AorB)
			tail = recur(ns[0:len(ns)-1], A+ns[len(ns)-1], B, !AorB)
			return head || tail
		} else {
			head = recur(ns[1:], A, B+ns[0], !AorB)
			tail = recur(ns[0:len(ns)-1], A, B+ns[len(ns)-1], !AorB)
			return head && tail
		}
	}
	return recur(nums, 0, 0, true)
}

func main() {
	fmt.Println(PredictTheWinner([]int{1, 5, 2}))
}
