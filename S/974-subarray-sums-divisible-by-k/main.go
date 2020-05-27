package main

import "fmt"

func subarraysDivByK(A []int, K int) int {
	MOD := map[int]int{}
	MOD[0] = 1
	c := 0
	sum := 0
	for i := 0; i < len(A); i++ {
		sum += A[i]
		m := sum % K
		if m < 0 {
			m = K + m
		}
		c += MOD[m]
		MOD[m]++
	}
	return c
}

func main() {
	fmt.Println(subarraysDivByK([]int{2, -2, 2, -4}, 6))
	fmt.Println(subarraysDivByK([]int{-1, 2, 9}, 2))

}
