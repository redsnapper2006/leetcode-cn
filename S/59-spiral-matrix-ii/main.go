package main

import "fmt"

func generateMatrix(n int) [][]int {
	buf := make([][]int, n)
	for i := 0; i < n; i++ {
		buf[i] = make([]int, n)
	}
	direction := 0
	steps := n
	r, c := 0, 0
	s := 0
	for i := 0; i < n*n; i++ {
		buf[r][c] = i + 1
		s++
		if s == steps {
			direction++
			direction %= 4
			if direction%2 == 1 {
				steps--
			}
			s = 0
		}
		if direction == 0 {
			c++
		} else if direction == 1 {
			r++
		} else if direction == 2 {
			c--
		} else {
			r--
		}
	}
	return buf
}

func main() {
	fmt.Println(generateMatrix(5))
}
