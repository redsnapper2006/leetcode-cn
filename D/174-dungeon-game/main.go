package main

import "fmt"

func calculateMinimumHP(dungeon [][]int) int {
	buf := make([][]int, len(dungeon))
	need := make([][]int, len(dungeon))
	for i := 0; i < len(dungeon); i++ {
		buf[i] = make([]int, len(dungeon[0]))
		need[i] = make([]int, len(dungeon[0]))
	}

	for i := len(dungeon) - 1; i >= 0; i-- {
		for j := len(dungeon[0]) - 1; j >= 0; j-- {
			if i == len(dungeon)-1 && j == len(dungeon[0])-1 {
				if dungeon[i][j] > 0 {
					buf[i][j] = 1
				} else {
					buf[i][j] = -dungeon[i][j] + 1
				}
				continue
			}
			if i == len(dungeon)-1 {
				if buf[i][j+1]-dungeon[i][j] <= 0 {
					buf[i][j] = 1
				} else {
					buf[i][j] = buf[i][j+1] - dungeon[i][j]
				}
				continue
			}
			if j == len(dungeon[0])-1 {
				if buf[i+1][j]-dungeon[i][j] <= 0 {
					buf[i][j] = 1
				} else {
					buf[i][j] = buf[i+1][j] - dungeon[i][j]
				}
				continue
			}
			var downSupply, rightSupply int
			if buf[i+1][j]-dungeon[i][j] <= 0 {
				downSupply = 1
			} else {
				downSupply = buf[i+1][j] - dungeon[i][j]
			}
			if buf[i][j+1]-dungeon[i][j] <= 0 {
				rightSupply = 1
			} else {
				rightSupply = buf[i][j+1] - dungeon[i][j]
			}

			buf[i][j] = downSupply
			if downSupply > rightSupply {
				buf[i][j] = rightSupply
			}
		}
	}

	return buf[0][0]
}

func main() {
	fmt.Println("a")
}
