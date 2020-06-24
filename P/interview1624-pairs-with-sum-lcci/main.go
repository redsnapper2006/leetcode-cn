package main

import "fmt"

func pairSums(nums []int, target int) [][]int {
	buf := map[int]int{}
	for i := 0; i < len(nums); i++ {
		buf[nums[i]]++
	}

	var ret [][]int
	for i := 0; i < len(nums); i++ {
		b := nums[i]
		_, ok1 := buf[b]
		if !ok1 {
			continue
		}
		buf[b]--
		if buf[b] == 0 {
			delete(buf, b)
		}

		d := target - b
		_, ok2 := buf[d]
		if !ok2 {
			continue
		}
		ret = append(ret, []int{b, d})

		buf[d]--
		if buf[d] == 0 {
			delete(buf, d)
		}
	}

	return ret
}

func main() {
	fmt.Println("a")
}
