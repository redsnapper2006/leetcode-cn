package main

import (
	"fmt"
)

func reverseString(s []byte) {
	i := 0
	j := len(s) - 1
	for i < j {
		t := s[i]
		s[i] = s[j]
		s[j] = t
		i++
		j--
	}
}

func main() {
	ba := []byte("hell")

	fmt.Println(string(ba))
	reverseString(ba)
	fmt.Println(string(ba))
}
