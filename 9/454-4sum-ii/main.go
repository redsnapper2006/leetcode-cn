package main

import (
	"fmt"
)

func fourSumCount(A []int, B []int, C []int, D []int) int {
	mA := make(map[int]int)
	mB := make(map[int]int)
	mC := make(map[int]int)
	mD := make(map[int]int)

	for _, v := range A {
		mA[v]++
	}

	for _, v := range B {
		for k, va := range mA {
			mB[v+k] += va
		}
	}

	for _, v := range C {
		mC[v]++
	}

	for _, v := range D {
		for k, vc := range mC {
			mD[v+k] += vc
		}
	}
	accum := 0
	for k, vAB := range mB {
		vCD, ok := mD[k*-1]
		if ok {
			accum += vAB * vCD
		}
	}

	return accum
}

func main() {
	fmt.Println(fourSumCount([]int{1, 2}, []int{-2, -1}, []int{-1, 2}, []int{0, 2}))
}
