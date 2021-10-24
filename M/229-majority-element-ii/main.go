package main

import "fmt"

func majorityElement(nums []int) []int {
	vote1, vote2 := 0, 0
	candi1, candi2 := -1000000001, -1000000001
	for _, v := range nums {
		if vote1 > 0 && v == candi1 {
			vote1++
		} else if vote2 > 0 && v == candi2 {
			vote2++
		} else if vote1 == 0 {
			vote1++
			candi1 = v
		} else if vote2 == 0 {
			vote2++
			candi2 = v
		} else {
			vote1--
			vote2--
		}
	}

	nvote1, nvote2 := 0, 0
	for _, v := range nums {
		if v == candi1 && vote1 > 0 {
			nvote1++
		}
		if v == candi2 && vote2 > 0 {
			nvote2++
		}
	}
	ret := []int{}
	if nvote1 > len(nums)/3 && candi1 != -1000000001 {
		ret = append(ret, candi1)
	}
	if nvote2 > len(nums)/3 && candi2 != -1000000001 {
		ret = append(ret, candi2)
	}
	return ret
}

func main() {
	fmt.Println()
}
