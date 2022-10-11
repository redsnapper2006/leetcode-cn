package main

func countVowelSubstrings2(word string) int {
	ret := 0
	buf := make([][]int, len(word))
	start := -1
	for i := 0; i < len(word); i++ {
		buf[i] = make([]int, 5)
		b := byte(word[i])
		if b != 'a' && b != 'e' && b != 'i' && b != 'o' && b != 'u' {
			start = -1
			continue
		}
		if start == -1 {
			start = i
		}
		if i > 0 {
			copy(buf[i], buf[i-1])
		}
		idx := -1
		if byte(word[i]) == 'a' {
			idx = 0
		}
		if byte(word[i]) == 'e' {
			idx = 1
		}
		if byte(word[i]) == 'i' {
			idx = 2
		}
		if byte(word[i]) == 'o' {
			idx = 3
		}
		if byte(word[i]) == 'u' {
			idx = 4
		}
		buf[i][idx]++

		if i-start < 4 {
			continue
		}

		valid := true
		for j := 0; j < len(buf[i]); j++ {
			if buf[i][j] < 1 {
				valid = false
				break
			}
		}
		if valid {
			ret++
		}

		for m := start; m < i-5; m++ {
			valid = true
			for j := 0; j < len(buf[i]); j++ {
				if buf[i][j]-buf[m][j] < 1 {
					valid = false
					break
				}
			}
			if valid {
				ret++
			}
		}
	}
	return ret
}

func countVowelSubstrings(word string) int {
	ret := 0

	for ii := 0; ii < len(word); ii++ {
		a, e, i, o, u := 0, 0, 0, 0, 0
		for j := ii; j < len(word); j++ {
			b := byte(word[j])
			if b == 'a' {
				a++
			} else if b == 'e' {
				e++
			} else if b == 'i' {
				i++
			} else if b == 'o' {
				o++
			} else if b == 'u' {
				u++
			} else {
				break
			}

			if a > 0 && e > 0 && i > 0 && o > 0 && u > 0 {
				ret++
			}
		}

	}
	return ret
}
