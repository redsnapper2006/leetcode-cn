package main

import "fmt"

func maxProduct(words []string) int {
	x := make([]int, len(words))
	for i, w := range words {
		s := 0
		for _, b := range w {
			o := int(b - 'a')
			s |= 1 << o
		}
		x[i] = s
	}
	max := 0
	for i := 0; i < len(words); i++ {
		for j := i + 1; j < len(words); j++ {
			if x[i]&x[j] == 0 && len(words[i])*len(words[j]) > max {
				max = len(words[i]) * len(words[j])
			}
		}
	}
	return max
}

func main() {
	fmt.Println()
}
