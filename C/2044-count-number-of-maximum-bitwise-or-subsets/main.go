package main

import "fmt"

func countMaxOrSubsets(nums []int) int {
	m := map[int]int{}
	m[0] = 1
	sum := 0
	for _, n := range nums {
		sum |= n
		b := map[int]int{}
		for k, v := range m {
			nn := k | n
			b[nn] += v
		}
		for k, v := range b {
			m[k] += v
		}
	}

	return m[sum]
}

func main() {
	fmt.Println(countMaxOrSubsets([]int{3, 1}))
	fmt.Println(countMaxOrSubsets([]int{2, 2, 2}))

}
