package main

import (
	"fmt"
)

func numFriendRequests(ages []int) int {
	buf := make([]int, 121)
	for i := 0; i < len(ages); i++ {
		buf[ages[i]]++
	}

	sums := make([]int, 121)
	sum := 0
	for i := 0; i < len(sums); i++ {
		sum += buf[i]
		sums[i] = sum
	}

	ret := 0
	for i := 0; i < len(ages); i++ {
		age := ages[i]
		min := (age >> 1) + 7
		max := age
		if min < max {
			ret += sums[max] - sums[min] - 1
		}
	}
	return ret
}

func main() {
	fmt.Println("a")
}
