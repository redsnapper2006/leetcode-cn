package main

import "fmt"

func maxScoreSightseeingPair(A []int) int {

	I := 0
	max := -1 << 31
	for i := 0; i < len(A); i++ {
		s := A[i] - i + I
		if s > max {
			max = s
		}
		if A[i]+i > I {
			I = A[i] + i
		}
	}
	return max
}

func maxScoreSightseeingPairV2(A []int) int {
	I := make([]int, len(A))
	J := make([]int, len(A))
	for i := 0; i < len(A); i++ {
		I[i] = A[i] + i
		J[i] = A[i] - i
	}

	var recur func(I, J []int) int
	recur = func(I, J []int) int {
		if len(I) <= 1 {
			return -1 << 31
		}
		if len(I) == 2 {
			return I[0] + J[1]
		}

		max := -1 << 31
		jIdx := -1
		for i := 1; i < len(J); i++ {
			if J[i] >= max {
				max = J[i]
				jIdx = i
			}
		}
		max = -1 << 31
		iIdx := -1
		for i := 0; i < jIdx; i++ {
			if I[i] >= max {
				max = I[i]
				iIdx = i
			}
		}
		r := I[iIdx] + J[jIdx]
		left := recur(I[jIdx:], J[jIdx:])
		if r > left {
			return r
		}
		return left
	}
	return recur(I, J)
}

func main() {
	fmt.Println("a")
}
