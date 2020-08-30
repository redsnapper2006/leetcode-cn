package main

import (
	"fmt"
	"sort"
)

func maximumSwap(num int) int {
	if num < 10 {
		return num
	}

	var buf []int
	for num > 0 {
		m := num % 10
		buf = append(buf, m)
		num /= 10
	}

	ss := make([]int, len(buf))
	copy(ss, buf)
	sort.Ints(ss)

	idx := -1
	for i := len(buf) - 1; i >= 0; i-- {
		if ss[i] != buf[i] {
			idx = i
			break
		}
	}
	if idx != -1 {
		idx2 := -1
		for i := 0; i < len(buf); i++ {
			if buf[i] == ss[idx] {
				idx2 = i
				break
			}
		}
		buf[idx], buf[idx2] = buf[idx2], buf[idx]
	}

	n := 0
	for i := len(buf) - 1; i >= 0; i-- {
		n = n*10 + buf[i]
	}
	return n
}

func main() {
	fmt.Println("a")
}
