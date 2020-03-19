package main

import (
	"fmt"
)

func letterCombinations(digits string) []string {
	if len(digits) == 0 {
		return []string{}
	}
	M := map[byte][]byte{
		'2': []byte{'a', 'b', 'c'},
		'3': []byte{'d', 'e', 'f'},
		'4': []byte{'g', 'h', 'i'},
		'5': []byte{'j', 'k', 'l'},
		'6': []byte{'m', 'n', 'o'},
		'7': []byte{'p', 'q', 'r', 's'},
		'8': []byte{'t', 'u', 'v'},
		'9': []byte{'w', 'x', 'y', 'z'},
	}

	var recur func(digits string) [][]byte
	recur = func(digits string) [][]byte {
		var r [][]byte
		bb := M[digits[0]]

		if len(digits) == 1 {
			for i := 0; i < len(bb); i++ {
				r = append(r, []byte{bb[i]})
			}
			return r
		}

		rr := recur(digits[1:])
		for i := 0; i < len(bb); i++ {
			for j := 0; j < len(rr); j++ {
				t := make([]byte, len(digits))
				copy(t, rr[j])
				t[len(t)-1] = bb[i]
				r = append(r, t)
			}
		}
		return r
	}

	ds := []byte(digits)
	s, e := 0, len(ds)-1
	for s < e {
		ds[s], ds[e] = ds[e], ds[s]
		s++
		e--
	}

	rr := recur(string(ds))

	var ret []string
	for i := 0; i < len(rr); i++ {
		ret = append(ret, string(rr[i]))
	}
	return ret
}

func main() {
	fmt.Println(letterCombinations("23"))
}
