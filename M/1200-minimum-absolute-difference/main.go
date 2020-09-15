package main

import (
	"fmt"
	"sort"
)

func minimumAbsDifference(arr []int) [][]int {
	sort.Ints(arr)
	min := 1<<31 - 1
	for i := 0; i < len(arr)-1; i++ {
		sub := arr[i+1] - arr[i]
		if min > sub {
			min = sub
		}
	}

	var ret [][]int
	for i := 0; i < len(arr)-1; i++ {
		sub := arr[i+1] - arr[i]
		if min == sub {
			ret = append(ret, []int{arr[i], arr[i+1]})
		}
	}
	return ret
}

func main() {
	fmt.Println("a")
}
