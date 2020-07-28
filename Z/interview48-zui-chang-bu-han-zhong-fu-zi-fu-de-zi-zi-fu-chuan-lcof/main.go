package main

import "fmt"

func lengthOfLongestSubstring(s string) int {

	start := 0
	ret := 0
	m := make(map[int]int)
	for i, v := range s {
		o, ok := m[int(v)]
		if !ok {
			m[int(v)] = i
		} else {
			if start <= o {
				if i-start > ret {
					ret = i - start
				}
				start = o + 1
			}
			m[int(v)] = i
		}
	}

	if len(s)-start > ret {
		return len(s) - start
	}
	return ret
}

func main() {
	fmt.Println("a")
}
