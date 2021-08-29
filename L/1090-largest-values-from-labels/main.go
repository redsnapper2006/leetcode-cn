package main

import (
	"fmt"
	"sort"
)

func largestValsFromLabels(values []int, labels []int, num_wanted int, use_limit int) int {
	m := map[int][]int{}
	for i := 0; i < len(values); i++ {
		_, ok := m[labels[i]]
		if !ok {
			m[labels[i]] = []int{}
		}
		m[labels[i]] = append(m[labels[i]], values[i])
	}
	for _, v := range m {
		sort.Ints(v)
	}
	cache := []int{}
	for _, v := range m {
		start := len(v) - use_limit
		if start < 0 {
			start = 0
		}
		cache = append(cache, v[start:]...)
	}

	sort.Ints(cache)
	sum := 0
	for j := len(cache) - 1; j >= 0 && num_wanted > 0; j-- {
		sum += cache[j]
		num_wanted--
	}

	return sum
}

func main() {
	fmt.Println()
}
