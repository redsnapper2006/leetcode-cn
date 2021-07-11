package main

import (
	"fmt"
)

func countPalindromicSubsequence(s string) int {
	buf := make([][]int, len(s))
	pos := make([][]int, 26)
	for i := 0; i < 26; i++ {
		pos[i] = []int{}
	}
	for i, b := range s {
		buf[i] = make([]int, 26)
		if i > 0 {
			for j := 0; j < 26; j++ {
				buf[i][j] = buf[i-1][j]
			}
		}
		buf[i][int(byte(b)-'a')]++
		pos[int(byte(b)-'a')] = append(pos[int(byte(b)-'a')], i)
	}
	// fmt.Println(buf, pos)

	ret := 0
	for i := 0; i < 26; i++ {
		if len(pos[i]) <= 1 {
			continue
		}
		mm := map[int]int{}

		p1, p2 := pos[i][0], pos[i][len(pos[i])-1]

		// fmt.Println(p1, p2)
		for m := 0; m < 26; m++ {
			if buf[p2-1][m]-buf[p1][m] > 0 {
				mm[m]++
			}
		}

		// fmt.Println(mm)
		ret += len(mm)
	}
	return ret
}

func main() {
	fmt.Println(countPalindromicSubsequence("aabca"))
}
