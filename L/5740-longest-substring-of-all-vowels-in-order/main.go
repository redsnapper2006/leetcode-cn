package main

import "fmt"

func longestBeautifulSubstring(word string) int {
	s := 0
	for s < len(word) {
		if word[s] == 'a' {
			break
		}
		s++
	}
	start := s

	ret := 0
	state := byte('a')
	for s < len(word) {
		// fmt.Println(s, string(state), string(word[s]))
		if word[s] == state {
			s++
			continue
		}
		if state == byte('a') && word[s] == byte('e') {
			state = byte('e')
			s++
			continue
		} else if state == byte('e') && word[s] == byte('i') {
			state = byte('i')
			s++
			continue
		} else if state == byte('i') && word[s] == byte('o') {
			state = byte('o')
			s++
			continue
		} else if state == byte('o') && word[s] == byte('u') {
			state = byte('u')
			s++
			continue
		} else if state == byte('u') {
			if ret < s-start {
				ret = s - start
			}
		}
		for s < len(word) {
			if word[s] == 'a' {
				start = s
				state = byte('a')
				break
			}
			s++
		}
	}
	if state == byte('u') {
		if ret < len(word)-start {
			ret = len(word) - start
		}
	}
	return ret
}

func main() {
	fmt.Println(longestBeautifulSubstring("aeiaaioaaaaeiiiiouuuooaauuaeiu"))
}
