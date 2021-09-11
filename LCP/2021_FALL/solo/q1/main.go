package main

import "fmt"

func minimumSwitchingTimes(source [][]int, target [][]int) int {
	m := map[int]int{}

	for _, r := range source {
		for _, v := range r {
			m[v]++
		}
	}
	for _, r := range target {
		for _, v := range r {
			if m[v] > 0 {
				m[v]--
			}
		}
	}
	ret := 0
	for _, v := range m {
		if v > 0 {
			ret += v
		}
	}
	return ret
}

func main() {
	fmt.Println()
}
