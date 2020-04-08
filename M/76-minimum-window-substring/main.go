package main

import "fmt"

func minWindow(s string, t string) string {
	TM := make([]int, 256)
	for i := 0; i < len(t); i++ {
		TM[t[i]]++
	}

	isEnough := func(SM []int, TM []int) bool {
		for i := 0; i < len(TM); i++ {
			if SM[i] < TM[i] {
				return false
			}
		}
		return true
	}
	SM := make([]int, 256)
	sIdx, r := 0, s
	isExist := false
	for i := 0; i < len(s); i++ {
		SM[s[i]]++

		for isEnough(SM, TM) {
			isExist = true
			if len(r) > len(s[sIdx:i+1]) {
				r = s[sIdx : i+1]
			}
			SM[s[sIdx]]--
			sIdx++
		}
	}
	if isExist {
		return r
	}
	return ""
}

func main() {
	fmt.Println(minWindow("a", "aa"))
}
