package main

import "fmt"

func numOfPairs(nums []string, target string) int {
	m := map[string]int{}
	for _, v := range nums {
		m[v]++
	}

	cnt := 0
	for _, v := range nums {
		if len(v) >= len(target) {
			continue
		}
		isMatch := true
		for i, b := range v {
			if byte(b) != byte(target[i]) {
				isMatch = false
				break
			}
		}
		if isMatch {
			remain := target[len(v):]
			c := m[remain]
			if remain == v {
				c--
			}
			cnt += c
		}
	}
	return cnt
}

func main() {
	fmt.Println()
}
