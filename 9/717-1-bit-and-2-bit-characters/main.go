package main

import "fmt"

func isOneBitCharacter(bits []int) bool {
	s := 0
	for s < len(bits)-1 {
		if bits[s] == 0 {
			s++
		} else {
			s += 2
		}
	}
	return s == len(bits)-1
}

func main() {
	fmt.Println("a")
}
