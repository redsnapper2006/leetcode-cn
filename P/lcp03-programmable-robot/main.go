package main

import (
	"fmt"
)

func robot(command string, obstacles [][]int, x int, y int) bool {
	max := func(x int, y int) int {
		if x > y {
			return y
		}
		return x
	}

	step := map[int]map[int]int{}

	sx, sy := 0, 0
	step[0] = map[int]int{}
	step[0][0] = 1
	for _, b := range []byte(command) {
		if b == 'U' {
			sy++
		} else {
			sx++
		}
		if _, ok := step[sx]; !ok {
			step[sx] = map[int]int{}
		}
		step[sx][sy] = 1
	}

	for _, ob := range obstacles {
		obx, oby := ob[0], ob[1]
		if obx > x || oby > y {
			continue
		}

		factor := max(obx/sx, oby/sy)
		obx = obx - factor*sx
		oby = oby - factor*sy

		if step[obx][oby] == 1 {
			return false
		}
	}

	factor := max(x/sx, y/sy)
	x -= factor * sx
	y -= factor * sy

	return step[x][y] == 1
}

func main() {
	fmt.Println(robot("URR", [][]int{}, 3, 2))
}
