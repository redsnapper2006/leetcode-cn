package main

import "fmt"

func findPairs(nums []int, k int) int {
	if k < 0 {
		return 0
	}
	M := map[int]int{}
	for _, n := range nums {
		M[n]++
	}

	c := 0
	if k == 0 {
		for _, v2 := range M {
			if v2 > 1 {
				c++
			}
		}
		return c
	}

	for v := range M {
		_, ok2 := M[v+k]
		if ok2 {
			c++
		}
	}
	return c
}

func main() {
	fmt.Println("a")
}
