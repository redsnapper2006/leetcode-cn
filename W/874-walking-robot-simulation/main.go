package main

import "fmt"

func robotSim(commands []int, obstacles [][]int) int {
	// 0 North, 1 East, 2 South, 3 West
	direction := 0
	x, y := 0, 0
	max := -1
	for _, command := range commands {
		if command == -1 {
			if direction == 0 {
				direction = 1
			} else if direction == 1 {
				direction = 2
			} else if direction == 2 {
				direction = 3
			} else {
				direction = 0
			}
		} else if command == -2 {
			if direction == 0 {
				direction = 3
			} else if direction == 1 {
				direction = 0
			} else if direction == 2 {
				direction = 1
			} else {
				direction = 2
			}
		} else {
			startx, starty := x, y
			endx, endy := x, y
			if direction == 0 {
				endy += command
			} else if direction == 1 {
				endx += command
			} else if direction == 2 {
				endy -= command
			} else {
				endx -= command
			}
			for _, obstacle := range obstacles {
				if direction == 0 {
					if obstacle[0] == startx && obstacle[1] > starty && obstacle[1] <= endy {
						endy = obstacle[1] - 1
					}
				} else if direction == 1 {
					if obstacle[1] == starty && obstacle[0] > startx && obstacle[0] <= endx {
						endx = obstacle[0] - 1
					}
				} else if direction == 2 {
					if obstacle[0] == startx && obstacle[1] >= endy && obstacle[1] < starty {
						endy = obstacle[1] + 1
					}
				} else {
					if obstacle[1] == starty && obstacle[0] >= endx && obstacle[0] < startx {
						endx = obstacle[0] + 1
					}
				}
			}
			x, y = endx, endy
			if x*x+y*y > max {
				max = x*x + y*y
			}
		}
	}
	return max
}

func main() {
	fmt.Println(robotSim([]int{1, 2, -2, 5, -1, -2, -1, 8, 3, -1, 9, 4, -2, 3, 2, 4, 3, 9, 2, -1, -1, -2, 1, 3, -2, 4, 1, 4, -1, 1, 9, -1, -2, 5, -1, 5, 5, -2, 6, 6, 7, 7, 2, 8, 9, -1, 7, 4, 6, 9, 9, 9, -1, 5, 1, 3}, [][]int{}))
}
