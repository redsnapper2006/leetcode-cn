package main

func findDuplicates(nums []int) []int {
	ret := []int{}
	for i := 0; i < len(nums); i++ {
		offset := nums[i]
		if offset < 0 {
			offset = -offset
		}
		nums[offset-1] = -nums[offset-1]
		if nums[offset-1] > 0 {
			ret = append(ret, offset)
		}
	}

	return ret
}
