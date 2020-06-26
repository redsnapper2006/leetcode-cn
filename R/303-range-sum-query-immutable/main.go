package main

import "fmt"

type NumArray struct {
	Sum []int
}

func Constructor(nums []int) NumArray {
	sum := make([]int, len(nums))
	s := 0
	for i := 0; i < len(nums); i++ {
		s += nums[i]
		sum[i] = s
	}
	return NumArray{Sum: sum}
}

func (this *NumArray) SumRange(i int, j int) int {
	b := 0
	if i > 0 {
		b = this.Sum[i-1]
	}
	return this.Sum[j] - b
}

func main() {
	fmt.Println("a")
}
