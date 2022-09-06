package main

func numberOfPairs(nums []int) []int {
	m := map[int]int{}
	for _, n := range nums {
		m[n]++
	}
	p, r := 0, 0
	for _, v := range m {
		p += v / 2
		r += v % 2
	}
	return []int{p, r}
}
