package main

import "fmt"

func areOccurrencesEqual(s string) bool {
	buf := make([]int, 26)
	for _, b := range s {
		buf[int(b-'a')]++
	}
	cnt := -1
	for _, c := range buf {
		if c == 0 {
			continue
		}
		if cnt == -1 {
			cnt = c
		} else if cnt != c {
			return false
		}
	}
	return true
}

func main() {
	fmt.Println()
}
