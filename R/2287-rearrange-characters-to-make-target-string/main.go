package main

func rearrangeCharacters(s string, target string) int {
	cntArr := make([]int, 26)
	for _, b := range s {
		cntArr[byte(b)-'a']++
	}

	tArr := make([]int, 26)
	for _, b := range target {
		tArr[byte(b)-'a']++
	}

	max := len(s)
	for i := 0; i < 26; i++ {
		if tArr[i] == 0 {
			continue
		}
		if max > cntArr[i]/tArr[i] {
			max = cntArr[i] / tArr[i]
		}
	}
	return max
}
