package main

import (
	"fmt"
	"sort"
)

func maximumElementAfterDecrementingAndRearranging(arr []int) int {
	sort.Ints(arr)
	arr[0] = 1
	for i := 1; i < len(arr); i++ {
		d := arr[i] - arr[i-1]
		if d < 0 {
			d = -d
		}
		if d <= 1 {
			continue
		}
		arr[i] = arr[i-1] + 1
	}
	return arr[len(arr)-1]
}

func main() {
	fmt.Println(maximumElementAfterDecrementingAndRearranging([]int{2, 2, 1, 2, 1}))
}
