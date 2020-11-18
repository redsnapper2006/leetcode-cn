package main

import "fmt"

func findClosest(words []string, word1 string, word2 string) int {
	M := map[string][]int{}
	for i, c := range words {
		_, ok := M[c]
		if !ok {
			M[c] = []int{}
		}
		M[c] = append(M[c], i)
	}
	l1 := M[word1]
	l2 := M[word2]
	i1, i2 := 0, 0
	min := len(words) + 1
	for i1 < len(l1) && i2 < len(l2) {
		s := l1[i1] - l2[i2]
		if s < 0 {
			s = -s
		}
		if s < min {
			min = s
		}
		if l1[i1] < l2[i2] {
			i1++
		} else {
			i2++
		}
	}
	if i1 < len(l1) {
		for i := i1; i < len(l1); i++ {
			s := l1[i] - l2[len(l2)-1]
			if s < 0 {
				s = -s
			}
			if s < min {
				min = s
			}
		}
	}
	if i2 < len(l2) {
		for i := i2; i < len(l2); i++ {
			s := l1[len(l1)-1] - l2[i]
			if s < 0 {
				s = -s
			}
			if s < min {
				min = s
			}
		}
	}
	return min
}

func main() {
	fmt.Println()
}
