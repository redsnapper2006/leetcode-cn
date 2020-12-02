package main

import "fmt"

func pancakeSort(A []int) []int {
	doneIdx := len(A)
	var ret []int
	for doneIdx >= 0 {
		max := -1
		targetIdx := -1
		for i := 0; i < doneIdx; i++ {
			if max < A[i] {
				targetIdx = i
				max = A[i]
			}
		}

		if targetIdx == doneIdx-1 {
			doneIdx--
			continue
		}
		ret = append(ret, targetIdx+1)
		s, e := 0, targetIdx
		for s < e {
			A[s], A[e] = A[e], A[s]
			s++
			e--
		}
		s, e = 0, doneIdx-1
		for s < e {
			A[s], A[e] = A[e], A[s]
			s++
			e--
		}
		ret = append(ret, doneIdx)
		doneIdx--

	}
	return ret
}

func main() {
	fmt.Println()
}
