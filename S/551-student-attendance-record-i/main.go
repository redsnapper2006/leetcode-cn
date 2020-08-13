package main

import "fmt"

func checkRecord(s string) bool {
	A := 0
	L := 0
	isAward := true
	for i := 0; i < len(s); i++ {
		if s[i] == 'A' {
			A++
			if A > 1 {
				isAward = false
				break
			}
		} else if s[i] == 'L' {
			L++
			if L == 3 {
				if s[i-1] == 'L' && s[i-2] == 'L' {
					isAward = false
					break
				} else {
					if s[i-1] == 'L' {
						L = 2
					} else {
						L = 1
					}
				}
			}
		}
	}
	return isAward
}

func main() {
	fmt.Println("a")
}
