package main

import "fmt"

func largeGroupPositions(S string) [][]int {
	bb := '0'
	sIdx := -1
	var ret [][]int
	for i, b := range S {
		if b != bb {
			if sIdx != -1 && i-sIdx >= 3 {
				ret = append(ret, []int{sIdx, i - 1})
			}
			sIdx = i
			bb = b
		}
	}
	if sIdx != -1 && len(S)-sIdx >= 3 {
		ret = append(ret, []int{sIdx, len(S) - 1})
	}
	return ret
}

func main() {
	fmt.Println("a")
}
