package main

import "fmt"

func thirdMax(nums []int) int {
	m1, m2, m3 := 0, 0, 0
	s1, s2, s3 := false, false, false

	for i := 0; i < len(nums); i++ {
		if s1 == false {
			s1 = true
			m1 = nums[i]
			continue
		}
		if s2 == false {
			if m1 == nums[i] {
				continue
			}
			s2 = true
			if m1 < nums[i] {
				m2 = m1
				m1 = nums[i]
			} else {
				m2 = nums[i]
			}
			continue
		}
		if s3 == false {
			if m1 == nums[i] || m2 == nums[i] {
				continue
			}
			s3 = true
			if m1 < nums[i] {
				m3 = m2
				m2 = m1
				m1 = nums[i]
			} else if m2 < nums[i] {
				m3 = m2
				m2 = nums[i]
			} else {
				m3 = nums[i]
			}
			continue
		}
		if m1 == nums[i] || m2 == nums[i] || m3 == nums[i] {
			continue
		}
		if m3 < nums[i] && m2 > nums[i] {
			m3 = nums[i]
		} else if m2 < nums[i] && m1 > nums[i] {
			m3 = m2
			m2 = nums[i]
		} else if m1 < nums[i] {
			m3 = m2
			m2 = m1
			m1 = nums[i]
		}
	}
	if s3 == false {
		return m1
	}
	return m3
}

func main() {
	fmt.Println("a")
}
