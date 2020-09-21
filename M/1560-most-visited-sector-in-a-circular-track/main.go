package main

import (
	"fmt"
	"sort"
)

func mostVisited(n int, rounds []int) []int {
	s, e := rounds[0], rounds[len(rounds)-1]
	var ret []int
	if s <= e {
		for i := s; i <= e; i++ {
			ret = append(ret, i)
		}
	} else {
		for i := s; i <= n; i++ {
			ret = append(ret, i)
		}
		for i := 1; i <= e; i++ {
			ret = append(ret, i)
		}
	}
	sort.Ints(ret)
	return ret
}

func mostVisitedV2(n int, rounds []int) []int {
	buf := make([]int, n)

	start := rounds[0] - 1
	buf[start]++
	for i := 1; i < len(rounds); i++ {
		for start != (rounds[i]-1)%n {
			start++
			start %= n
			buf[start]++
		}
	}
	max := -1
	var ret []int
	for i := 0; i < len(buf); i++ {
		if max < buf[i] {
			max = buf[i]
			ret = []int{}
			ret = append(ret, i+1)
		} else if max == buf[i] {
			ret = append(ret, i+1)
		}
	}
	return ret

}

func main() {
	fmt.Println("a")
}
