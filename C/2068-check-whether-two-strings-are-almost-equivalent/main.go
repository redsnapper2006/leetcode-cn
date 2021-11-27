package main

import "fmt"

func checkAlmostEquivalent(word1 string, word2 string) bool {
	b1, b2 := make([]int, 26), make([]int, 26)
	for _, b := range word1 {
		b1[int(byte(b)-byte('a'))]++
	}
	for _, b := range word2 {
		b2[int(byte(b)-byte('a'))]++
	}
	for i := 0; i < 26; i++ {
		d := b1[i] - b2[i]
		if d < 0 {
			d = -d
		}
		if d > 3 {
			return false
		}
	}
	return true

}

func main() {
	fmt.Println()
}
