package main

import (
	"fmt"
	"sort"
)

func permutation(s string) []string {
	var buf []int
	for i := 0; i < len(s); i++ {
		buf = append(buf, int(s[i]-'a'))
	}
	sort.Ints(buf)

	var permute func(buf []int) [][]int
	permute = func(buf []int) [][]int {
		if len(buf) == 1 {
			return [][]int{buf}
		}
		var ret [][]int
		for i := 0; i < len(buf); i++ {
			if i > 0 && buf[i] == buf[i-1] {
				continue
			}
			tt := make([]int, len(buf)-1)
			copy(tt, buf[0:i])
			copy(tt[i:], buf[i+1:])
			r := permute(tt)
			for _, v := range r {
				ret = append(ret, append(v, buf[i]))
			}
		}
		return ret
	}
	var r []string
	ret := permute(buf)
	for i := 0; i < len(ret); i++ {
		var b []byte
		for j := 0; j < len(ret[i]); j++ {
			b = append(b, byte(ret[i][j]+'a'))
		}
		r = append(r, string(b))
	}
	return r
}

func main() {
	fmt.Println("a")
}
