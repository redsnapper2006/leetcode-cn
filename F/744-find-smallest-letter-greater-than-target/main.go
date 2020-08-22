package main

import "fmt"

func nextGreatestLetter(letters []byte, target byte) byte {
	s, e := 0, len(letters)-1
	for s < e {
		m := s + (e-s)/2
		if letters[m] <= target {
			s = m + 1
		} else {
			e = m
		}
	}
	if e == len(letters)-1 && letters[e] <= target {
		return letters[0]
	}
	if letters[e] > target {
		return letters[e]
	} else {
		return letters[e+1]
	}
}

func main() {
	fmt.Println("a")
}
