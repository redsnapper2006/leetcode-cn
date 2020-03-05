package main

import "fmt"

func longestConsecutive(nums []int) int {
	if len(nums) <= 1 {
		return len(nums)
	}
	max := 0
	m := make(map[int]int)
	for i := 0; i < len(nums); i++ {
		_, ok := m[nums[i]]
		if !ok {
			l := m[nums[i]-1]
			r := m[nums[i]+1]
			a := l + 1 + r
			m[nums[i]] = a
			m[nums[i]-l] = a
			m[nums[i]+r] = a
			if a > max {
				max = a
			}
		}
	}

	return max
}

func longestConsecutiveV2(nums []int) int {
	if len(nums) <= 1 {
		return len(nums)
	}

	size := len(nums)
	min, max := nums[0], nums[0]
	for i := 1; i < len(nums); i++ {
		if nums[i] < min {
			min = nums[i]
		}
		if nums[i] > max {
			max = nums[i]
		}
	}

	m := make(map[int][]int)
	for i := 0; i < len(nums); i++ {
		idx := (nums[i] - min) / size
		offset := (nums[i] - min) % size
		_, ok := m[idx]
		if !ok {
			t := make([]int, size)
			t[offset] = 1
			m[idx] = t
		} else {
			m[idx][offset] = 1
		}
	}

	rm := make(map[int][]int)
	for k, v := range m {
		s, m, e := 0, 0, 0
		a := 0
		for j := 0; j < len(v); j++ {
			if v[j] == 1 {
				a++
			} else {
				if a > 0 {
					if a == j {
						s = a
					} else {
						if a > m {
							m = a
						}
					}
				}
				a = 0
			}
		}
		if a > 0 {
			if a == len(v) {
				return a
			} else {
				e = a
			}
		}
		rm[k] = []int{s, m, e}
	}
	// fmt.Println(m)
	// fmt.Println(rm, min, max)
	ret := 0
	for k, v := range rm {
		s, m, e := v[0], v[1], v[2]
		b, ok := rm[k-1]
		if ok && b[2] > 0 {
			s += b[2]
		}
		a, ok := rm[k+1]
		if ok && a[0] > 0 {
			e += a[0]
		}
		if s > ret {
			ret = s
		}
		if m > ret {
			ret = m
		}
		if e > ret {
			ret = e
		}
	}

	return ret
}

func main() {
	// fmt.Println(longestConsecutive([]int{2147483646, -2147483647, 0, 2, 2147483644, -2147483645, 2147483645}))
	fmt.Println(longestConsecutive([]int{9, 1, 4, 7, 3, -1, 0, 5, 8, -1, 6}))
}
