package main

func repeatedCharacter(s string) byte {
	buf := make([]int, 26)
	for _, b := range s {
		offset := byte(b) - 'a'
		if buf[offset] == 1 {
			return byte(b)
		}
		buf[offset]++
	}
	return '0'
}
