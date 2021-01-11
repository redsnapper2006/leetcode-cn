package main

import "fmt"

func longestSubsequence(arr []int, difference int) int {
	M := map[int]int{}
	for _, b := range arr {
		_, ok := M[b-difference]
		if !ok {
			M[b] = 1
		} else {
			M[b] = M[b-difference] + 1
		}
	}

	max := -1
	for _, v := range M {
		if v > max {
			max = v
		}
	}
	return max
}

func main() {
	fmt.Println()
}
