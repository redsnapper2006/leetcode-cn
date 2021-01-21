package main

import "fmt"

func kLengthApart(nums []int, k int) bool {
	pIdx := 0
	for pIdx < len(nums) {
		if nums[pIdx] == 0 {
			pIdx++
			continue
		}

		nIdx := -1
		for j := pIdx + 1; j < len(nums); j++ {
			if nums[j] == 1 {
				nIdx = j
				break
			}
		}
		fmt.Println(pIdx, nIdx)
		if nIdx != -1 {
			if nIdx-pIdx <= k {
				return false
			}
		} else {
			break
		}
		pIdx = nIdx
	}

	return true
}

func main() {
	fmt.Println()
}
