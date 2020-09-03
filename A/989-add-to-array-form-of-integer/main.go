package main

import "fmt"

func addToArrayForm(A []int, K int) []int {
	var buf []int
	for K > 0 {
		m := K % 10
		buf = append(buf, m)
		K /= 10
	}

	s, e := 0, len(A)-1
	for s < e {
		A[s], A[e] = A[e], A[s]
		s++
		e--
	}

	aIdx, kIdx := 0, 0
	isPlus := false
	var ret []int
	for aIdx < len(A) || kIdx < len(buf) {
		a := 0
		if aIdx < len(A) {
			a = A[aIdx]
		}
		k := 0
		if kIdx < len(buf) {
			k = buf[kIdx]
		}
		r := a + k
		if isPlus {
			r++
		}
		if r >= 10 {
			r -= 10
			isPlus = true
		} else {
			isPlus = false
		}
		aIdx++
		kIdx++
		ret = append(ret, r)
	}
	if isPlus {
		ret = append(ret, 1)
	}
	s, e = 0, len(ret)-1
	for s < e {
		ret[s], ret[e] = ret[e], ret[s]
		s++
		e--
	}

	return ret
}

func main() {
	fmt.Println("a")
}
