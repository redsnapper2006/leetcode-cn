package main

import "fmt"

func executeInstructions(n int, startPos []int, s string) []int {
	ret := []int{}

	for i := 0; i < len(s); i++ {
		x, y := startPos[0], startPos[1]
		cmds := 0
		for j := i; j < len(s); j++ {
			if s[j] == byte('L') {
				y--
			}
			if s[j] == byte('R') {
				y++
			}
			if s[j] == byte('U') {
				x--
			}
			if s[j] == byte('D') {
				x++
			}
			if !(x >= 0 && x < n && y >= 0 && y < n) {
				break
			}
			cmds++
		}
		ret = append(ret, cmds)
	}
	return ret
}

func main() {
	fmt.Println()
}
