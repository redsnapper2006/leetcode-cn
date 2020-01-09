package main

import (
	"fmt"
	"strings"
)

func isAbleWordBreak(s string, wordDict []string) bool {
	mark := make([]int, len(s)+1)
	mark[0] = 1
	for i := 0; i < len(s); i++ {
		if mark[i] == 0 {
			continue
		}
		for _, v := range wordDict {
			if i+len(v) <= len(s) && s[i:i+len(v)] == v {
				mark[i+len(v)] = 1
			}
		}
	}

	if mark[len(s)] == 1 {
		return true
	} else {
		return false
	}
}

func wordBreak(s string, wordDict []string) []string {
	if !isAbleWordBreak(s, wordDict) {
		return []string{}
	}

	size := len(s)
	offset := make(map[int][]string)
	offset[size] = []string{""}
	for i := size; i >= 0; i-- {
		candi, ok := offset[i]
		if ok {
			for _, w := range wordDict {
				if i-len(w) >= 0 && s[i-len(w):i] == w {
					wc, ok := offset[i-len(w)]
					var orgSize, size int
					if !ok {
						orgSize = 0
						size = len(candi)
					} else {
						orgSize = len(wc)
						size = len(wc) + len(candi)
					}
					t := make([]string, size)
					if ok {
						copy(t, wc)
					}
					for j := 0; j < len(candi); j++ {
						t[orgSize+j] = w + " " + candi[j]
					}
					offset[i-len(w)] = t
				}
			}
		}
	}
	if offset[0] != nil {
		t := make([]string, len(offset[0]))
		for i, v := range offset[0] {
			t[i] = strings.TrimSpace(v)
		}
		return t
	} else {
		return []string{}
	}
}

func main() {

	// fmt.Println("helloworld"[1:3])
	// fmt.Println(wordBreak("leetcode", []string{"leet", "code"}))
	// fmt.Println(wordBreak("applepenapple", []string{"apple", "pen"}))
	// fmt.Println(wordBreak("pineapplepenapple", []string{"apple", "pen", "applepen", "pine", "pineapple"}))
	// fmt.Println(wordBreak("catsandog", []string{"cats", "dog", "sand", "and", "cat"}))
	// fmt.Println(wordBreak("catsanddog", []string{"cat", "cats", "and", "sand", "dog"}))
	// fmt.Println(wordBreak("aaaaaaa", []string{"aaaa", "aa", "a"}))
	fmt.Println(wordBreak("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", []string{"a", "aa", "aaa", "aaaa", "aaaaa", "aaaaaa", "aaaaaaa", "aaaaaaaa", "aaaaaaaaa", "aaaaaaaaaa"}))

	fmt.Println(wordBreak("bccdbacdbdacddabbaaaadababadad", []string{"cbc", "bcda", "adb", "ddca", "bad", "bbb", "dad", "dac", "ba", "aa", "bd", "abab", "bb", "dbda", "cb", "caccc", "d", "dd", "aadb", "cc", "b", "bcc", "bcd", "cd", "cbca", "bbd", "ddd", "dabb", "ab", "acd", "a", "bbcc", "cdcbd", "cada", "dbca", "ac", "abacd", "cba", "cdb", "dbac", "aada", "cdcda", "cdc", "dbc", "dbcb", "bdb", "ddbdd", "cadaa", "ddbc", "babb"}))
	// fmt.Println(wordBreak("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab", []string{"a", "aa", "aaa", "aaaa", "aaaaa", "aaaaaa", "aaaaaaa", "aaaaaaaa", "aaaaaaaaa", "aaaaaaaaaa"}))
	// fmt.Println(wordBreak("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", []string{"a", "aa", "aaa", "aaaa", "aaaaa", "aaaaaa", "aaaaaaa", "aaaaaaaa", "aaaaaaaaa", "aaaaaaaaaa"}))
	// fmt.Println(wordBreak("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabaabaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", []string{"aa", "aaa", "aaaa", "aaaaa", "aaaaaa", "aaaaaaa", "aaaaaaaa", "aaaaaaaaa", "aaaaaaaaaa", "ba"}))

}
