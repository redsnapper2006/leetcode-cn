package main

import "fmt"

func isDecomposable(s string) bool {
	two := 0
	cnt := 0
	base := byte('a')
	for _, b := range s {
		if byte(b) != base {
			if cnt > 0 {
				if cnt%3 == 1 {
					return false
				} else if cnt%3 == 2 {
					two++
				}
			}
			base = byte(b)
			cnt = 1
		} else {
			cnt++
		}
	}

	if cnt%3 == 1 {
		return false
	} else if cnt%3 == 2 {
		two++
	}
	return two == 1
}

func main() {
	fmt.Println()
}
