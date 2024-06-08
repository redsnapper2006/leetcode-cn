package main

func maxOperations(nums []int) int {
	m := map[int]map[int]int{}

	var recur func(nums []int, start int, end int, sum int) int
	recur = func(nums []int, start int, end int, sum int) int {
		if vs, ok := m[start]; ok {
			if ve, ok2 := vs[end]; ok2 {
				return ve
			}
		}

		if start >= end {
			return 0
		}

		ans := 0
		if nums[start]+nums[start+1] == sum {
			v := recur(nums, start+2, end, sum) + 1
			if v > ans {
				ans = v
			}
		}
		if nums[end]+nums[end-1] == sum {
			v := recur(nums, start, end-2, sum) + 1
			if v > ans {
				ans = v
			}
		}
		if nums[start]+nums[end] == sum {
			v := recur(nums, start+1, end-1, sum) + 1
			if v > ans {
				ans = v
			}
		}
		if _, ok := m[start]; !ok {
			m[start] = map[int]int{}
		}
		m[start][end] = ans
		return ans
	}

	head := recur(nums, 2, len(nums)-1, nums[0]+nums[1])
	mid := recur(nums, 1, len(nums)-2, nums[0]+nums[len(nums)-1])
	tail := recur(nums, 0, len(nums)-3, nums[len(nums)-2]+nums[len(nums)-1])

	ret := head
	if ret < mid {
		ret = mid
	}

	if ret < tail {
		ret = tail
	}
	return ret + 1
}
