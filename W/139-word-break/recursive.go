package main

import (
	"fmt"
	"strings"
)

func wordBreak2(s string, wordDict []string) bool {
	// fmt.Println(s)
	if len(s) == 0 {
		return true
	}

	for _, v := range wordDict {
		// fmt.Println("go :", v)
		if strings.HasPrefix(s, v) {
			ret := wordBreak(s[len(v):], wordDict)
			if ret {
				return true
			}
		}
	}
	return false
}

func main2() {

	// fmt.Println("helloworld"[1:3])
	// fmt.Println(wordBreak("leetcode", []string{"leet", "code"}))
	// fmt.Println(wordBreak("applepenapple", []string{"apple", "pen"}))
	// fmt.Println(wordBreak("catsandog", []string{"cats", "dog", "sand", "and", "cat"}))
	// fmt.Println(wordBreak("aaaaaaa", []string{"aaaa", "aaa"}))
	// fmt.Println(wordBreak("bccdbacdbdacddabbaaaadababadad", []string{"cbc", "bcda", "adb", "ddca", "bad", "bbb", "dad", "dac", "ba", "aa", "bd", "abab", "bb", "dbda", "cb", "caccc", "d", "dd", "aadb", "cc", "b", "bcc", "bcd", "cd", "cbca", "bbd", "ddd", "dabb", "ab", "acd", "a", "bbcc", "cdcbd", "cada", "dbca", "ac", "abacd", "cba", "cdb", "dbac", "aada", "cdcda", "cdc", "dbc", "dbcb", "bdb", "ddbdd", "cadaa", "ddbc", "babb"}))
	// fmt.Println(wordBreak("cars", []string{"car", "ca", "rs"}))

	// fmt.Println(wordBreak("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab", []string{"a", "aa", "aaa", "aaaa", "aaaaa", "aaaaaa", "aaaaaaa", "aaaaaaaa", "aaaaaaaaa", "aaaaaaaaaa"}))
	// fmt.Println(wordBreak("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", []string{"a", "aa", "aaa", "aaaa", "aaaaa", "aaaaaa", "aaaaaaa", "aaaaaaaa", "aaaaaaaaa", "aaaaaaaaaa"}))
	fmt.Println(wordBreak("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabaabaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", []string{"aa", "aaa", "aaaa", "aaaaa", "aaaaaa", "aaaaaaa", "aaaaaaaa", "aaaaaaaaa", "aaaaaaaaaa", "ba"}))

}
