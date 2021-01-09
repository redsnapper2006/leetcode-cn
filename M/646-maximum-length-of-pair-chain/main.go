package main

import (
	"fmt"
	"sort"
)

type CordArrSlice [][]int

func (p CordArrSlice) Len() int {
	return len(p)
}

func (p CordArrSlice) Swap(i, j int) {
	p[i], p[j] = p[j], p[i]
}

func (p CordArrSlice) Less(i, j int) bool {
	if p[i][0] == p[j][0] {
		return p[i][1] < p[j][1]
	}

	return p[i][0] < p[j][0]
}

func findLongestChain(pairs [][]int) int {
	sort.Sort(CordArrSlice(pairs))
	buf := make([]int, len(pairs))
	buf[0] = 1
	for i := 1; i < len(pairs); i++ {
		cnt := -1
		for j := i - 1; j >= 0; j-- {
			if pairs[j][1] < pairs[i][0] && buf[j] > cnt {
				cnt = buf[j]
			}
		}
		// fmt.Println(i, cnt)
		if cnt == -1 {
			buf[i] = 1
		} else {
			buf[i] = cnt + 1
		}
	}
	// fmt.Println(buf)
	max := -1
	for _, b := range buf {
		if b > max {
			max = b
		}
	}
	return max
}

func main() {
	fmt.Println()
}
