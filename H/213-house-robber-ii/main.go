package main

import "fmt"

func rob(nums []int) int {
	if len(nums) == 0 {
		return 0
	}
	if len(nums) == 1 {
		return nums[0]
	}
	buf1 := make([]int, len(nums)-1)
	buf2 := make([]int, len(nums)-1)

	for i := 0; i < len(nums)-1; i++ {
		b1, b2 := 0, 0
		if i-3 >= 0 {
			b1 = buf1[i-3]
		}
		if i-2 >= 0 {
			b2 = buf1[i-2]
		}
		t := b1
		if t < b2 {
			t = b2
		}
		buf1[i] = t + nums[i]
	}

	for i := 1; i < len(nums); i++ {
		b1, b2 := 0, 0
		if i-3 >= 1 {
			b1 = buf2[i-4]
		}
		if i-2 >= 1 {
			b2 = buf2[i-3]
		}
		t := b1
		if t < b2 {
			t = b2
		}
		buf2[i-1] = t + nums[i]
	}
	fmt.Println(buf1, buf2)
	max := -1
	for i := 0; i < len(buf1); i++ {
		if max < buf1[i] {
			max = buf1[i]
		}
	}
	for i := 0; i < len(buf2); i++ {
		if max < buf2[i] {
			max = buf2[i]
		}
	}
	return max
}

func main() {
	fmt.Println("a")
}
