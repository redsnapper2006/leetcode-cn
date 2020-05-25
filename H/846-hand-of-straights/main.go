package main

import (
	"fmt"
	"sort"
)

func isNStraightHand(hand []int, W int) bool {
	if W == 1 {
		return true
	}

	sort.Ints(hand)
	buf := make(map[int]int)
	min, max := hand[0], hand[len(hand)-W+1]
	for i := 0; i < len(hand); i++ {
		buf[hand[i]]++
	}

	for i := min; i < max; {
		v, ok := buf[i]
		if !ok || v <= 0 {
			i++
			continue
		}

		min := v
		for j := 1; j < W; j++ {
			v, ok := buf[i+j]
			if !ok || v <= 0 {
				return false
			}
			if min >= v {
				min = v
			}
		}
		for j := 0; j < W; j++ {
			buf[i+j] -= min
		}
		i++
	}

	for _, v := range buf {
		if v > 0 {
			return false
		}
	}
	return true
}

func main() {
	fmt.Println(isNStraightHand([]int{1, 2, 3, 6, 2, 3, 4, 7, 8}, 3))
}
