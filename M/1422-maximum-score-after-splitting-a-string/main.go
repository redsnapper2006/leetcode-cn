package main

import "fmt"

func maxScore(s string) int {
	one := 0
	for i := 0; i < len(s); i++ {
		if s[i] == '1' {
			one++
		}
	}

	max := -1
	zero := 0
	for i := 1; i < len(s); i++ {
		if s[i-1] == '0' {
			zero++
		} else {
			one--
		}

		if max < zero+one {
			max = zero + one
		}
	}

	return max
}

func main() {
	fmt.Println("a")
}
