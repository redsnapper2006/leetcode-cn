package main

import (
	"fmt"
	"sort"
)

func findLHS(nums []int) int {
	m := map[int]int{}
	for i := 0; i < len(nums); i++ {
		m[nums[i]]++
	}

	var keys []int
	for k := range m {
		keys = append(keys, k)
	}
	sort.Ints(keys)

	ret := 0
	for i := 0; i < len(keys); i++ {
		v := m[keys[i]]
		v2, ok := m[keys[i]+1]
		if !ok {
			continue
		}
		if v+v2 > ret {
			ret = v + v2
		}
	}

	return ret
}

func main() {
	fmt.Println("a")
}
