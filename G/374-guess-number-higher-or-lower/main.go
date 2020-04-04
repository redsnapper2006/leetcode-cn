package main

import "fmt"

func guess(num int) int {
	if num > 6 {
		return -1
	} else if num < 6 {
		return 1
	}
	return 0
}

func guessNumber(n int) int {
	s, e := 1, n

	for {
		m := s + (e-s)/2
		r := guess(m)
		if r == 0 {
			return m
		}
		if r == 1 {
			s = m + 1
		}
		if r == -1 {
			e = m - 1
		}
	}
	return 0
}

func main() {
	fmt.Println(guessNumber(10))
}
