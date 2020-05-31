package main

import (
	"fmt"
	"sort"
	"strings"
)

type StrSlice []string

func (p StrSlice) Len() int {
	return len(p)
}

func (p StrSlice) Swap(i, j int) {
	p[i], p[j] = p[j], p[i]
}

func (p StrSlice) Less(i, j int) bool {
	return len(p[i]) < len(p[j])
}

func longestWord(words []string) string {
	sort.Sort(StrSlice(words))
	m := map[string]int{}
	ret := ""
	for i := 0; i < len(words); i++ {
		isPrefix := false
		for k := range m {
			if strings.HasPrefix(words[i], k) && len(words[i]) == len(k)+1 {
				if len(words[i]) > len(ret) {
					ret = words[i]
				} else if len(words[i]) == len(ret) && words[i] < ret {
					ret = words[i]
				}
				isPrefix = true
				m[words[i]]++
				break
			}
		}
		if !isPrefix && len(words[i]) == 1 {
			m[words[i]]++
			if len(words[i]) > len(ret) {
				ret = words[i]
			} else if len(words[i]) == len(ret) && words[i] < ret {
				ret = words[i]
			}
		}
	}
	return ret
}

func main() {
	fmt.Println("a")
}
