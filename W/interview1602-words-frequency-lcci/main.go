package main

import "fmt"

type WordsFrequency struct {
	DICT map[string]int
}

func Constructor(book []string) WordsFrequency {
	d := map[string]int{}
	for i := 0; i < len(book); i++ {
		d[book[i]]++
	}
	return WordsFrequency{DICT: d}
}

func (this *WordsFrequency) Get(word string) int {
	return this.DICT[word]
}

func main() {
	fmt.Println("a")
}
