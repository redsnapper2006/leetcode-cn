package main

import (
	"fmt"
	"sort"
)

func numSpecialEquivGroups(A []string) int {
	buf := make(map[string]int)
	for i := 0; i < len(A); i++ {
		var b0, b1 []int
		var s0, s1 []byte
		for j := 0; j < len(A[i]); j++ {
			if j%2 == 0 {
				b0 = append(b0, int(A[i][j]-'a'))
			} else {
				b1 = append(b1, int(A[i][j]-'a'))
			}
		}
		sort.Ints(b0)
		sort.Ints(b1)
		for i := 0; i < len(b0); i++ {
			s0 = append(s0, byte(b0[i]+'a'))
		}
		for i := 0; i < len(b1); i++ {
			s1 = append(s1, byte(b1[i]+'a'))
		}
		buf[string(s0)+string(s1)]++
	}
	return len(buf)
}

func main() {
	fmt.Println("a")
}
