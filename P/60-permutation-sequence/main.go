package main

import (
	"fmt"
	"strconv"
)

func getPermutation(n int, k int) string {
	if n == 1 && k == 1 {
		return "1"
	}
	B := make([]int, n-1)
	acc := 1
	for i := 1; i < n; i++ {
		acc *= i
		B[i-1] = acc
	}

	N := make([]int, n)
	for i := 1; i <= n; i++ {
		N[i-1] = i
	}

	k--
	var ret []int
	for i := len(B) - 1; i >= 1; i-- {
		d := k / B[i]
		k = k % B[i]
		ret = append(ret, N[d])
		N = append(N[0:d], N[d+1:]...)
	}
	if k == 0 {
		ret = append(ret, N[0], N[1])
	} else {
		ret = append(ret, N[1], N[0])
	}
	s := ""
	for i := 0; i < len(ret); i++ {
		s += strconv.FormatInt(int64(ret[i]), 10)
	}
	return s
}

func main() {
	fmt.Println(getPermutation(4, 9))
}
