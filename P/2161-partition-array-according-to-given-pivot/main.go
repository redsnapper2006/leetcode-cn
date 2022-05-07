package main

func pivotArray(nums []int, pivot int) []int {
	less, equal, more := []int{}, []int{}, []int{}

	for _, v := range nums {
		if v < pivot {
			less = append(less, v)
		} else if v > pivot {
			more = append(more, v)
		} else {
			equal = append(equal, v)
		}
	}
	return append(append(less, equal...), more...)
}
