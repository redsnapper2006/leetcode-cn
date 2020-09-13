package main

import "fmt"

func findKthPositive(arr []int, k int) int {
	idx := -1
	accum := 0
	prev := 0
	for i, v := range arr {
		accum += v - prev - 1
		if accum >= k {
			idx = i
			break
		}
		prev = v
	}
	if idx == -1 {
		return arr[len(arr)-1] + k - accum
	}
	return arr[idx] - accum + k - 1
}

func main() {
	fmt.Println("a")
}
