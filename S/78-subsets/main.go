package main

import "fmt"

func subsets(nums []int) [][]int {
	if len(nums) == 0 {
		return [][]int{}
	}
	buf := [][]int{[]int{}, []int{nums[0]}}
	for i := 1; i < len(nums); i++ {
		size := len(buf)
		t := make([][]int, size*2)
		copy(t, buf)

		for j := 0; j < size; j++ {
			var b []int
			for m := 0; m < len(buf[j]); m++ {
				b = append(b, buf[j][m])
			}
			t[size+j] = append(b, nums[i])
		}

		buf = t
	}
	return buf
}

func main() {
	fmt.Println(subsets([]int{1, 2, 3, 4, 5, 6}))
}
