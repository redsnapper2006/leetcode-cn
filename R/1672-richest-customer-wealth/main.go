package main

import "fmt"

func maximumWealth(accounts [][]int) int {
	max := -1
	for _, v := range accounts {
		sum := 0
		for _, a := range v {
			sum += a
		}
		if sum > max {
			max = sum
		}
	}

	return max
}

func main() {
	fmt.Println()
}
