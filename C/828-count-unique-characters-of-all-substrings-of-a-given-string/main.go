package main

import "fmt"

func uniqueLetterString(s string) int {
	ret := 0
	buf := make([][]int, 26)
	for i := 0; i < 26; i++ {
		buf[i] = []int{-1}
	}
	for i := 0; i < len(s); i++ {
		offset := byte(s[i]) - 'A'
		buf[offset] = append(buf[offset], i)
	}
	for i := 0; i < 26; i++ {
		buf[i] = append(buf[i], len(s))
	}

	for i := 0; i < 26; i++ {
		target := buf[i]
		if len(target) <= 2 {
			continue
		}

		for j := 1; j < len(target)-1; j++ {
			ret += (target[j] - target[j-1]) * (target[j+1] - target[j])
		}
	}

	return ret
}

func main() {
	fmt.Println(uniqueLetterString("LEETCODE"))
	fmt.Println(uniqueLetterString("ABC"))
	fmt.Println(uniqueLetterString("ABA"))
	fmt.Println(uniqueLetterString("AAAB"))
}
