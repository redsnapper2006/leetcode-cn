package main

import "fmt"

func rotatedDigits(N int) int {
	s := 1
	r := 0
	for s <= N {
		t := s
		isSkip := false
		isGood := false
		for t > 0 {
			m := t % 10
			t = t / 10
			if m == 3 || m == 4 || m == 7 {
				isSkip = true
				break
			}
			if m == 2 || m == 5 || m == 6 || m == 9 {
				isGood = true
			}
		}
		if isSkip {
			s++
			continue
		}
		if isGood {
			r++
		}
		s++
	}
	return r
}

func main() {
	fmt.Println("a")
}
