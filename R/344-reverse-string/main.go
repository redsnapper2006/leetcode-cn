package main

import (
	"fmt"
)

func reverseStringSwap(s []byte) {
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


func reverseString(s []byte) {
	if len(s) <= 1 {
		return
	}
	t := s[0]
	for i:=1;i<len(s);i++ {
		s[i-1] = s[i]
	}
	s[len(s)-1] =t
	return reverseString(s[0:len(s)-1])
}

func main() {
	ba := []byte("hell")

	fmt.Println(string(ba))
	reverseString(ba)
	fmt.Println(string(ba))
}
