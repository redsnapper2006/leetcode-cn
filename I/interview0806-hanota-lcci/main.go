package main

func hanota(A []int, B []int, C []int) []int {
	var move func(n int, A, B, C *[]int)
	move = func(n int, A, B, C *[]int) {
		if n == 1 {
			*C = append(*C, (*A)[len(*A)-1])
			*A = (*A)[0 : len(*A)-1]
			return
		}
		move(n-1, A, C, B)
		move(1, A, B, C)
		move(n-1, B, A, C)
	}
	move(len(A), &A, &B, &C)
	return C
}
