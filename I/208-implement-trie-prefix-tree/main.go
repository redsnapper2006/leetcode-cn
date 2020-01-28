package main

import (
	"fmt"
)

type Trie struct {
	M     map[int]*Trie
	isEnd bool
}

/** Initialize your data structure here. */
func Constructor() Trie {
	return Trie{M: make(map[int]*Trie), isEnd: false}
}

/** Inserts a word into the trie. */
func (this *Trie) Insert(word string) {
	p := this
	for _, c := range word {
		o := int(c - 'a')
		if p.M[o] == nil {
			t := Constructor()
			p.M[o] = &t
		}
		p = p.M[o]
	}
	p.isEnd = true
}

/** Returns if the word is in the trie. */
func (this *Trie) Search(word string) bool {
	p := this
	for _, c := range word {
		o := int(c - 'a')
		if p.M[o] != nil {
			p = p.M[o]
		} else {
			return false
		}
	}
	return p.isEnd
}

/** Returns if there is any word in the trie that starts with the given prefix. */
func (this *Trie) StartsWith(prefix string) bool {
	p := this
	for _, c := range prefix {
		o := int(c - 'a')
		if p.M[o] != nil {
			p = p.M[o]
		} else {
			return false
		}
	}
	return true
}

/**
 * Your Trie object will be instantiated and called as such:
 * obj := Constructor();
 * obj.Insert(word);
 * param_2 := obj.Search(word);
 * param_3 := obj.StartsWith(prefix);
 */
func main() {
	fmt.Println("h")
	trie := Constructor()

	trie.Insert("apple")
	fmt.Println(trie.Search("apple"))   // return true
	fmt.Println(trie.Search("app"))     // return false
	fmt.Println(trie.StartsWith("app")) // return true
	trie.Insert("app")
	fmt.Println(trie.Search("app")) // return true
}
