package main

func arithmeticTriplets(nums []int, diff int) int {
	ret := 0
	m := map[int]int{}
	for _, n := range nums {
		m[n] = 1
	}
	for i := 0; i < len(nums)-2; i++ {
		n2, n3 := nums[i]+diff, nums[i]+diff+diff
		_, ok2 := m[n2]
		_, ok3 := m[n3]
		if ok2 && ok3 {
			ret++
		}
	}
	return ret
}
