package main

import (
	"fmt"
)

func minEatingSpeed(piles []int, H int) int {
	s, e := 1, -1
	for i := 0; i < len(piles); i++ {
		if piles[i] > e {
			e = piles[i]
		}
	}
	if len(piles) == H {
		return e
	}

	for s < e {
		m := s + (e-s)/2
		hours := 0
		for i := 0; i < len(piles); i++ {
			hours += (piles[i] + m - 1) / m
		}
		if hours > H {
			s = m + 1
		} else {
			e = m
		}
	}
	return s
}

func main() {
	fmt.Println(minEatingSpeed([]int{332484035, 524908576, 855865114, 632922376, 222257295, 690155293, 112677673, 679580077, 337406589, 290818316, 877337160, 901728858, 679284947, 688210097, 692137887, 718203285, 629455728, 941802184}, 823855818))
}
