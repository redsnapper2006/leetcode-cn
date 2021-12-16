package main

import "fmt"

func countWords(words1 []string, words2 []string) int {
	m1, m2 := map[string]int{}, map[string]int{}
	for _, w := range words1 {
		m1[w]++
	}
	for _, w := range words2 {
		m2[w]++
	}
	cnt := 0
	for k, v := range m1 {
		if v > 1 {
			continue
		}
		v2, ok := m2[k]
		if !ok || v2 > 1 {
			continue
		}
		cnt++
	}
	return cnt
}

func main() {
	fmt.Println()
}
