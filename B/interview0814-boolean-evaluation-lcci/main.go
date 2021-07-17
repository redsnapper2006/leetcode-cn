package main

import "fmt"

func countEval(s string, result int) int {
	cached := make([][][]int, len(s))
	for i := 0; i < len(s); i++ {
		cached[i] = make([][]int, len(s)+1)
		for j := 0; j < len(s)+1; j++ {
			cached[i][j] = []int{}
		}
	}

	var calc func(str string, s, e int, cached [][][]int) []int
	calc = func(str string, s, e int, cached [][][]int) []int {
		if len(cached[s][e]) != 0 {
			return cached[s][e]
		}
		cached[s][e] = []int{0, 0}
		if s+1 == e {
			if str[s] == byte('0') {
				cached[s][e] = []int{1, 0}
			} else {
				cached[s][e] = []int{0, 1}
			}
			return cached[s][e]
		}

		for i := s + 1; i < e; i = i + 2 {
			left := calc(str, s, i, cached)
			right := calc(str, i+1, e, cached)
			if str[i] == byte('&') {
				cached[s][e][0] += left[0]*right[0] + left[0]*right[1] + left[1]*right[0]
				cached[s][e][1] += left[1] * right[1]
			} else if str[i] == byte('|') {
				cached[s][e][0] += left[0] * right[0]
				cached[s][e][1] += left[0]*right[1] + left[1]*right[0] + left[1]*right[1]
			} else {
				cached[s][e][0] += left[0]*right[0] + left[1]*right[1]
				cached[s][e][1] += left[0]*right[1] + left[1]*right[0]
			}
		}
		return cached[s][e]
	}

	calc(s, 0, len(s), cached)

	return cached[0][len(s)][result]
}

func main() {
	fmt.Println(countEval("1^0|0|1", 0))
	fmt.Println(countEval("0&0&0&1^1|0", 1))
}
