package main

func findLonely(nums []int) []int {
	m := map[int]int{}
	for _, v := range nums {
		m[v]++
	}
	ret := []int{}
	for k, v := range m {
		if v > 1 {
			continue
		}
		if _, ok1 := m[k-1];_,ok2:=m[k+1]; ok1||ok2 {
			continue
		}
		ret = append(ret, k)
	}
	return ret
}
