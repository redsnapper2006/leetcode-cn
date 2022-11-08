package main

import "fmt"

func countConsistentStrings(allowed string, words []string) int {
	buf := make([]int, 26)
	for _, v := range allowed {
		buf[int(v-'a')]++
	}
	count := 0
	for _, word := range words {
		isIn := true
		for _, b := range word {
			if buf[int(b-'a')] == 0 {
				isIn = false
				break
			}
		}
		if isIn {
			count++
		}
	}
	return count
}

func main() {
	fmt.Println()
}
