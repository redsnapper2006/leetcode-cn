package main

import (
	"fmt"
	"sort"
)

func nextGreaterElement(n int) int {
	buf := []int{}
	m := n
	for m > 0 {
		buf = append(buf, m%10)
		m /= 10
	}

	idx := 1
	for idx < len(buf) && buf[idx] >= buf[idx-1] {
		idx++
	}
	if idx == len(buf) {
		return -1
	}

	idx2 := -1
	min := 10
	for i := 0; i < idx; i++ {
		if buf[i] > buf[idx] && buf[i] < min {
			min = buf[i]
			idx2 = i
		}
	}

	t := buf[idx]
	buf[idx] = buf[idx2]
	buf[idx2] = t
	sort.Sort(sort.Reverse(sort.IntSlice(buf[0:idx])))

	ret := 0
	for i := len(buf) - 1; i >= 0; i-- {
		ret = ret*10 + buf[i]
	}
	if ret > 1<<31-1 {
		return -1
	}
	return ret
}

func main() {
	fmt.Println(nextGreaterElement(123))
}
