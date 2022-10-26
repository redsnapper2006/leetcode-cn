package main

func divideArray(nums []int) bool {
	m := map[int]int{}
	for _, n := range nums {
		m[n]++
	}
	for _, v := range m {
		if v%2 != 0 {
			return false
		}
	}
	return true
}
