package main

func expectNumber(scores []int) int {
	m := map[int]int{}
	for _, v := range scores {
		m[v]++
	}
	return len(m)
}
