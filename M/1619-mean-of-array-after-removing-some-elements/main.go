package main

import (
	"fmt"
	"sort"
)

func trimMean(arr []int) float64 {
	sort.Ints(arr)
	five := int(float64(len(arr)) * 0.05)
	e := len(arr) - five
	sum := 0
	for i := five; i < e; i++ {
		sum += arr[i]
	}
	return float64(sum) / float64(e-five)
}

func main() {
	fmt.Println("a")
}
