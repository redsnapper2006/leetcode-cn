package main

import "fmt"

func maximumRemovals(s string, p string, removable []int) int {
	bs := []byte(s)
	bp := []byte(p)

	rs, re := 0, len(removable)-1
	for rs <= re {
		rm := rs + (re-rs)/2
		buf := make([]int, len(s))
		for i := 0; i <= rm; i++ {
			buf[removable[i]] = 1
		}

		ss, sp := 0, 0
		for ss < len(bs) && sp < len(bp) {
			if buf[ss] == 1 {
				ss++
				continue
			}
			if bs[ss] == bp[sp] {
				ss++
				sp++
			} else {
				ss++
			}
		}
		if sp != len(bp) {
			re = rm - 1
		} else {
			rs = rm + 1
		}
	}
	return rs
}

func main() {
	fmt.Println(maximumRemovals("abcacb", "ab", []int{3, 1, 0}))
	fmt.Println(maximumRemovals("abcab", "abc", []int{0, 1, 2, 3, 4}))
}
