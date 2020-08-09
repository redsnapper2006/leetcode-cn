package main

import (
	"fmt"
	"sort"
)

func findContentChildren(g []int, s []int) int {
	sort.Ints(g)
	sort.Ints(s)
	c := 0
	ssIdx := 0
	sgIdx := 0
	for sgIdx < len(g) && ssIdx < len(s) {
		if s[ssIdx] >= g[sgIdx] {
			ssIdx++
			sgIdx++
			c++
		} else {
			ssIdx++
		}
	}
	return c
}

func main() {
	fmt.Println("a")
}
