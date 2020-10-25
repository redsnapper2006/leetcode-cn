package main

import "fmt"

func longestMountain(A []int) int {
	asc, desc := map[int]int{}, map[int]int{}

	sIdx := 1
	for sIdx < len(A) {
		if A[sIdx] > A[sIdx-1] {
			s := sIdx - 1
			for sIdx < len(A) {
				if A[sIdx] > A[sIdx-1] {
					sIdx++
				} else {
					break
				}
			}
			if s != sIdx-1 {
				asc[s] = sIdx - 1
			}
		} else if A[sIdx] < A[sIdx-1] {
			ss := sIdx - 1
			for sIdx < len(A) {
				if A[sIdx] < A[sIdx-1] {
					sIdx++
				} else {
					break
				}
			}
			if ss != sIdx-1 {
				desc[ss] = sIdx - 1
			}
		} else {
			sIdx++
		}
	}
	max := 0
	for i := 0; i < len(A); i++ {
		sm, ok := asc[i]
		if !ok {
			continue
		}
		end, ok := desc[sm]
		if !ok {
			continue
		}
		if end-i > max {
			max = end - i
		}
	}
	if max == 0 {
		return max
	}
	return max + 1
}

func main() {
	fmt.Println("a")
}
