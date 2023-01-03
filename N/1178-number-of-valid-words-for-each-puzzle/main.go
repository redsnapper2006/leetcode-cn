package main

import "fmt"

func findNumOfValidWords(words []string, puzzles []string) []int {
	wordsIndex := make([][]int, 26)
	for i := 0; i < 26; i++ {
		wordsIndex[i] = []int{}
	}
	wordsInt := make([]int, len(words))
	for i, w := range words {
		sum := 0
		buf := make([]int, 26)
		for _, b := range w {
			if buf[int(b-'a')] == 0 {
				wordsIndex[int(b-'a')] = append(wordsIndex[int(b-'a')], i)
				sum += 1 << int(b-'a')
				buf[int(b-'a')] = 1
			}
		}
		wordsInt[i] = sum
	}
	puzzlesInt := make([]int, len(puzzles))
	for i, p := range puzzles {
		sum := 0
		for _, b := range p {
			sum += 1 << int(b-'a')
		}
		puzzlesInt[i] = sum
	}

	ret := make([]int, len(puzzles))
	for i, p := range puzzles {
		candi := wordsIndex[int(p[0]-'a')]
		for _, c := range candi {
			if wordsInt[c]&puzzlesInt[i] == wordsInt[c] {
				ret[i]++
			}
		}
	}
	return ret
}

func main() {
	fmt.Println("helloworld")
}
