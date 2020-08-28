package main

import "fmt"

func judgeCircle(moves string) bool {
	buf := make([]int, 2)
	for i := 0; i < len(moves); i++ {
		if moves[i] == 'R' {
			buf[0]++
		}
		if moves[i] == 'L' {
			buf[0]--
		}
		if moves[i] == 'U' {
			buf[1]++
		}
		if moves[i] == 'D' {
			buf[1]--
		}
	}
	return buf[0] == 0 && buf[1] == 0
}

func main() {
	fmt.Println("a")
}
