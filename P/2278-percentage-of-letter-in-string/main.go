package main

func percentageLetter(s string, letter byte) int {
	cnt := 0

	for _, b := range s {
		if byte(b) == letter {
			cnt++
		}
	}

	return cnt * 100 / len(s)
}
