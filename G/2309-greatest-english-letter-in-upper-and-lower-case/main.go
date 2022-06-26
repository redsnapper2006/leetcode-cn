package main

func greatestLetter(s string) string {
	upper, lower := [26]int{}, [26]int{}
	for _, b := range s {
		if byte(b) >= 'A' && byte(b) <= 'Z' {
			upper[int(byte(b)-'A')] = 1
		}
		if byte(b) >= 'a' && byte(b) <= 'z' {
			lower[int(byte(b)-'a')] = 1
		}
	}
	for i := 25; i >= 0; i-- {
		if upper[i] > 0 && lower[i] > 0 {
			return string([]byte{byte('A' + i)})
		}
	}
	return ""
}
