package main

func countSubstrings(s string) int {
	f := func(s string, l, r int) int {
		cnt := 0

		for l >= 0 && r < len(s) {
			if s[l] != s[r] {
				break
			}
			cnt++
			l--
			r++
		}
		return cnt
	}

	ret := 0
	for i := 0; i < len(s); i++ {
		ret += f(s, i)
	}

	for i := 0; i < len(s)-1; i++ {
		ret += f(s, i, i+1)
	}
	return ret
}
