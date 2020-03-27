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
	return len(p[i]) > len(p[j])
}

type Trie struct {
	Val  int
	Next []Trie
}

func minimumLengthEncodingV2(words []string) int {
	if len(words) == 1 {
		return len(words[0]) + 1
	}
	sort.Sort(StrSlice(words))

	head := make([]Trie, 26)

	c := 0
	for i := 0; i < len(words); i++ {
		w := words[i]
		p := head
		isPlus := false
		for j := len(w) - 1; j >= 0; j-- {
			if p[w[j]-'a'].Val == 0 {
				p[w[j]-'a'].Val = 1
				p[w[j]-'a'].Next = make([]Trie, 26)
				isPlus = true
			}
			p = p[w[j]-'a'].Next
		}
		if isPlus {
			c += len(w) + 1
		}
	}

	return c
}

func minimumLengthEncoding(words []string) int {
	if len(words) == 1 {
		return len(words[0]) + 1
	}

	var buf []string
	for i := 0; i < len(words); i++ {
		w := words[i]
		b := make([]byte, len(w))
		for j := 0; j < len(w); j++ {
			b[j] = w[len(w)-1-j]
		}
		buf = append(buf, string(b))
	}
	sort.Sort(sort.StringSlice(buf))

	c := 0
	for i := 0; i < len(buf); i++ {
		if i < len(buf)-1 && strings.HasPrefix(buf[i+1], buf[i]) {
			continue
		} else {
			c += len(buf[i]) + 1
		}
	}

	return c
}

func main() {
	fmt.Println(minimumLengthEncoding([]string{"time", "me", "bell"}))
}
