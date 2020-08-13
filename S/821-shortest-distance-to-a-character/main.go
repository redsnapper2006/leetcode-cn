package main

import "fmt"

func shortestToChar(S string, C byte) []int {
	var pos []int
	for i := 0; i < len(S); i++ {
		if S[i] == C {
			pos = append(pos, i)
		}
	}
	var ret []int
	for i := 0; i < len(S); i++ {
		if S[i] == C {
			ret = append(ret, 0)
		} else {
			min := len(S) + 1
			for j := 0; j < len(pos); j++ {
				p := pos[j] - i
				if p < 0 {
					p = -p
				}
				if p < min {
					min = p
				}
			}
			ret = append(ret, min)
		}
	}
	return ret
}

func main() {
	fmt.Println("a")
}
