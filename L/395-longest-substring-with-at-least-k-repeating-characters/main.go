package main

import (
	"fmt"
)

func isMatch(table [][]int, k, size, index int) int {
	for i := size - 1; i >= index; i-- {
		isMatch := true
		for j := 0; j < 26; j++ {
			if table[i][j] > 0 && table[i][j] < k {
				isMatch = false
			}
		}
		if isMatch {
			return i
		}
	}
	return -1
}

func longestSubstring(s string, k int) int {
	size := len(s)
	table := make([][]int, size)
	for i := 0; i < size; i++ {
		table[i] = make([]int, 26)
	}

	t := make([]int, 26)
	for i := 0; i < size; i++ {
		t[s[i]-'a']++
		for j := 0; j < 26; j++ {
			table[i][j] = t[j]
		}
	}

	maxIndex := isMatch(table, k, size, 0)
	if maxIndex == size-1 {
		return maxIndex + 1
	}

	max := maxIndex + 1
	j := 1
	for maxIndex != size-1 {

		for m := j; m < size; m++ {
			table[m][s[j-1]-'a']--
		}
		maxIndex = isMatch(table, k, size, j)
		if max < maxIndex-j+1 {
			max = maxIndex - j + 1
		}
		if max >= size-j {
			break
		}
		j++
	}
	// fmt.Println(table, max)

	return max
}

func main() {
	fmt.Println(longestSubstring("saaabb", 2))
	fmt.Println(longestSubstring("ababbba", 2))
	fmt.Println(longestSubstring("ababbbaaaababbbaaaababbbaaaababbbaaa", 1))
}
