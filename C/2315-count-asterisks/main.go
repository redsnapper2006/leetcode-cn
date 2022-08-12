package main

func countAsterisks(s string) int {
	cnt := 0
	pipecnt := 0

	for _, b := range s {
		if byte(b) == '*' && pipecnt%2 == 0 {
			cnt++
		}
		if byte(b) == '|' {
			pipecnt++
		}
	}
	return cnt
}
