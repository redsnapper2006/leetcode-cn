package main

import "fmt"

func sortArrayByParityII(A []int) []int {

	var ODD, EVEN []int
	for _, v := range A {
		if v%2 == 0 {
			EVEN = append(EVEN, v)
		} else {
			ODD = append(ODD, v)
		}
	}
	sO, sE := 0, 0
	for i := 0; i < len(A); i++ {
		if i%2 == 0 {
			A[i] = EVEN[sE]
			sE++
		} else {
			A[i] = ODD[sO]
			sO++
		}
	}
	return A
}

func main() {
	fmt.Println("a")
}
