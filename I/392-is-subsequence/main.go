package main

import "fmt"

func isSubsequence(s string, t string) bool {
	sIdx, tIdx := 0, 0

	for sIdx < len(s) && tIdx < len(t) {
		if s[sIdx] == t[tIdx] {
			tIdx++
		}
		sIdx++
	}
	if tIdx == len(t) {
		return true
	}
	return false
}

func main() {
	fmt.Println("a")
}
