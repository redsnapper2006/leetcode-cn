package main

import "fmt"

type FindSumPairs struct {
	M1 map[int]int
	M2 map[int]int
	N2 []int
}

func Constructor(nums1 []int, nums2 []int) FindSumPairs {
	m1, m2 := map[int]int{}, map[int]int{}
	for _, v := range nums1 {
		m1[v]++
	}
	for _, v := range nums2 {
		m2[v]++
	}
	return FindSumPairs{M1: m1, M2: m2, N2: nums2}
}

func (this *FindSumPairs) Add(index int, val int) {
	o := this.N2[index]
	this.M2[o]--
	this.N2[index] += val
	this.M2[this.N2[index]]++
}

func (this *FindSumPairs) Count(tot int) int {
	ret := 0
	for k, v := range this.M1 {
		v2, ok := this.M2[tot-k]
		if ok {
			ret += v * v2
		}
	}
	return ret
}

func main() {
	fmt.Println()
}
