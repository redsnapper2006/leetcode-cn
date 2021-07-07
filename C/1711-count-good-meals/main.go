package main

import "fmt"

func countPairs(deliciousness []int) int {
	twoexp := []int{1, 1 << 1, 1 << 2, 1 << 3, 1 << 4, 1 << 5, 1 << 6, 1 << 7,
		1 << 8, 1 << 9, 1 << 10, 1 << 11, 1 << 12, 1 << 13, 1 << 14, 1 << 15,
		1 << 16, 1 << 17, 1 << 18, 1 << 19, 1 << 20, 1 << 21}

	cnt := map[int]int{}
	for _, v := range deliciousness {
		cnt[v]++
	}

	ret := 0
	for k, v := range cnt {
		for _, exp := range twoexp {
			if exp < k {
				continue
			}
			odd := exp - k
			if odd < k {
				continue
			}
			if odd != k {
				ret += v * cnt[odd]
			} else {
				ret += v * (v - 1) / 2
			}
			ret %= 1000000007
		}
	}

	return ret
}

func main() {
	fmt.Println(countPairs([]int{1, 3, 5, 7, 9}))
	fmt.Println(countPairs([]int{1, 1, 1, 3, 3, 3, 7}))
}
