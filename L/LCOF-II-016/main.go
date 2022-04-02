package main

import "fmt"

func lengthOfLongestSubstring(s string) int {
	m := map[byte]int{}

	ret := -1
	idx := 0
	prev := -1
	for idx < len(s) {
		v, ok := m[s[idx]]
		if ok && v > prev {
			prev = v
		}

		if idx-prev > ret {
			ret = idx - prev
		}
		m[s[idx]] = idx
		idx++
	}
	if ret == -1 {
		return len(s)
	}
	return ret
}

func main() {
	fmt.Println(lengthOfLongestSubstring("abba"))
}
