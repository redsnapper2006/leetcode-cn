package main

import "fmt"

func diStringMatch(S string) []int {
	size := len(S)
	s, e := 0, size

	var r []int
	for i := 0; i < len(S); i++ {
		if S[i] == 'I' {
			r = append(r, s)
			s++
		} else {
			r = append(r, e)
			e--
		}
	}
	r = append(r, s)
	return r
}

func main() {
	fmt.Println("a")
}
