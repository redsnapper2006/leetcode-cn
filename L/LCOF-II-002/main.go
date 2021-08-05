package main

import "fmt"

func addBinary(a string, b string) string {
	bufa, bufb := []byte(a), []byte(b)
	max := len(bufa)
	if max < len(bufb) {
		max = len(bufb)
	}
	ret := []byte{}
	isExtra := false
	for i := 0; i < max; i++ {
		ca := 0
		if len(bufa)-1-i >= 0 {
			ca = int(bufa[len(bufa)-1-i] - '0')
		}
		cb := 0
		if len(bufb)-1-i >= 0 {
			cb = int(bufb[len(bufb)-1-i] - '0')
		}
		v := ca + cb
		if isExtra {
			v++
		}
		if v >= 2 {
			v -= 2
			isExtra = true
		} else {
			isExtra = false
		}
		ret = append(ret, byte(v+'0'))
	}

	if isExtra {
		ret = append(ret, byte('1'))
	}
	// fmt.Println(ret)
	s, e := 0, len(ret)-1
	for s < e {
		ret[s], ret[e] = ret[e], ret[s]
		s++
		e--
	}
	return string(ret)
}

func main() {
	fmt.Println()
}
