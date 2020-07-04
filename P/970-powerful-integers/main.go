package main

import (
	"fmt"
	"sort"
)

func powerfulIntegers(x int, y int, bound int) []int {
	if x == 1 && y == 1 {
		if bound < 2 {
			return nil
		}
		return []int{2}
	}
	if x == 1 || y == 1 {
		var a, b int
		if x == 1 {
			a = y
			b = 1
		} else {
			a = x
			b = 1
		}
		r := []int{2}
		acc := a
		for acc+b <= bound {
			r = append(r, acc+b)
			acc *= a
		}
		return r
	}

	var recur func(ax, ay, x, y, bound int, ret *[]int)
	recur = func(ax, ay, x, y, bound int, ret *[]int) {
		if ax+ay > bound {
			return
		}
		*ret = append(*ret, ax+ay)
		recur(ax*x, ay, x, y, bound, ret)
		recur(ax, ay*y, x, y, bound, ret)
	}
	var ret []int
	recur(1, 1, x, y, bound, &ret)
	if len(ret) == 0 {
		return nil
	}
	sort.Ints(ret)
	var r []int
	r = append(r, ret[0])
	for i := 1; i < len(ret); i++ {
		if ret[i] == r[len(r)-1] {
			continue
		}
		r = append(r, ret[i])
	}
	return r
}

func main() {
	fmt.Println(powerfulIntegers(2, 1, 10))
}
