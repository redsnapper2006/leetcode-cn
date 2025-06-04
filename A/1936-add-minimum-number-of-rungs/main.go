package main

import "fmt"

func addRungs(rungs []int, dist int) int {
	step := 0
	ret := 0
	for i := 0; i < len(rungs); i++ {
		c := (rungs[i] - step) / dist
		if (rungs[i]-step)%dist == 0 {
			c--
		}
		ret += c
		step = rungs[i]
	}

	return ret
}

func main() {
	fmt.Println(addRungs([]int{1, 3, 5, 10}, 2))
}
