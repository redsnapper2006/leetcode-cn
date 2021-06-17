package main

import (
	"fmt"
)

func numberOfSubarrays(nums []int, k int) int {
	s := 0
	c := 0
	p := 0
	sum := 0
	for s < len(nums) {
		if nums[s]%2 == 1 {
			c++
		}

		if c == k+1 {
			pp := p
			for nums[p]%2 == 0 {
				p++
			}
			t := s - 1
			for nums[t]%2 == 0 {
				t--
			}
			sum += (p - pp + 1) * (s - t)
			c--
			p++
		}
		s++
	}
	if c == k {
		pp := p
		for nums[p]%2 == 0 {
			p++
		}
		t := len(nums) - 1
		for nums[t]%2 == 0 {
			t--
		}
		sum += (p - pp + 1) * (s - t)
	}
	return sum
}

func main() {
	fmt.Println("a")
}
