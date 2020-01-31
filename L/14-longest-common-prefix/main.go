package main

import (
	"fmt"
)

func longestCommonPrefix(strs []string) string {
	if len(strs) == 0 {
		return ""
	}
	if len(strs) == 1 {
		return strs[0]
	}

	key := strs[0]
	offset := len(key)
	for i := 1; i < len(strs); i++ {
		if offset > len(strs[i]) {
			offset = len(strs[i])
		}
		for j := 0; j < offset; j++ {
			if key[j] != strs[i][j] {
				offset = j
				break
			}
		}
	}
	return key[0:offset]
}

func main() {
	fmt.Println(longestCommonPrefix([]string{"flower", "flow", "flight"}))
	fmt.Println(longestCommonPrefix([]string{"dog", "racecar", "car"}))
}
