package main

import (
	"fmt"
	"sort"
)

func findEvenNumbers(digits []int) []int {
	numMap := map[int]int{}
	evenMap := map[int]int{}
	for _, d := range digits {
		numMap[d]++
		if d%2 == 0 {
			evenMap[d]++
		}
	}

	ret := []int{}
	ks := []int{}
	for k := range numMap {
		ks = append(ks, k)
	}
	// sort.Ints(ks)
	for ed := range evenMap {
		numMap[ed]--
		for i := 0; i < len(ks); i++ {
			if ks[i] == 0 {
				continue
			}
			numMap[ks[i]]--
			for j := 0; j < len(ks); j++ {

				numMap[ks[j]]--
				if numMap[ed] >= 0 && numMap[ks[i]] >= 0 && numMap[ks[j]] >= 0 {
					ret = append(ret, ks[i]*100+ks[j]*10+ed)
				}

				numMap[ks[j]]++
			}

			numMap[ks[i]]++
		}
		numMap[ed]++
	}
	sort.Ints(ret)
	return ret
}

func main() {
	fmt.Println()
}
