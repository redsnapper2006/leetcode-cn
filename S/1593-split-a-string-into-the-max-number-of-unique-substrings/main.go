package main

import "fmt"

func maxUniqueSplit(s string) int {
	m := map[string]int{}

	max := 0

	var dfs func(s string, m map[string]int)
	dfs = func(s string, m map[string]int) {
		if len(s)+len(m) <= max {
			return
		}
		if len(s) == 0 {
			if len(m) > max {
				max = len(m)
			}
			return
		}
		for i := 1; i <= len(s); i++ {
			_, ok := m[s[0:i]]
			if ok {
				continue
			}
			m[s[0:i]] = 1
			dfs(s[i:], m)
			delete(m, s[0:i])
		}
	}
	dfs(s, m)
	return max
}

func main() {
	fmt.Println()
}
