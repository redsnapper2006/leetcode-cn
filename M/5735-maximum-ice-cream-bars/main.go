package main

import (
	"fmt"
	"sort"
)

func maxIceCream(costs []int, coins int) int {
	sort.Ints(costs)
	cnt := 0
	for i := 0; i < len(costs) && coins > 0; i++ {
		coins -= costs[i]
		cnt++
	}
	if coins < 0 {
		cnt--
	}
	return cnt
}

func main() {
	fmt.Println(maxIceCream([]int{1, 2, 3}, 5))
}
