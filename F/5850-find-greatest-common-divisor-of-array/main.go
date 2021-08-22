package main

import "fmt"

func findGCD(nums []int) int {
	min, max := nums[0], nums[0]
	for _, v := range nums {
		if min > v {
			min = v
		}
		if max < v {
			max = v
		}
	}

	var GCD func(x, y int) int
	GCD = func(x, y int) int {
		p1, p2 := x, y
		if p2 > p1 {
			p1, p2 = y, x
		}
		if p1%p2 == 0 {
			return p2
		}
		return GCD(p1%p2, p2)
	}

	return GCD(min, max)

}

func main() {
	fmt.Println()
}
