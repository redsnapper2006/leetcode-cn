package main

import "fmt"

func minEatingSpeed(piles []int, h int) int {
	min, max := 1, 1<<31-1
	for min <= max {
		m := min + (max-min)/2
		cnt := 0
		for _, p := range piles {
			cnt += p / m
			if p%m > 0 {
				cnt++
			}
		}
		if cnt > h {
			min = m + 1
		} else {
			max = m - 1
		}
	}
	return min
}

func main() {
	fmt.Println()
}
