package main

import (
	"fmt"
	"sort"
	"strings"
)

type strSlice []string

func (p strSlice) Len() int {
	return len(p)
}

func (p strSlice) Swap(i, j int) {
	p[i], p[j] = p[j], p[i]
}

func (p strSlice) Less(i, j int) bool {
	return len(p[i]) < len(p[j])
}

func stringMatching(words []string) []string {

	var ret []string

	sort.Sort(strSlice(words))

	for i := len(words) - 2; i >= 0; i-- {
		for j := len(words) - 1; j > i; j-- {
			if strings.Index(words[j], words[i]) != -1 {
				ret = append(ret, words[i])
				break
			}
		}
	}
	return ret
}

func main() {
	fmt.Println("a")
}
