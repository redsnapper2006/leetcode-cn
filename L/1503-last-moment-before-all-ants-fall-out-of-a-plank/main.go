package main

import "fmt"

func getLastMoment(n int, left []int, right []int) int {
	max := -1
	for _, v := range left {
		if v > max {
			max = v
		}
	}
	for _, v := range right {
		if n-v > max {
			max = n - v
		}
	}
	return max
}

func main() {
	fmt.Println()
}
