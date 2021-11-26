package main

import "fmt"

func winnerOfGame(colors string) bool {
	base := colors[0]

	A, B := 0, 0
	cnt := 1
	for i := 1; i < len(colors); i++ {
		if colors[i] == base {
			cnt++
		} else {
			if cnt >= 2 {
				cnt -= 2
			} else {
				cnt = 0
			}
			if base == byte('A') {
				A += cnt
			} else {
				B += cnt
			}
			cnt = 1
			base = colors[i]
		}
	}
	if cnt >= 2 {
		cnt -= 2
	} else {
		cnt = 0
	}
	if base == byte('A') {
		A += cnt
	} else {
		B += cnt
	}
	return A > B
}

func main() {
	fmt.Println()
}
