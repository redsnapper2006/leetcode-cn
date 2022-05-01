package main

func rearrangeArray(nums []int) []int {
	positive, negative := []int{}, []int{}
	for _, n := range nums {
		if n > 0 {
			positive = append(positive, n)
		} else {
			negative = append(negative, n)
		}
	}

	ret := []int{}
	for i := 0; i < len(positive); i++ {
		ret = append(ret, positive[i], negative[i])
	}

	return ret 
}
