package main

import (
	"fmt"
	"sort"
	"strings"
)

func arrangeWords(text string) string {
	arr := strings.Split(text, " ")
	M := map[int][]string{}
	for i := 0; i < len(arr); i++ {
		_, ok := M[len(arr[i])]
		if !ok {
			M[len(arr[i])] = []string{}
		}
		M[len(arr[i])] = append(M[len(arr[i])], arr[i])
	}

	var s []int
	for k := range M {
		s = append(s, k)
	}
	sort.Ints(s)

	isFirst := true
	var ret []string
	for i := 0; i < len(s); i++ {
		v := M[s[i]]
		for j := 0; j < len(v); j++ {
			if isFirst {
				isFirst = false
				ret = append(ret, strings.Title(v[j]))
			} else {
				ret = append(ret, strings.ToLower(v[j]))
			}
		}
	}
	return strings.Join(ret, " ")
}

func main() {
	fmt.Println("a")
}
