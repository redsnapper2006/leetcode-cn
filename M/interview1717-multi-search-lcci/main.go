package main

import (
	"fmt"
	"strings"
)

func multiSearch(big string, smalls []string) [][]int {
	var ret [][]int
	for i := 0; i < len(smalls); i++ {
		target := smalls[i]
		if target == "" {
			ret = append(ret, nil)
			continue
		}
		var t []int
		for j := 0; j < len(big)-len(target)+1; j++ {
			if big[j] == target[0] && strings.HasPrefix(big[j:], target) {
				t = append(t, j)
			}
		}
		ret = append(ret, t)
	}
	return ret
}

func main() {
	fmt.Println()
}
