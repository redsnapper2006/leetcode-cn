package main

import "fmt"

func diffWaysToCompute(expression string) []int {
	nums := []int{}
	ops := []byte{}
	cnt := 0
	for _, b := range expression {
		if byte(b) >= '0' && byte(b) <= '9' {
			cnt = cnt*10 + int(b-'0')
		} else {
			nums = append(nums, cnt)
			ops = append(ops, byte(b))
			cnt = 0
		}
	}
	nums = append(nums, cnt)
	buf := make([][][]int, len(nums))
	for i := 0; i < len(buf); i++ {
		buf[i] = make([][]int, len(nums))
	}

	var recur func(buf [][][]int, nums []int, ops []byte, start int, end int) []int
	recur = func(buf [][][]int, nums []int, ops []byte, start int, end int) []int {
		if buf[start][end] != nil {
			return buf[start][end]
		}
		if start == end {
			buf[start][end] = []int{nums[start]}
			return buf[start][end]
		}
		ret := []int{}
		for i := start; i < end; i++ {
			left := recur(buf, nums, ops, start, i)
			right := recur(buf, nums, ops, i+1, end)
			for _, l := range left {
				for _, r := range right {
					v := 0
					if ops[i] == '+' {
						v = l + r
					} else if ops[i] == '-' {
						v = l - r
					} else {
						v = l * r
					}
					ret = append(ret, v)
				}
			}
		}
		buf[start][end] = ret
		return ret
	}
	return recur(buf, nums, ops, 0, len(nums)-1)
}

func main() {
	fmt.Println(diffWaysToCompute("2-1-1"))
	fmt.Println(diffWaysToCompute("2*3-4*5"))
}
