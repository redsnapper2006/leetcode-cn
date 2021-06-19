package main

import (
	"fmt"
	"sort"
)

func halfQuestions(questions []int) int {
	buf := map[int]int{}
	for i := 0; i < len(questions); i++ {
		buf[questions[i]]++
	}
	count := []int{}
	for _, v := range buf {
		count = append(count, v)
	}
	sort.Ints(count)
	N := len(questions) / 2
	ret := 0
	for i := len(count) - 1; i >= 0; i-- {
		N -= count[i]
		ret++
		if N <= 0 {
			break
		}
	}
	return ret
}

func main() {
	fmt.Println()
}
