package main

import "sort"

func numMatchingSubseq(s string, words []string) int {
	idxs := make([][]int, 26)
	for i, b := range s {
		offset := int(byte(b) - 'a')
		if len(idxs[offset]) == 0 {
			idxs[offset] = []int{}
		}
		idxs[offset] = append(idxs[offset], i)
	}
	ret := 0
	for _, word := range words {
		sIdx := -1
		isF := true
		for _, b := range word {
			j := sort.SearchInts(idxs[b-'a'], sIdx+1)
			if j == len(idxs[b-'a']) {
				isF = false
				break
			}
			sIdx = idxs[b-'a'][j]
		}

		if isF {
			ret++
		}
	}
	return ret
}
