package main

func minimumOperations(nums []int) int {
	m := map[int]int{}
	for _, n := range nums {
		if n == 0 {
			continue
		}
		m[n]++

	}

	return len(m)
}
