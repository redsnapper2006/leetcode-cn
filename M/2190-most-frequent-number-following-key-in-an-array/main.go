package main

func mostFrequent(nums []int, key int) int {
	ret := 0
	m := map[int]int{}
	for i := 0; i < len(nums)-1; i++ {
		if nums[i] == key {
			m[nums[i+1]]++
			ret = nums[i+1]
		}
	}
	max := m[ret]
	for k, v := range m {
		if v > max {
			max = v
			ret = k
		}
	}
	return ret
}
