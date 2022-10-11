package main

func prefixCount(words []string, pref string) int {
	ret := 0
	n := len(pref)
	for _, w := range words {
		if len(w) >= n && w[:n] == pref {
			ret++
		}
	}
	return ret
}
