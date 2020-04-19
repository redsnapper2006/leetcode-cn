package main

import (
	"fmt"
)

func getMaxRepetitions(s1 string, n1 int, s2 string, n2 int) int {
	if n1 == 0 {
		return 0
	}
	s1cnt, index, s2cnt := 0, 0, 0
	recall := make(map[int][]int)
	var pre_loop, in_loop []int
	for {
		s1cnt++
		for i := 0; i < len(s1); i++ {
			if s1[i] == s2[index] {
				index++
				if index == len(s2) {
					s2cnt++
					index = 0
				}
			}
		}
		if s1cnt == n1 {
			return s2cnt / n2
		}
		_, ok := recall[index]
		if !ok {
			recall[index] = []int{s1cnt, s2cnt}
		} else {
			prime := recall[index]
			s1cnt_prime, s2cnt_prime := prime[0], prime[1]
			pre_loop = []int{s1cnt_prime, s2cnt_prime}
			in_loop = []int{s1cnt - s1cnt_prime, s2cnt - s2cnt_prime}
			break
		}
	}
	ans := pre_loop[1] + (n1-pre_loop[0])/in_loop[0]*in_loop[1]
	rest := (n1 - pre_loop[0]) % in_loop[0]
	for i := 0; i < rest; i++ {
		for j := 0; j < len(s1); j++ {
			if s1[j] == s2[index] {
				index++
				if index == len(s2) {
					ans++
					index = 0
				}
			}
		}
	}

	return ans / n2
}

func main() {
	// fmt.Println(getMaxRepetitions("acb", 4, "ab", 2))
	// fmt.Println(getMaxRepetitions("aaa", 3, "aa", 1))
	fmt.Println(getMaxRepetitions("baba", 11, "baab", 1))
}
