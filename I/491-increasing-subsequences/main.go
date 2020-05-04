package main

import (
	"fmt"
	"strconv"
	"strings"
)

func findSubsequences(nums []int) [][]int {
	if len(nums) == 1 {
		return nil
	}
	var ret [][]int
	for i := len(nums) - 2; i >= 0; i-- {
		size := len(ret)
		for j := 0; j < size; j++ {
			if nums[i] <= ret[j][0] {
				t := make([]int, len(ret[j])+1)
				copy(t[1:], ret[j])
				t[0] = nums[i]
				ret = append(ret, t)
			}
		}
		for j := i + 1; j < len(nums); j++ {
			if nums[i] <= nums[j] {
				ret = append(ret, []int{nums[i], nums[j]})
			}
		}
	}
	if len(ret) <= 1 {
		return ret
	}

	M := make(map[string]int)
	for i := 0; i < len(ret); i++ {
		s := ""
		for j := 0; j < len(ret[i]); j++ {
			s += strconv.FormatInt(int64(ret[i][j]), 10) + "_"
		}
		M[s]++
	}

	var r [][]int
	for k, _ := range M {
		arr := strings.Split(k, "_")
		num := make([]int, len(arr)-1)
		for i := 0; i < len(arr)-1; i++ {
			v, _ := strconv.Atoi(arr[i])
			num[i] = v
		}
		r = append(r, num)
	}#
	return r
}

func main() {
	fmt.Println("a")
}
