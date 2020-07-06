package main

import (
	"fmt"
	"sort"
)

func permutation(S string) []string {
	intBuf := make([]int, len(S))
	for i := 0; i < len(S); i++ {
		intBuf[i] = int(S[i])
	}
	sort.Ints(intBuf)
	buf := make([]byte, len(S))
	for i := 0; i < len(intBuf); i++ {
		buf[i] = byte(intBuf[i])
	}

	var recur func(buf []byte) [][]byte
	recur = func(buf []byte) [][]byte {
		if len(buf) == 1 {
			return [][]byte{buf}
		}
		var ret [][]byte
		for i := 0; i < len(buf); i++ {
			if i > 0 && buf[i] == buf[i-1] {
				continue
			}
			b := buf[i]
			tbuf := make([]byte, len(buf)-1)
			copy(tbuf, buf[0:i])
			copy(tbuf[i:], buf[i+1:])
			sub := recur(tbuf)
			for j := 0; j < len(sub); j++ {
				t := make([]byte, len(sub[j])+1)
				copy(t, sub[j])
				t[len(t)-1] = b
				ret = append(ret, t)
			}
		}
		return ret
	}

	r := recur(buf)
	var rs []string
	for i := 0; i < len(r); i++ {
		rs = append(rs, string(r[i]))
	}
	return rs
}

func main() {
	fmt.Println("a")
}
