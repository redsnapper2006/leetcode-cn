package main

func removeAnagrams(words []string) []string {
	ret := []string{words[0]}

	base := [26]int{}
	for _, b := range words[0] {
		base[int(byte(b)-'a')]++
	}

	for i := 1; i < len(words); i++ {
		cur := [26]int{}
		for _, b := range words[i] {
			cur[int(byte(b)-'a')]++
		}
		match := true
		for j := 0; j < 26; j++ {
			if base[j] != cur[j] {
				match = false
				break
			}
		}
		if !match {
			ret = append(ret, words[i])
			base = cur
		}
	}
	return ret
}
