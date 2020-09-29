package main

import "fmt"

func isPathCrossing(path string) bool {
	x, y := 0, 0
	M := map[string]int{"0_0": 1}
	for _, v := range path {
		if v == 'N' {
			y++
		}
		if v == 'E' {
			x++
		}
		if v == 'S' {
			y--
		}
		if v == 'W' {
			x--
		}
		s := fmt.Sprintf("%d_%d", x, y)
		M[s]++
		if M[s] > 1 {
			return false
		}
	}
	return true
}

func main() {
	fmt.Println("a")
}
