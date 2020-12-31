package main

import (
	"fmt"
	"sort"
	"strconv"
	"strings"
)

func alertNames(keyName []string, keyTime []string) []string {
	M := map[string][]int{}
	for i, name := range keyName {
		_, ok := M[name]
		if !ok {
			M[name] = []int{}
		}
		arr := strings.Split(keyTime[i], ":")
		h, _ := strconv.ParseInt(arr[0], 10, 32)
		hour := int(h)
		m, _ := strconv.ParseInt(arr[1], 10, 32)
		minute := int(m)
		M[name] = append(M[name], hour*60+minute)
	}
	var buf []string
	for k, v := range M {
		isTarget := false
		sort.Ints(v)
		for i := 2; i < len(v); i++ {
			if v[i] > v[i-2] && v[i]-v[i-2] <= 60 {
				isTarget = true
				break
			}
		}
		if isTarget {
			buf = append(buf, k)
		}
	}
	sort.Strings(buf)
	return buf
}

func main() {
	fmt.Println()
}
