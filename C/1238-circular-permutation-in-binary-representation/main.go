package main

import "fmt"

func circularPermutation(n int, start int) []int {
	buf := []int{0, 1}

	power := 0
	for len(buf) < 1<<n {
		power++
		idx := len(buf) - 1
		for i := idx; i >= 0; i-- {
			buf = append(buf, buf[i]+1<<power)
		}
	}
	idx := -1
	for i := 0; i < len(buf); i++ {
		if buf[i] == start {
			idx = i
			break
		}
	}
	var ret []int
	for i := 0; i < len(buf); i++ {
		ret = append(ret, buf[(idx+i)%len(buf)])
	}
	return ret
}

func main() {
	fmt.Println()
}
