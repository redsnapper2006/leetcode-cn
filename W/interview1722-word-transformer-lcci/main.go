package main

import (
	"fmt"
	"sort"
)

func findLadders(beginWord string, endWord string, wordList []string) []string {
	isSimilar := func(s, t string) bool {
		if len(s) != len(t) {
			return false
		}
		c := 0
		for i := 0; i < len(s); i++ {
			if s[i] != t[i] {
				c++
			}
		}
		return c == 1
	}

	MAX := len(wordList) + 2

	M := make(map[string][]string)
	E := make(map[string]int)
	wordList = append(wordList, beginWord)
	sort.Strings(wordList)
	var candi []string
	for i := 0; i < len(wordList); i++ {
		if i > 0 && wordList[i] == wordList[i-1] {
			continue
		}
		candi = append(candi, wordList[i])
	}

	for i := 0; i < len(candi); i++ {
		for j := i + 1; j < len(candi); j++ {
			if isSimilar(candi[i], candi[j]) {
				v, ok := M[candi[i]]
				if !ok {
					M[candi[i]] = []string{candi[j]}
				} else {
					M[candi[i]] = append(v, candi[j])
				}

				v, ok = M[candi[j]]
				if !ok {
					M[candi[j]] = []string{candi[i]}
				} else {
					M[candi[j]] = append(v, candi[i])
				}
			}
		}
		E[candi[i]] = MAX
	}

	_, ok := M[endWord]
	if !ok {
		return []string{}
	}

	buf := [][]string{{beginWord}}
	step := 1
	for {
		step++
		var bb [][]string
		for i := 0; i < len(buf); i++ {
			p := buf[i]
			t := M[p[len(p)-1]]
			for j := 0; j < len(t); j++ {
				if E[t[j]] == MAX {
					var copy []string
					for m := 0; m < len(p); m++ {
						copy = append(copy, p[m])
					}
					copy = append(copy, t[j])
					bb = append(bb, copy)
				}
			}
		}
		if len(bb) == 0 {
			break
		}
		for i := 0; i < len(bb); i++ {
			p := bb[i]
			E[p[len(p)-1]] = step
		}

		if E[endWord] < MAX {
			for i := 0; i < len(bb); i++ {
				c := bb[i]
				if c[len(c)-1] == endWord {
					return c
				}
			}
		}
		buf = bb
	}

	if E[endWord] < MAX {
		for i := 0; i < len(buf); i++ {
			c := buf[i]
			if c[len(c)-1] == endWord {
				return c
			}
		}
	}

	return []string{}
}

func main() {
	fmt.Println("a")
}
