package main

import "fmt"

func subsets(nums []int) [][]int {
	ret := [][]int{{}}
	for _, n := range nums {
		size := len(ret)
		for i := 0; i < size; i++ {
			t := make([]int, len(ret[i]))
			copy(t, ret[i])
			t = append(t, n)
			ret = append(ret, t)
		}
	}
	return ret
}

func main() {
	fmt.Println()
}
