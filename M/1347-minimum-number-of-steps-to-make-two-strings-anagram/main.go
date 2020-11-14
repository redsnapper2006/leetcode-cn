package main

import "fmt"

func minSteps(s string, t string) int {
	bufS, bufT := make([]int, 26), make([]int, 26)
	for i := 0; i < len(s); i++ {
		bufS[s[i]-'a']++
		bufT[t[i]-'a']++
	}

	sum := 0
	for i := 0; i < 26; i++ {
		if bufS[i]-bufT[i] > 0 {
			sum += bufS[i] - bufT[i]
		}
	}
	return sum
}

func main() {
	fmt.Println()
}
