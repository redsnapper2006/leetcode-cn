package main

import (
	"fmt"
	"sort"
)

func numSmallerByFrequency(queries []string, words []string) []int {
	qbuf := make([]int, len(queries))
	for i, v := range queries {
		buf := make([]int, 26)
		for _, b := range v {
			buf[b-'a']++
		}
		for _, c := range buf {
			if c > 0 {
				qbuf[i] = c
				break
			}
		}
	}
	wbuf := make([]int, len(words))
	for i, v := range words {
		buf := make([]int, 26)
		for _, b := range v {
			buf[b-'a']++
		}
		for _, c := range buf {
			if c > 0 {
				wbuf[i] = c
				break
			}
		}
	}
	sort.Ints(wbuf)
	var ret []int
	for _, b := range qbuf {
		s, e := 0, len(wbuf)-1
		for s <= e {
			m := s + (e-s)/2
			if wbuf[m] <= b {
				s = m + 1
			} else if wbuf[m] > b {
				e = m - 1
			}
		}
		ret = append(ret, len(wbuf)-s)
	}
	return ret
}

func main() {
	fmt.Println("a")
}
