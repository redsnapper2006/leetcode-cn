package main

import (
	"fmt"
	"strconv"
	"strings"
)

func trulyMostPopular(names []string, synonyms []string) []string {
	vm := map[string]int{}
	for _, v := range names {
		idx := -1
		for i, b := range v {
			if byte(b) == byte('(') {
				idx = i
				break
			}
		}
		cnt, _ := strconv.ParseInt(v[idx+1:len(v)-1], 10, 32)
		vm[v[0:idx]] = int(cnt)
	}

	sm := map[string][]string{}
	for _, v := range synonyms {
		s := v[1 : len(v)-1]
		a := strings.Split(s, ",")
		sm[a[0]] = append(sm[a[0]], a[1])
		sm[a[1]] = append(sm[a[1]], a[0])
	}

	ret := []string{}

	occu := map[string]int{}
	for k := range vm {
		_, ok := occu[k]
		if ok {
			continue
		}
		sum := 0
		key := ""
		candi := []string{k}
		for len(candi) > 0 {
			t := []string{}
			for _, c := range candi {
				_, ok := occu[c]
				if ok {
					continue
				}
				sum += vm[c]
				occu[c] = 1
				if key == "" || key > c {
					key = c
				}
				for _, nc := range sm[c] {
					_, ok := occu[nc]
					if ok {
						continue
					}
					t = append(t, nc)
				}
			}
			candi = t
		}
		ret = append(ret, fmt.Sprintf("%s(%d)", key, sum))
	}
	return ret
}

func main() {
	fmt.Println()
}
