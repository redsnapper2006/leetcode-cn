package main

func validWordAbbreviation(word string, abbr string) bool {

	cnt := 0
	idx := 0
	for i := 0; i < len(abbr); i++ {
		if abbr[i] >= byte('0') && abbr[i] <= byte('9') {
			if abbr[i] == byte('0') && cnt == 0 {
				return false
			}
			cnt = cnt*10 + int(abbr[i]-byte('0'))
		} else {
			idx += cnt
			cnt = 0
			if word[idx] != abbr[i] {
				return false
			}
			idx++
		}
	}
	return len(word)-idx == cnt
}
