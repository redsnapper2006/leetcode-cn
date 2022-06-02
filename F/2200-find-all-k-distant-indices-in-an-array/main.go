package main

func findKDistantIndices(nums []int, key int, k int) []int {

	ret := []int{}
	buf := make([]int, len(nums))
	idx := 0
	for idx < len(nums) {
		if nums[idx] != key {
			idx++
			continue
		}
		l, r := idx-k, idx+k
		if l < 0 {
			l = 0
		}
		if r >= len(nums) {
			r = len(nums) - 1
		}
		for l <= r {
			buf[l] = 1
			l++
		}
		idx++
	}
	for i := 0; i < len(nums); i++ {
		if buf[i] == 1 {
			ret = append(ret, i)
		}
	}

	return ret
}
