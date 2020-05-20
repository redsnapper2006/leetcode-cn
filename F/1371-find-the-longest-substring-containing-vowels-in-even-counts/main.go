package main

import (
	"fmt"
)

func findTheLongestSubstring(s string) int {
	buf := make([]int, 32)
	for i := 0; i < 32; i++ {
		buf[i] = -1
	}
	buf[31] = -1
	length := 0
	as, es, is, os, us := true, true, true, true, true
	for i := 0; i < len(s); i++ {
		if s[i] == 'a' {
			as = !as
		}
		if s[i] == 'e' {
			es = !es
		}
		if s[i] == 'i' {
			is = !is
		}
		if s[i] == 'o' {
			os = !os
		}
		if s[i] == 'u' {
			us = !us
		}
		r := 0
		if as == true {
			r += 1
		}
		if es == true {
			r += 1 << 1
		}
		if is == true {
			r += 1 << 2
		}
		if os == true {
			r += 1 << 3
		}
		if us == true {
			r += 1 << 4
		}
		if buf[r] == -1 && r != 31 {
			buf[r] = i
		} else if i-buf[r] > length {
			length = i - buf[r]
		}
	}
	return length
}

func findTheLongestSubstringV2(s string) int {
	buf := make([][]bool, 5)
	for i := 0; i < len(buf); i++ {
		buf[i] = make([]bool, len(s)+1)
	}
	buf[0][0] = true
	buf[1][0] = true
	buf[2][0] = true
	buf[3][0] = true
	buf[4][0] = true

	for i := 0; i < len(s); i++ {
		if s[i] == 'a' {
			buf[0][i+1] = !buf[0][i]
		} else {
			buf[0][i+1] = buf[0][i]
		}
		if s[i] == 'e' {
			buf[1][i+1] = !buf[1][i]
		} else {
			buf[1][i+1] = buf[1][i]
		}
		if s[i] == 'i' {
			buf[2][i+1] = !buf[2][i]
		} else {
			buf[2][i+1] = buf[2][i]
		}
		if s[i] == 'o' {
			buf[3][i+1] = !buf[3][i]
		} else {
			buf[3][i+1] = buf[3][i]
		}
		if s[i] == 'u' {
			buf[4][i+1] = !buf[4][i]
		} else {
			buf[4][i+1] = buf[4][i]
		}
	}

	length := 0
	start := 0
	for start+length < len(s) {
		idx := -1
		for i := len(s) - 1; i >= start+length; i-- {
			if buf[0][i+1] && buf[1][i+1] && buf[2][i+1] && buf[3][i+1] && buf[4][i+1] {
				idx = i
				break
			}
		}
		if idx != -1 {
			length = idx - start + 1
		}

		next := -1
		for i := start; i < len(s); i++ {
			if s[i] != 'a' && s[i] != 'e' && s[i] != 'i' && s[i] != 'o' && s[i] != 'u' {
				continue
			}
			next = i
			break
		}
		arrIdx := -1
		if next == -1 {
			break
		}
		if s[next] == 'a' {
			arrIdx = 0
		}
		if s[next] == 'e' {
			arrIdx = 1
		}
		if s[next] == 'i' {
			arrIdx = 2
		}
		if s[next] == 'o' {
			arrIdx = 3
		}
		if s[next] == 'u' {
			arrIdx = 4
		}
		for i := idx; i < len(s); i++ {
			buf[arrIdx][i+1] = !buf[arrIdx][i+1]
		}
		start = next + 1
	}

	return length
}

func main() {
	// fmt.Println(findTheLongestSubstring("id"))
	// fmt.Println(findTheLongestSubstring("eleetminicoworoep"))
	fmt.Println(findTheLongestSubstring("leetcodeisgreat"))

}
