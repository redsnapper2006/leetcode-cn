package main

import "fmt"

func binaryGap(N int) int {
	p := 30
	s := 0
	for i := 30; i >= 0; i-- {
		if N&(1<<i) > 0 {
			p = i
			break
		}
	}

	for i := p - 1; i >= 0; i-- {
		if N&(1<<i) > 0 {
			if s < p-i {
				s = p - i
			}
			p = i
		}
	}
	return s
}

func main() {
	fmt.Println("a")
}
