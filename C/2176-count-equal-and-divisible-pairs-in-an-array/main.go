package main

import "fmt"

func countPairs(nums []int, k int) int {
	m := map[int][]int{}
	for i, v := range nums {
		m[v] = append(m[v], i)
	}
	ret := 0
	for _, idx := range m {
		for i := 0; i < len(idx); i++ {
			for j := i + 1; j < len(idx); j++ {
				if (idx[i]*idx[j])%k == 0 {
					ret++
				}
			}
		}
	}
	return ret
}

func main() {
	fmt.Println(countPairs([]int{3, 1, 2, 2, 2, 1, 3}, 2))
}
