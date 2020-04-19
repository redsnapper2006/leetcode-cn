package main

import "fmt"

func findAnagrams(s string, p string) []int {
	if len(s) < len(p) {
		return []int{}
	}
	M := make(map[byte]int)
	for i := 0; i < len(p); i++ {
		M[p[i]]++
	}

	T := make(map[byte]int)
	for i := 0; i < len(p); i++ {
		T[s[i]]++
	}
	isMatch := func(M, T map[byte]int) bool {
		for k, v := range M {
			v2, ok := T[k]
			if !ok || v != v2 {
				return false
			}
		}
		return true
	}
	var r []int
	if isMatch(M, T) {
		r = append(r, 0)
	}
	for i := len(p); i < len(s); i++ {
		T[s[i-len(p)]]--
		T[s[i]]++
		if isMatch(M, T) {
			// fmt.Println(i, len(p))
			r = append(r, i-len(p)+1)
		}
	}
	return r
}

func main() {
	fmt.Println(findAnagrams("cbaebabacd", "abc"))
}
