package main

import "fmt"

func minimumMoves(s string) int {
	idx := 0

	ret := 0
	for idx < len(s) {
		if s[idx] == byte('X') {
			ret++
			idx += 3
		} else {
			idx++
		}
	}
	return ret
}

func main() {
	fmt.Println()
}
