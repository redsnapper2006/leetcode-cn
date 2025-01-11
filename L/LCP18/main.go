package main

import (
	"fmt"
	"sort"
)

func breakfastNumber(staple []int, drinks []int, x int) int {
	buf := make([]int, x+1)
	for _, v := range staple {
		if v >= x {
			continue
		}
		buf[v]++
	}
	for i := 1; i < len(buf); i++ {
		buf[i] = buf[i] + buf[i-1]
	}
	sum := 0
	for _, v := range drinks {
		if v >= x {
			continue
		}
		sum += buf[x-v]
		sum %= 1000000007
	}

	return sum
}

func breakfastNumberV2(staple []int, drinks []int, x int) int {
	sort.Ints(staple)
	sort.Ints(drinks)

	sum := 0
	for _, v := range staple {
		if v >= x {
			break
		}
		remain := x - v
		s, e := 0, len(drinks)-1
		for s <= e {
			m := s + (e-s)/2
			if drinks[m] > remain {
				e = m - 1
			} else {
				s = m + 1
			}
		}
		sum += s
		sum %= 1000000007
	}

	return sum
}

func main() {
	fmt.Println("a")
}
