package main

import (
	"fmt"
)

func isInterleave(s1 string, s2 string, s3 string) bool {
	if len(s1)+len(s2) != len(s3) {
		return false
	}
	M12 := make(map[byte]int)
	M3 := make(map[byte]int)
	for i := 0; i < len(s1); i++ {
		M12[s1[i]]++
	}
	for i := 0; i < len(s2); i++ {
		M12[s2[i]]++
	}
	for i := 0; i < len(s3); i++ {
		M3[s3[i]]++
	}

	for k, v := range M12 {
		v2, ok := M3[k]
		if !ok || v2 != v {
			return false
		}
	}

	buf := make([][]bool, len(s1)+1)
	for i := 0; i <= len(s1); i++ {
		buf[i] = make([]bool, len(s2)+1)
	}

	// fmt.Println(buf)
	buf[0][0] = true
	for i := 1; i <= len(s2); i++ {
		buf[0][i] = s2[i-1] == s3[i-1]
	}
	for i := 1; i <= len(s1); i++ {
		buf[i][0] = s1[i-1] == s3[i-1]
	}

	for i := 1; i <= len(s1); i++ {
		for j := 1; j <= len(s2); j++ {
			buf[i][j] = (buf[i-1][j] && s1[i-1] == s3[i+j-1]) || (buf[i][j-1] && s2[j-1] == s3[i+j-1])
		}
	}

	return buf[len(s1)][len(s2)]
}

func isInterleaveV2(s1 string, s2 string, s3 string) bool {
	if len(s1)+len(s2) != len(s3) {
		return false
	}
	M12 := make(map[byte]int)
	M3 := make(map[byte]int)
	for i := 0; i < len(s1); i++ {
		M12[s1[i]]++
	}
	for i := 0; i < len(s2); i++ {
		M12[s2[i]]++
	}
	for i := 0; i < len(s3); i++ {
		M3[s3[i]]++
	}

	for k, v := range M12 {
		v2, ok := M3[k]
		if !ok || v2 != v {
			return false
		}
	}

	var recur func(s1, s2, s3 string) bool
	recur = func(s1, s2, s3 string) bool {
		if len(s1) == 0 {
			return s2 == s3
		}
		if len(s2) == 0 {
			return s1 == s3
		}
		if s1[0] != s3[0] && s2[0] != s3[0] {
			return false
		} else if s1[0] == s3[0] && s2[0] != s3[0] {
			return recur(s1[1:], s2, s3[1:])
		} else if s1[0] != s3[0] && s2[0] == s3[0] {
			return recur(s1, s2[1:], s3[1:])
		} else {
			return recur(s1[1:], s2, s3[1:]) || recur(s1, s2[1:], s3[1:])
		}
	}
	return recur(s1, s2, s3)
}
func main() {
	fmt.Println(isInterleave("aabcc", "dbbca", "aadbbcbcac"))
	fmt.Println(isInterleave("aabcc", "dbbca", "aadbbbaccc"))
	fmt.Println(isInterleave("aabc", "abad", "aabcabad"))
}
